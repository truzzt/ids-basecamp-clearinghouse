use biscuit::Empty;
use rocket::State;
use core_lib::{
    api::{
        ApiResponse,
        auth::ApiKey,
        client::document_api::DocumentApiClient,
        claims::IdsClaims,
    },
    constants::DEFAULT_PROCESS_ID,
    model::{
        document::Document,
        process::Process
    }
};
use core_lib::constants::{DEFAULT_NUM_RESPONSE_ENTRIES, MAX_NUM_RESPONSE_ENTRIES};
use ch_lib::model::{ids::{
    message::IdsMessage,
    request::ClearingHouseMessage,
    response::IdsResponse
}, ServerInfo, OwnerList, DataTransaction};
use ch_lib::crypto::get_jwks;
use ch_lib::db::ProcessStore;
use ch_lib::model::constants::{ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_LOG_API, ROCKET_QUERY_API, ROCKET_PROCESS_API, ROCKET_PK_API};
use rocket::serde::json::{json, Json};
use rocket::fairing::AdHoc;
use std::convert::TryFrom;
use core_lib::model::SortingOrder;
use core_lib::model::SortingOrder::Ascending;

#[post( "/<pid>", format = "json", data = "<message>")]
async fn log(
    server_info: &State<ServerInfo>,
    apikey: ApiKey<IdsClaims, Empty>,
    db: &State<ProcessStore>,
    doc_api: &State<DocumentApiClient>,
    key_path: &State<String>,
    message: Json<ClearingHouseMessage>,
    pid: String
) -> IdsResponse {
    // Add non-InfoModel information to IdsMessage
    let msg = message.into_inner();
    let mut m = msg.header;
    m.payload = msg.payload;
    m.payload_type = msg.payload_type;
    m.pid = Some(pid.clone());

    // filter out calls for default process id and call application logic
    let api_response = match DEFAULT_PROCESS_ID.eq(pid.as_str()){
        true => {
            warn!("Log to default pid '{}' not allowed", DEFAULT_PROCESS_ID);
            ApiResponse::BadRequest(String::from("Document already exists"))
        },
        false => {
            // track if an error occurs
            let mut error = String::new();
            let mut unauth = false;
            // validate that there is a payload
            if m.payload.is_none(){
                error!("Trying to log an empty payload!");
                error = String::from("No payload received for logging");
            }

            // prepare credentials for authorization check
            let user = match apikey.sub() {
                Some(subject) => subject,
                None => {
                    // No credentials, ergo no authorization possible
                    error!("Cannot authorize user. Missing credentials");
                    error = String::from("Cannot authorize user. Missing credentials");
                    String::new()
                }
            };

            // convenience: if process does not exist, we create it but only if no error occurred before
            if error.is_empty(){
                match db.get_process(&pid).await{
                    Ok(Some(_p)) => {
                        debug!("Requested pid '{}' exists. Nothing to create.", &pid);
                    }
                    Ok(None) => {
                        info!("Requested pid '{}' does not exist. Creating...", &pid);
                        // create a new process
                        let new_process = Process::new(pid.clone(), vec!(user.clone()));

                        if db.store_process(new_process).await.is_err(){
                            error!("Error while creating process '{}'", &pid);
                            error = String::from("Error while creating process");
                        }
                    }
                    Err(_) => {
                        error!("Error while getting process '{}'", &pid);
                        error = String::from("Error while getting process");
                    }
                }
            }

            // now check if user is authorized to write to pid
            match db.is_authorized(&user, &pid).await {
                Ok(true) => info!("User authorized."),
                Ok(false) => {
                    warn!("User is not authorized to write to pid '{}'", &pid);
                    error = String::from("User not authorized");
                    unauth = true;
                }
                Err(_) => {
                    error!("Error while checking authorization of user '{}' for '{}'", &user, &pid);
                    error = String::from("Error during authorization");
                }
            }

            // if previously no error occured, log message
            if error.is_empty(){
                debug!("logging message for pid {}", &pid);
                log_message(apikey, db,user, doc_api, key_path.inner().as_str(), m.clone()).await
            }
            else{
                if unauth{
                    ApiResponse::Unauthorized(String::from("User not authorized."))
                }
                else{
                    ApiResponse::InternalError(error)
                }
            }
        }
    };
    // Depending on api_response create the correct IDS message
    let ids_response_message = IdsMessage::respond_to(m, &server_info);
    IdsResponse::respond(api_response, ids_response_message)
}

