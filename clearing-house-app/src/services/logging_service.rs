use crate::model::{
    claims::ChClaims,
    constants::{DEFAULT_NUM_RESPONSE_ENTRIES, DEFAULT_PROCESS_ID, MAX_NUM_RESPONSE_ENTRIES},
    {document::Document, process::Process, SortingOrder},
};
use std::sync::Arc;

use crate::db::process_store::ProcessStore;
use crate::model::{
    ids::{message::IdsMessage, request::ClearingHouseMessage, IdsQueryResult},
    process::{DataTransaction, OwnerList, Receipt},
};
use crate::services::document_service::DocumentService;

/// Error type for LoggingService
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
            Self::EmptyPayloadReceived => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Self::AttemptedAccessToDefaultPid => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Self::DatabaseError {
                source,
                description,
            } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{}: {}", description, source),
            )
                .into_response(),
            Self::UserNotAuthorized => (StatusCode::FORBIDDEN, self.to_string()).into_response(),
            Self::InvalidRequest => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::ProcessAlreadyExists => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Self::ProcessDoesNotExist(_) => {
                (StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            Self::ParsingError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::DocumentServiceError(e) => e.into_response(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LoggingService {
    db: ProcessStore,
    doc_api: Arc<DocumentService>,
    write_lock: Arc<tokio::sync::Mutex<()>>,
}

impl LoggingService {
    pub fn new(db: ProcessStore, doc_api: Arc<DocumentService>) -> LoggingService {
        LoggingService {
            db,
            doc_api,
            write_lock: Arc::new(tokio::sync::Mutex::new(())),
        }
    }

    pub async fn log(
        &self,
        ch_claims: ChClaims,
        key_path: &str,
        msg: ClearingHouseMessage,
        pid: String,
    ) -> Result<Receipt, LoggingServiceError> {
        trace!("...user '{:?}'", &ch_claims.client_id);
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
        if let Err(LoggingServiceError::ProcessDoesNotExist(_)) =
            self.get_process_and_check_authorized(&pid, user).await
        {
            // convenience: if process does not exist, we create it but only if no error occurred before
            info!("Requested pid '{}' does not exist. Creating...", &pid);
            // create a new process
            let new_process = Process::new(pid.clone(), vec![user.clone()]);

            if let Err(e) = self.db.store_process(new_process).await {
                error!("Error while creating process '{}'", &pid);
                return Err(LoggingServiceError::DatabaseError {
                    source: e,
                    description: "Creating process failed".to_string(),
                }); // InternalError
            }
        }

        // transform message to document
        debug!("transforming message to document...");
        let mut doc = Document::try_from(m).map_err(LoggingServiceError::ParsingError)?;

        // lock write access
        let _x = self.write_lock.lock().await;
        match self.db.get_transaction_counter().await {
            Ok(Some(tid)) => {
                debug!("Storing document...");
                doc.tc = tid;
                // TODO: ChClaims usage check
                match self
                    .doc_api
                    .create_enc_document(ChClaims::new(user), doc.clone())
                    .await
                {
                    Ok(doc_receipt) => {
                        debug!("Increase transaction counter");
                        match self.db.increment_transaction_counter().await {
                            Ok(Some(_tid)) => {
                                debug!("Creating receipt...");
                                let transaction = DataTransaction {
                                    transaction_id: doc.get_formatted_tc(),
                                    timestamp: doc_receipt.timestamp,
                                    process_id: doc_receipt.pid,
                                    document_id: doc_receipt.doc_id,
                                    payload,
                                    chain_hash: doc_receipt.chain_hash,
                                    client_id: user.to_owned(),
                                    clearing_house_version: env!("CARGO_PKG_VERSION").to_string(),
                                };
                                debug!("...done. Signing receipt...");
                                Ok(transaction.sign(key_path))
                            }
                            Ok(None) => {
                                unreachable!("increment_transaction_counter never returns None!")
                            }
                            Err(e) => {
                                error!("Error while incrementing transaction id!");
                                Err(LoggingServiceError::DatabaseError {
                                    source: e,
                                    description: "Error while incrementing transaction id!"
                                        .to_string(),
                                }) // InternalError
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error while creating document: {:?}", e);
                        Err(LoggingServiceError::DocumentServiceError(e))
                    }
                }
            }
            Ok(None) => unreachable!("get_transaction_counter never returns None!"),
            Err(e) => {
                error!("Error while getting transaction id!");
                Err(LoggingServiceError::DatabaseError {
                    source: e,
                    description: "Error while getting transaction id".to_string(),
                }) // InternalError
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
                if !p.owners.contains(user) {
                    Err(LoggingServiceError::UserNotAuthorized) // Forbidden
                } else {
                    Err(LoggingServiceError::ProcessAlreadyExists) // BadRequest
                }
            }
            Ok(None) => {
                info!("Requested pid '{}' will have {} owners", &pid, owners.len());

                // create process
                info!("Requested pid '{}' does not exist. Creating...", &pid);
                let new_process = Process::new(pid.clone(), owners);

                match self.db.store_process(new_process).await {
                    Ok(_) => Ok(pid.clone()),
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

        trace!("...user '{:?}'", &ch_claims.client_id);
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

    pub(crate) async fn query_id(
        &self,
        ch_claims: ChClaims,
        pid: String,
        id: String,
        _message: ClearingHouseMessage,
    ) -> Result<IdsMessage, LoggingServiceError> {
        trace!("...user '{:?}'", &ch_claims.client_id);
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

#[cfg(test)]
mod test {
    use super::LoggingService;
    use crate::model::constants::DEFAULT_PROCESS_ID;

    #[test]
    fn check_for_default_pid() {
        assert!(LoggingService::check_for_default_pid(DEFAULT_PROCESS_ID).is_err());
        assert!(LoggingService::check_for_default_pid("not_default").is_ok());
    }
}
