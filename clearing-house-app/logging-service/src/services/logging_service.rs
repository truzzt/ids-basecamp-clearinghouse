use core_lib::{
    api::crypto::ChClaims,
    constants::{DEFAULT_NUM_RESPONSE_ENTRIES, MAX_NUM_RESPONSE_ENTRIES, DEFAULT_PROCESS_ID},
    model::{
        document::Document,
        process::Process,
        SortingOrder,
    },
};
use rocket::form::validate::Contains;
use rocket::State;
use std::convert::TryFrom;
use anyhow::anyhow;

use crate::model::{ids::{
    message::IdsMessage,
    IdsQueryResult,
    request::ClearingHouseMessage,
}, OwnerList, DataTransaction, Receipt};
use crate::db::ProcessStore;
use crate::services::document_service::DocumentService;

pub struct LoggingService {
    db: ProcessStore,
    doc_api: DocumentService,
}

impl LoggingService {
    pub async fn log(
        &self,
        ch_claims: ChClaims,
        key_path: &State<String>,
        msg: ClearingHouseMessage,
        pid: String,
    ) -> anyhow::Result<Receipt> {
        trace!("...user '{:?}'", &ch_claims.client_id);
        let user = &ch_claims.client_id;
        // Add non-InfoModel information to IdsMessage
        let mut m = msg.header;
        m.payload = msg.payload;
        m.payload_type = msg.payload_type;
        m.pid = Some(pid.clone());

        // validate that there is a payload
        if m.payload.is_none() || (m.payload.is_some() && m.payload.as_ref().unwrap().trim().is_empty()) {
            error!("Trying to log an empty payload!");
            return Err(anyhow!("No payload received for logging!")); // BadRequest
        }

        // filter out calls for default process id and call application logic
        match DEFAULT_PROCESS_ID.eq(pid.as_str()) {
            true => {
                warn!("Log to default pid '{}' not allowed", DEFAULT_PROCESS_ID);
                Err(anyhow!("Document already exists")) // BadRequest
            }
            false => {
                // convenience: if process does not exist, we create it but only if no error occurred before
                match self.db.get_process(&pid).await {
                    Ok(Some(_p)) => {
                        debug!("Requested pid '{}' exists. Nothing to create.", &pid);
                    }
                    Ok(None) => {
                        info!("Requested pid '{}' does not exist. Creating...", &pid);
                        // create a new process
                        let new_process = Process::new(pid.clone(), vec!(user.clone()));

                        if self.db.store_process(new_process).await.is_err() {
                            error!("Error while creating process '{}'", &pid);
                            return Err(anyhow!("Error while creating process")); // InternalError
                        }
                    }
                    Err(_) => {
                        error!("Error while getting process '{}'", &pid);
                        return Err(anyhow!("Error while getting process")); // InternalError
                    }
                }

                // now check if user is authorized to write to pid
                match self.db.is_authorized(&user, &pid).await {
                    Ok(true) => info!("User authorized."),
                    Ok(false) => {
                        warn!("User is not authorized to write to pid '{}'", &pid);
                        warn!("This is the forbidden branch");
                        return Err(anyhow!("User not authorized!")); // Forbidden
                    }
                    Err(_) => {
                        error!("Error while checking authorization of user '{}' for '{}'", &user, &pid);
                        return Err(anyhow!("Error during authorization"));
                    }
                }

                debug!("logging message for pid {}", &pid);
                self.log_message(user, key_path.inner().as_str(), m.clone()).await
            }
        }
    }