#[post( "/<pid>", format = "json", data = "<message>")]
async fn create_process(
    server_info: &State<ServerInfo>,
    apikey: ApiKey<IdsClaims, Empty>,
    db: &State<ProcessStore>,
    message: Json<ClearingHouseMessage>,
    pid: String
) -> IdsResponse {
    let msg = message.into_inner();
    let mut m = msg.header;
    m.payload = msg.payload;
    m.payload_type = msg.payload_type;

    // validate payload (happens mostly in camel)
    let payload = m.payload.clone().unwrap_or(String::new());

    // check if the pid already exists
    let api_response = match db.get_process(&pid).await{
        Ok(Some(p)) => {
            warn!("Requested pid '{}' already exists.", &p.id);
            ApiResponse::BadRequest(String::from("Process already exists"))
        }
        _ => {
            // filter out calls for default process id
            match DEFAULT_PROCESS_ID.eq(pid.as_str()) {
                true => {
                    warn!("Log to default pid '{}' not allowed", DEFAULT_PROCESS_ID);
                    ApiResponse::BadRequest(String::from("Document already exists"))
                },
                false => {
                    // prepare credentials of user
                    match apikey.sub() {
                        None => {
                            // No credentials, so we can't identify the owner later.
                            error!("Cannot create pid without user credentials");
                            ApiResponse::BadRequest(String::from("Cannot create pid without user credentials"))
                        },
                        Some(user) => {
                            // track if an error occurs
                            let mut error = String::new();
                            let mut owners = vec!(user);
                            // check list of owners and add them if they exist
                            if !payload.is_empty() {
                                debug!("OwnerList: '{:#?}'", &payload);
                                match serde_json::from_str::<OwnerList>(&payload){
                                    Ok(owner_list) => {
                                        for o in owner_list.owners{
                                            if !owners.contains(&o){
                                                owners.push(o);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        error!("Error while creating process '{}': {}", &pid, e);
                                        error = String::from("Error while creating process");
                                    }
                                };

                            }
                            info!("Requested pid '{}' will have {} owners", &pid, owners.len());

                            // if previously no error occured, create a new process and store it
                            if error.is_empty(){
                                // create process
                                info!("Requested pid '{}' does not exist. Creating...", &pid);
                                let new_process = Process::new(pid.clone(), owners);

                                match db.store_process(new_process).await{
                                    Ok(_) => {
                                        ApiResponse::SuccessCreate(json!(pid.clone()))
                                    }
                                    Err(e) => {
                                        error!("Error while creating process '{}': {}", &pid, e);
                                        ApiResponse::InternalError(String::from("Error while creating process"))
                                    }
                                }
                            }
                            else{
                                ApiResponse::BadRequest(error)
                            }
                        }
                    }
                }
            }
        }
    };

    // Depending on api_response create the correct IDS message
    let ids_response_message = IdsMessage::respond_to(m, &server_info);
    IdsResponse::respond(api_response, ids_response_message)
}

#[post( "/<_pid>", format = "json", data = "<message>", rank=50)]
async fn unauth_create_process(
    server_info: &State<ServerInfo>,
    message: Json<ClearingHouseMessage>,
    _pid: String
) -> IdsResponse {
    let msg = message.into_inner();
    let ids_response_msg = IdsMessage::respond_to(msg.header, &server_info);
    IdsResponse::new(ApiResponse::Unauthorized(String::from("Token not valid!")),IdsMessage::error(ids_response_msg))
}

async fn log_message(
    apikey: ApiKey<IdsClaims, Empty>,
    db: &State<ProcessStore>,
    user: String,
    doc_api: &State<DocumentApiClient>,
    key_path: &str,
    message: IdsMessage
) -> ApiResponse {
    debug!("transforming message to document...");
    let payload = message.payload.as_ref().unwrap().clone();
    // transform message to document
    let mut doc = Document::from(message);
    match db.get_transaction_counter().await{
        Ok(Some(tid)) => {
            debug!("Storing document...");
            doc.tc = tid;
            return match doc_api.create_document(&apikey.raw, &doc){
                Ok(doc_receipt) => {
                    debug!("Increase transabtion counter");
                    match db.increment_transaction_counter().await{
                        Ok(Some(_tid)) => {
                            debug!("Creating receipt...");
                            let transaction = DataTransaction{
                                transaction_id: doc.get_formatted_tc(),
                                timestamp: doc_receipt.timestamp,
                                process_id: doc_receipt.pid,
                                document_id: doc_receipt.doc_id,
                                payload,
                                chain_hash: doc_receipt.chain_hash,
                                client_id: user,
                                clearing_house_version: env!("CARGO_PKG_VERSION").to_string(),
                            };
                            debug!("...done. Signing receipt...");
                            ApiResponse::SuccessCreate(json!(transaction.sign(key_path)))
                        }
                        _ => {
                            error!("Error while incrementing transaction id!");
                            ApiResponse::InternalError(String::from("Internal error while preparing transaction data"))
                        }
                    }

                },
                Err(e) => {
                    error!("Error while creating document: {:?}", e);
                    ApiResponse::BadRequest(String::from("Document already exists"))
                }
            }
        },
        _ => {
            error!("Error while getting transaction id!");
            ApiResponse::InternalError(String::from("Internal error while preparing transaction data"))
        }
    }
}

#[post("/<_pid>", format = "json", data = "<message>", rank=50)]
async fn unauth_log(server_info: &State<ServerInfo>, message: Json<ClearingHouseMessage>, _pid: Option<String>) -> IdsResponse {
    let msg = message.into_inner();
    let ids_response_msg = IdsMessage::respond_to(msg.header, &server_info);
    IdsResponse::new(ApiResponse::Unauthorized(String::from("Token not valid!")),IdsMessage::error(ids_response_msg))
}

#[post("/<pid>?<page>&<size>&<sort>", format = "json", data = "<message>")]
async fn query_pid(
    server_info: &State<ServerInfo>,
    apikey: ApiKey<IdsClaims, Empty>,
    db: &State<ProcessStore>,
    page: Option<i32>,
    size: Option<i32>,
    sort: Option<SortingOrder>,
    doc_api: &State<DocumentApiClient>,
    pid: String,
    message: Json<ClearingHouseMessage>
) -> IdsResponse {
    debug!("page: {:#?}, size:{:#?} and sort:{:#?}", page, size, sort);

    let mut authorized = false;

    // prepare credentials for authorization check
    let user = match apikey.sub() {
        Some(subject) => subject,
        None => {
            // No credentials, ergo no authorization possible
            error!("Cannot authorize user. Missing credentials");
            String::new()
        }
    };

    // now check if user is authorized to read infos in pid
    match db.is_authorized(&user, &pid).await {
        Ok(true) => {
            info!("User authorized.");
            authorized = true;
        },
        Ok(false) => {
            warn!("User is not authorized to write to pid '{}'", &pid);
        }
        Err(_) => {
            error!("Error while checking authorization of user '{}' for '{}'", &user, &pid);
        }
    }

    // sanity check for pagination
    let sanitized_page = match page{
        Some(p) => {
            if p >= 0{
                p
            }
            else{
                warn!("...invalid page requested. Falling back to 0.");
                1
            }
        },
        None => 1
    };

    let sanitized_size = match size{
        Some(s) => {
            let converted_max = i32::try_from(MAX_NUM_RESPONSE_ENTRIES).unwrap();
            if s > converted_max{
                warn!("...invalid size requested. Falling back to default.");
                converted_max
            }
            else{
                if s > 0 {
                    s
                }
                else{
                    warn!("...invalid size requested. Falling back to default.");
                    i32::try_from(DEFAULT_NUM_RESPONSE_ENTRIES).unwrap()
                }
            }
        },
        None => i32::try_from(DEFAULT_NUM_RESPONSE_ENTRIES).unwrap()
    };

    let sanitized_sort = match sort{
        Some(s) => s,
        None => Ascending
    };

    let api_response =
        if !authorized {
            ApiResponse::Unauthorized(String::from("User not authorized."))
        }
        else {
            match doc_api.get_documents_for_pid_paginated(&apikey.raw, &pid, sanitized_page, sanitized_size, sanitized_sort){
                Ok(docs) => {
                    let messages: Vec<IdsMessage> = docs.iter().map(|d|IdsMessage::from(d.clone())).collect();
                    ApiResponse::SuccessOk(json!(messages))
                },
                Err(e) => {
                    error!("Error while retrieving message: {:?}", e);
                    ApiResponse::InternalError(format!("Error while retrieving messages for pid {}!", &pid))
                }
            }
        };

    // Depending on api_response create the correct IDS message in response to the incoming message
    let msg = message.into_inner();
    let ids_response_message = IdsMessage::respond_to(msg.header, &server_info);
    IdsResponse::respond(api_response, ids_response_message)
}

#[post("/<_pid>", rank=50, format = "json", data = "<message>")]
async fn unauth_query_pid(server_info: &State<ServerInfo>, _pid: String, message: Json<ClearingHouseMessage>) -> IdsResponse {
    let msg = message.into_inner();
    let ids_response_msg = IdsMessage::respond_to(msg.header, &server_info);
    IdsResponse::new(ApiResponse::Unauthorized(String::from("Token not valid!")),IdsMessage::error(ids_response_msg))
}

#[post("/<pid>/<id>", format = "json", data = "<message>")]
async fn query_id(server_info: &State<ServerInfo>, apikey: ApiKey<IdsClaims, Empty>, db: &State<ProcessStore>, doc_api: &State<DocumentApiClient>, pid: String, id: String, message: Json<ClearingHouseMessage>) -> IdsResponse {

    let mut authorized = false;

    // prepare credentials for authorization check
    let user = match apikey.sub() {
        Some(subject) => subject,
        None => {
            // No credentials, ergo no authorization possible
            error!("Cannot authorize user. Missing credentials");
            String::new()
        }
    };

    // now check if user is authorized to read infos in pid
    match db.is_authorized(&user, &pid).await {
        Ok(true) => {
            info!("User authorized.");
            authorized = true;
        },
        Ok(false) => {
            warn!("User is not authorized to write to pid '{}'", &pid);
        }
        Err(_) => {
            error!("Error while checking authorization of user '{}' for '{}'", &user, &pid);
        }
    }

    let api_response =
        if !authorized {
            ApiResponse::Unauthorized(String::from("User not authorized."))
        }
        else {
            match doc_api.get_document(&apikey.raw, &pid, &id) {
                Ok(Some(doc)) => {
                    // transform document to IDS message
                    let queried_message = IdsMessage::from(doc);
                    ApiResponse::SuccessOk(json!(queried_message))
                },
                Ok(None) => {
                    debug!("Queried a non-existing document: {}", &id);
                    ApiResponse::NotFound(format!("No message found with id {}!", &id))
                },
                Err(e) => {
                    error!("Error while retrieving message: {:?}", e);
                    ApiResponse::InternalError(format!("Error while retrieving message with id {}!", &id))
                }
            }
        };
    // Depending on api_response create the correct IDS message in response to the incoming message
    let msg = message.into_inner();
    trace!("answer message: {:#?}", &msg);
    let ids_response_message = IdsMessage::respond_to(msg.header, &server_info);
    IdsResponse::respond(api_response, ids_response_message)
}

#[post("/<_pid>/<_id>", rank=50, format = "json", data = "<message>")]
async fn unauth_query_id(server_info: &State<ServerInfo>, _pid: String, _id: String, message: Json<ClearingHouseMessage>) -> IdsResponse {
    let msg = message.into_inner();
    let ids_response_msg = IdsMessage::respond_to(msg.header, &server_info);
    IdsResponse::new(ApiResponse::Unauthorized(String::from("Token not valid!")),IdsMessage::error(ids_response_msg))
}

#[get("/.well-known/jwks.json", format = "json")]
async fn get_public_sign_key(key_path: &State<String>) -> ApiResponse {
    match get_jwks(key_path.as_str()){
        Some(jwks) => ApiResponse::SuccessOk(json!(jwks)),
        None => ApiResponse::InternalError(String::from("Error reading signing key"))
    }
}


pub fn mount_api() -> AdHoc {
    AdHoc::on_ignite("Mounting Clearing House API", |rocket| async {
        rocket
            .mount(format!("{}{}", ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_LOG_API).as_str(), routes![log, unauth_log])
            .mount(format!("{}", ROCKET_PROCESS_API).as_str(), routes![create_process, unauth_create_process])
            .mount(format!("{}{}", ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_QUERY_API).as_str(),
                   routes![query_id, query_pid, unauth_query_id, unauth_query_pid])
            .mount(format!("{}", ROCKET_PK_API).as_str(), routes![get_public_sign_key])
    })
}