use crate::model::{
    claims::ChClaims,
    constants::{DEFAULT_NUM_RESPONSE_ENTRIES, DEFAULT_PROCESS_ID, MAX_NUM_RESPONSE_ENTRIES},
    {document::Document, process::Process, SortingOrder},
};
use std::sync::Arc;

use crate::db::{DocumentStore, ProcessStore};
use crate::model::{
    ids::{message::IdsMessage, request::ClearingHouseMessage, IdsQueryResult},
    process::{DataTransaction, OwnerList, Receipt},
};
use crate::services::document_service::DocumentService;

/// Error type for `LoggingService`
#[derive(Debug, thiserror::Error)]
pub enum LoggingServiceError {
    #[error("Received empty payload, which cannot be logged!")]
    EmptyPayloadReceived,
    #[error("Accessing default PID is not allowed!")]
    AttemptedAccessToDefaultPid,
    #[error("Error during database operation: {description}: {source}")]
    DatabaseError {
        source: anyhow::Error,
        description: String,
    },
    #[error("User not authorized!")]
    UserNotAuthorized,
    #[error("Invalid request received!")]
    InvalidRequest,
    #[error("Process already exists!")]
    ProcessAlreadyExists,
    #[error("Process '{0}' does not exist!")]
    ProcessDoesNotExist(String),
    #[error("Parsing error in {0}")]
    ParsingError(#[from] serde_json::Error),
    #[error("DocumentService error in {0}")]
    DocumentServiceError(#[from] crate::services::document_service::DocumentServiceError),
}

impl axum::response::IntoResponse for LoggingServiceError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;
        match self {
            Self::EmptyPayloadReceived
            | Self::AttemptedAccessToDefaultPid
            | Self::InvalidRequest
            | Self::ProcessAlreadyExists
            | Self::ParsingError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::DatabaseError {
                source,
                description,
            } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{description}: {source}"),
            )
                .into_response(),
            Self::UserNotAuthorized => (StatusCode::FORBIDDEN, self.to_string()).into_response(),
            Self::ProcessDoesNotExist(_) => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            Self::DocumentServiceError(e) => e.into_response(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LoggingService<T, S> {
    db: T,
    static_process_owner: Option<String>,
    doc_api: Arc<DocumentService<S>>,
}

impl<T: ProcessStore, S: DocumentStore> LoggingService<T, S> {
    pub fn new(
        db: T,
        doc_api: Arc<DocumentService<S>>,
        static_process_owner: Option<String>,
    ) -> LoggingService<T, S> {
        LoggingService {
            db,
            static_process_owner,
            doc_api,
        }
    }

    pub async fn log(
        &self,
        ch_claims: ChClaims,
        key_path: &str,
        msg: ClearingHouseMessage,
        pid: String,
    ) -> Result<Receipt, LoggingServiceError> {
        trace!("...user '{}'", &ch_claims.client_id);
        let user = &ch_claims.client_id;
        // Add non-InfoModel information to IdsMessage
        let mut m = msg.header;
        m.payload = msg.payload;
        m.payload_type = msg.payload_type;
        m.pid = Some(pid.clone());

        // Check for default process id
        Self::check_for_default_pid(&pid)?;

        // validate that there is a payload
        let payload = match m.payload.clone() {
            Some(p) if !p.trim().is_empty() => Ok(p),
            _ => {
                error!("Trying to log an empty payload!");
                Err(LoggingServiceError::EmptyPayloadReceived) // BadRequest
            }
        }?;

        // Check if process exists and if the user is authorized to access the process
        match self.get_process_and_check_authorized(&pid, user).await {
            Err(LoggingServiceError::ProcessDoesNotExist(_)) => {
                // convenience: if process does not exist, we create it but only if no error occurred before
                info!("Requested pid '{}' does not exist. Creating...", &pid);
                // create a new process
                let new_process = Process::new(pid.clone(), vec![user.clone()]);

                if let Err(e) = self.db.store_process(new_process).await {
                    error!("Error while creating process '{}' automatically for log message (could have been created in the meantime)", &pid);

                    match self.get_process_and_check_authorized(&pid, user).await {
                        Ok(_) => {}
                        Err(LoggingServiceError::ProcessDoesNotExist(_)) => {
                            error!(
                                "Process still not exists (failing with Database error now): {e:?}",
                            );
                            return Err(LoggingServiceError::DatabaseError {
                                source: e,
                                description: "Creating process failed".to_string(),
                            }); // InternalError
                        }
                        Err(e) => {
                            warn!("Error while checking process: {:?}", e);
                            return Err(e);
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Error while checking process: {:?}", e);
                return Err(e);
            }
            Ok(_) => {}
        }

        // transform message to document
        debug!("transforming message to document...");
        let doc: Document = m.into();

        debug!("Storing document...");
        match self
            .doc_api
            .create_enc_document(ChClaims::new(user), doc.clone())
            .await
        {
            Ok(doc_receipt) => {
                debug!("Creating receipt...");
                let transaction = DataTransaction {
                    timestamp: doc_receipt.timestamp,
                    process_id: doc_receipt.pid,
                    document_id: doc_receipt.doc_id,
                    payload,
                    client_id: user.to_owned(),
                    clearing_house_version: env!("CARGO_PKG_VERSION").to_string(),
                };
                debug!("...done. Signing receipt...");
                Ok(transaction.sign(key_path))
            }
            Err(e) => {
                error!("Error while creating document: {:?}", e);
                Err(LoggingServiceError::DocumentServiceError(e))
            }
        }
    }

    pub(crate) async fn create_process(
        &self,
        ch_claims: ChClaims,
        msg: ClearingHouseMessage,
        pid: String,
    ) -> Result<String, LoggingServiceError> {
        let mut m = msg.header;
        m.payload = msg.payload;
        m.payload_type = msg.payload_type;

        trace!("...user '{:?}'", &ch_claims.client_id);
        let user = &ch_claims.client_id;

        // Check for default process id
        Self::check_for_default_pid(&pid)?;

        // validate payload
        let mut owners = vec![user.clone()];
        if let Some(static_process_owner) = &self.static_process_owner {
            owners.push(static_process_owner.clone());
        }
        match m.payload {
            Some(ref payload) if !payload.is_empty() => {
                trace!("OwnerList: '{:#?}'", &payload);
                match serde_json::from_str::<OwnerList>(payload) {
                    Ok(owner_list) => {
                        for o in owner_list.owners {
                            if !owners.contains(&o) {
                                owners.push(o);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Could not parse OwnerList '{payload}' for pid '{pid}': {e}");
                        return Err(LoggingServiceError::InvalidRequest); // BadRequest
                    }
                };
            }
            _ => {}
        };

        // check if the pid already exists
        match self.db.get_process(&pid).await {
            Ok(Some(p)) => {
                warn!("Requested pid '{}' already exists.", &p.id);
                if p.owners.contains(user) {
                    Err(LoggingServiceError::ProcessAlreadyExists) // BadRequest
                } else {
                    Err(LoggingServiceError::UserNotAuthorized) // Forbidden
                }
            }
            Ok(None) => {
                info!(
                    "Requested pid '{}' does not exist and will have {} owners. Creating...",
                    &pid,
                    owners.len()
                );

                // create process
                let new_process = Process::new(pid.clone(), owners);

                match self.db.store_process(new_process).await {
                    Ok(()) => Ok(pid.clone()),
                    Err(e) => {
                        error!("Error while creating process '{}': {}", &pid, e);
                        Err(LoggingServiceError::DatabaseError {
                            source: e,
                            description: "Creating process failed".to_string(),
                        }) // InternalError
                    }
                }
            }
            Err(e) => Err(LoggingServiceError::DatabaseError {
                source: e,
                description: "Error while getting process".to_string(),
            }),
        }
    }

    pub(crate) async fn query_pid(
        &self,
        ch_claims: ChClaims,
        page: Option<u64>,
        size: Option<u64>,
        sort: Option<SortingOrder>,
        (date_to, date_from): (Option<String>, Option<String>),
        pid: String,
    ) -> Result<IdsQueryResult, LoggingServiceError> {
        debug!("page: {:#?}, size:{:#?} and sort:{:#?}", page, size, sort);

        trace!("...user '{}'", &ch_claims.client_id);
        let user = &ch_claims.client_id;

        // Check if process exists and if the user is authorized to access the process
        self.get_process_and_check_authorized(&pid, user).await?;

        let sanitized_page = page.unwrap_or(1);
        let sanitized_size = match size {
            Some(s) => s.min(MAX_NUM_RESPONSE_ENTRIES),
            None => DEFAULT_NUM_RESPONSE_ENTRIES,
        };

        let sanitized_sort = sort.unwrap_or(SortingOrder::Descending);

        match self
            .doc_api
            .get_enc_documents_for_pid(
                ChClaims::new(user),
                Some(sanitized_page),
                Some(sanitized_size),
                Some(sanitized_sort),
                (date_from, date_to),
                pid.clone(),
            )
            .await
        {
            Ok(r) => {
                let messages: Vec<IdsMessage> = r
                    .documents
                    .iter()
                    .map(|d| IdsMessage::from(d.clone()))
                    .collect();
                let result =
                    IdsQueryResult::new(r.date_from, r.date_to, r.page, r.size, r.order, messages);
                Ok(result)
            }
            Err(e) => {
                error!("Error while retrieving message: {:?}", e);
                Err(LoggingServiceError::DocumentServiceError(e))
            }
        }
    }

    /// Query a single message by its `id` and `pid`
    ///
    /// `_message` is required because the `ClearingHouseMessage` as request body is required by the route
    #[allow(clippy::no_effect_underscore_binding)]
    pub(crate) async fn query_id(
        &self,
        ch_claims: ChClaims,
        pid: String,
        id: String,
        _message: ClearingHouseMessage,
    ) -> Result<IdsMessage, LoggingServiceError> {
        trace!("...user '{}'", &ch_claims.client_id);
        let user = &ch_claims.client_id;

        // Check if process exists and if the user is authorized to access the process
        self.get_process_and_check_authorized(&pid, user).await?;

        match self
            .doc_api
            .get_enc_document(ChClaims::new(user), pid.clone(), id.clone(), None)
            .await
        {
            Ok(doc) => {
                // transform document to IDS message
                let queried_message = IdsMessage::from(doc);
                Ok(queried_message)
            }
            Err(e) => {
                error!("Error while retrieving message: {:?}", e);
                Err(LoggingServiceError::DocumentServiceError(e))
            }
        }
    }

    /// Checks if the given pid is the default pid
    fn check_for_default_pid(pid: &str) -> Result<(), LoggingServiceError> {
        // Check for default process id
        if DEFAULT_PROCESS_ID.eq(pid) {
            warn!("Log to default pid '{}' not allowed", DEFAULT_PROCESS_ID);
            Err(LoggingServiceError::AttemptedAccessToDefaultPid)
        } else {
            Ok(())
        }
    }

    /// Checks if a process exists and the user is authorized to access the process
    async fn get_process_and_check_authorized(
        &self,
        pid: &String,
        user: &str,
    ) -> Result<Process, LoggingServiceError> {
        match self.db.get_process(pid).await {
            Ok(Some(p)) if !p.is_authorized(user) => {
                warn!("User is not authorized to read from pid '{}'", &pid);
                Err(LoggingServiceError::UserNotAuthorized)
            }
            Ok(Some(p)) => {
                info!("User authorized.");
                Ok(p)
            }
            Ok(None) => Err(LoggingServiceError::ProcessDoesNotExist(pid.clone())),
            Err(e) => {
                error!("Error while getting process '{}': {}", &pid, e);
                Err(LoggingServiceError::DatabaseError {
                    source: e,
                    description: "Getting process failed".to_string(),
                })
            }
        }
    }
}