    pub(crate) async fn create_process(
        &self,
        ch_claims: ChClaims,
        msg: ClearingHouseMessage,
        pid: String,
    ) -> anyhow::Result<String> {
        let mut m = msg.header;
        m.payload = msg.payload;
        m.payload_type = msg.payload_type;

        trace!("...user '{:?}'", &ch_claims.client_id);
        let user = &ch_claims.client_id;

        // validate payload
        let mut owners = vec!(user.clone());
        let payload = m.payload.clone().unwrap_or(String::new());
        if !payload.is_empty() {
            trace!("OwnerList: '{:#?}'", &payload);
            match serde_json::from_str::<OwnerList>(&payload) {
                Ok(owner_list) => {
                    for o in owner_list.owners {
                        if !owners.contains(&o) {
                            owners.push(o);
                        }
                    }
                }
                Err(e) => {
                    error!("Error while creating process '{}': {}", &pid, e);
                    return Err(anyhow!("Invalid owner list!")); // BadRequest
                }
            };
        };

        // check if the pid already exists
        match self.db.get_process(&pid).await {
            Ok(Some(p)) => {
                warn!("Requested pid '{}' already exists.", &p.id);
                if !p.owners.contains(user) {
                    Err(anyhow!("User not authorized!")) // Forbidden
                } else {
                    Err(anyhow!("Process already exists!")) // BadRequest
                }
            }
            _ => {
                // filter out calls for default process id
                match DEFAULT_PROCESS_ID.eq(pid.as_str()) {
                    true => {
                        warn!("Log to default pid '{}' not allowed", DEFAULT_PROCESS_ID);
                        Err(anyhow!("Document already exists")) // BadRequest
                    }
                    false => {
                        info!("Requested pid '{}' will have {} owners", &pid, owners.len());

                        // create process
                        info!("Requested pid '{}' does not exist. Creating...", &pid);
                        let new_process = Process::new(pid.clone(), owners);

                        match self.db.store_process(new_process).await {
                            Ok(_) => {
                                Ok(pid.clone())
                            }
                            Err(e) => {
                                error!("Error while creating process '{}': {}", &pid, e);
                                Err(anyhow!("Error while creating process")) // InternalError
                            }
                        }
                    }
                }
            }
        }
    }

    async fn log_message(
        &self,
        user: &String,
        key_path: &str,
        message: IdsMessage,
    ) -> anyhow::Result<Receipt> {
        debug!("transforming message to document...");
        let payload = message.payload.as_ref().unwrap().clone();
        // transform message to document
        let mut doc = Document::from(message);
        match self.db.get_transaction_counter().await {
            Ok(Some(tid)) => {
                debug!("Storing document...");
                doc.tc = tid;
                // TODO: ChClaims usage check
                match self.doc_api.create_enc_document(ChClaims::new(&user), doc.clone()).await {
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
                                    client_id: user.clone(),
                                    clearing_house_version: env!("CARGO_PKG_VERSION").to_string(),
                                };
                                debug!("...done. Signing receipt...");
                                Ok(transaction.sign(key_path))
                            }
                            _ => {
                                error!("Error while incrementing transaction id!");
                                Err(anyhow!("Internal error while preparing transaction data")) // InternalError
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error while creating document: {:?}", e);
                        Err(anyhow!("Document already exists")) // BadRequest
                    }
                }
            }
            Ok(None) => {
                println!("None!");
                Err(anyhow!("Internal error while preparing transaction data")) // InternalError
            }
            Err(e) => {
                error!("Error while getting transaction id!");
                println!("{}", e);
                Err(anyhow!("Internal error while preparing transaction data")) // InternalError
            }
        }
    }

    pub(crate) async fn query_pid(
        &self,
        ch_claims: ChClaims,
        page: Option<i32>,
        size: Option<i32>,
        sort: Option<SortingOrder>,
        date_to: Option<String>,
        date_from: Option<String>,
        pid: String,
        message: ClearingHouseMessage,
    ) -> anyhow::Result<IdsQueryResult> {
        debug!("page: {:#?}, size:{:#?} and sort:{:#?}", page, size, sort);

        trace!("...user '{:?}'", &ch_claims.client_id);
        let user = &ch_claims.client_id;

        // check if process exists
        match self.db.exists_process(&pid).await {
            Ok(true) => info!("User authorized."),
            Ok(false) => return Err(anyhow!("Process does not exist!")), // NotFound
            Err(_e) => {
                error!("Error while checking process '{}' for user '{}'", &pid, &user);
                return Err(anyhow!("Cannot authorize user!")); // InternalError
            }
        };

        // now check if user is authorized to read infos in pid
        match self.db.is_authorized(&user, &pid).await {
            Ok(true) => {
                info!("User authorized.");
            }
            Ok(false) => {
                warn!("User is not authorized to write to pid '{}'", &pid);
                return Err(anyhow!("User not authorized!")); // Forbidden
            }
            Err(_) => {
                error!("Error while checking authorization of user '{}' for '{}'", &user, &pid);
                return Err(anyhow!("Cannot authorize user!")); // InternalError
            }
        }

        // sanity check for pagination
        let sanitized_page = match page {
            Some(p) => {
                if p >= 0 {
                    p
                } else {
                    warn!("...invalid page requested. Falling back to 0.");
                    1
                }
            }
            None => 1
        };

        let sanitized_size = match size {
            Some(s) => {
                let converted_max = i32::try_from(MAX_NUM_RESPONSE_ENTRIES).unwrap();
                if s > converted_max {
                    warn!("...invalid size requested. Falling back to default.");
                    converted_max
                } else {
                    if s > 0 {
                        s
                    } else {
                        warn!("...invalid size requested. Falling back to default.");
                        i32::try_from(DEFAULT_NUM_RESPONSE_ENTRIES).unwrap()
                    }
                }
            }
            None => i32::try_from(DEFAULT_NUM_RESPONSE_ENTRIES).unwrap()
        };

        let sanitized_sort = sort.unwrap_or(SortingOrder::Descending);

        match self.doc_api.get_enc_documents_for_pid(ChClaims::new(&user), None, Some(sanitized_page), Some(sanitized_size), Some(sanitized_sort), date_from, date_to, pid.clone()).await {
            Ok(r) => {
                let messages: Vec<IdsMessage> = r.documents.iter().map(|d| IdsMessage::from(d.clone())).collect();
                let result = IdsQueryResult::new(r.date_from, r.date_to, r.page, r.size, r.order, messages);
                Ok(result)
            }
            Err(e) => {
                error!("Error while retrieving message: {:?}", e);
                Err(anyhow!("Error while retrieving messages for pid {}!", &pid)) // InternalError
            }
        }
    }

    pub(crate) async fn query_id(&self,
                                 ch_claims: ChClaims,
                                 pid: String,
                                 id: String,
                                 message: ClearingHouseMessage,
    ) -> anyhow::Result<IdsMessage> {
        trace!("...user '{:?}'", &ch_claims.client_id);
        let user = &ch_claims.client_id;

        // check if process exists
        match self.db.exists_process(&pid).await {
            Ok(true) => info!("User authorized."),
            Ok(false) => return Err(anyhow!("Process does not exist!")), // NotFound
            Err(_e) => {
                error!("Error while checking process '{}' for user '{}'", &pid, &user);
                return Err(anyhow!("Cannot authorize user!")); // InternalError
            }
        };

        // now check if user is authorized to read infos in pid
        match self.db.is_authorized(&user, &pid).await {
            Ok(true) => {
                info!("User authorized.");
            }
            Ok(false) => {
                warn!("User is not authorized to write to pid '{}'", &pid);
                return Err(anyhow!("User not authorized!")); // Forbidden
            }
            Err(_) => {
                error!("Error while checking authorization of user '{}' for '{}'", &user, &pid);
                return Err(anyhow!("Cannot authorize user!")); // InternalError
            }
        }

        match self.doc_api.get_enc_document(ChClaims::new(&user), pid.clone(), id.clone(), None).await {
            Ok(doc) => {
                // transform document to IDS message
                let queried_message = IdsMessage::from(doc);
                Ok(queried_message)
            }
            /*Result::Ok(None) => {
                debug!("Queried a non-existing document: {}", &id);
                ApiResponse::NotFound(format!("No message found with id {}!", &id))
            }*/
            Err(e) => {
                error!("Error while retrieving message: {:?}", e);
                Err(anyhow!("Error while retrieving message with id {}!", &id)) // InternalError
            }
        }
    }
}