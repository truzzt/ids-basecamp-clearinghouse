use biscuit::Empty;
use rocket::State;
use rocket_contrib::json::Json;
use core_lib::{
    api::{
        ApiResponse,
        auth::ApiKey,
        client::{
            blockchain_api::BlockchainApiClient,
            document_api::DocumentApiClient,
        },
        claims::IdsClaims,
        HashMessage
    },
    constants::DEFAULT_PROCESS_ID,
    model::{
        document::Document,
        process::Process
    }
};
use ch_lib::model::{
    ids::{
        message::IdsMessage,
        request::ClearingHouseMessage,
        response::IdsResponse
    },
    ServerInfo
};
use ch_lib::model::constants::{ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_LOG_API, ROCKET_QUERY_API};
use ch_lib::db::ProcessStore;

#[post( "/<pid>", format = "json", data = "<message>")]
fn log(
    server_info: State<ServerInfo>,
    apikey: ApiKey<IdsClaims, Empty>,
    db: State<ProcessStore>,
    bc_api: State<BlockchainApiClient>,
    doc_api: State<DocumentApiClient>,
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
                if db.get_process(&pid).is_err(){
                    info!("Requested pid '{}' does not exist. Creating...", &pid);
                    // create a new process
                    let new_process = Process::new(pid.clone(), user.clone());

                    if db.store_process(new_process).is_err(){
                        error!("Error while creating process '{}'", &pid);
                        error = String::from("Error while creating process");
                    }
                }
            }

            // now check if user is authorized to write to pid
            match db.is_authorized(&user, &pid) {
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

            if db.get_process(&pid).is_err(){
                info!("Requested pid '{}' does not exist. Creating...", &pid);
                // create a new process
                let process = match apikey.sub() {
                    Some(subject) => Process::new(pid.clone(), subject),
                    None => {
                        // We identify the user with the subject. So if it's missing we cannot proceed.
                        error!("Error while creating process '{}'. User unknown.", &pid);
                        error = String::from("Error while creating process. User unknown.");
                        Process::new(pid.clone(), "".to_string())
                    }
                };
                if error.is_empty(){
                    // if we already encountered an error, we will not store the pid.
                    if db.store_process(process).is_err(){
                        error!("Error while creating process '{}'", &pid);
                        error = String::from("Error while creating process");
                    }
                }
            }
            // if previously no error occured, log message
            if error.is_empty(){
                debug!("logging message for pid {}", &pid);
                log_message(apikey, bc_api, doc_api, m.clone())
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

fn log_message(
    apikey: ApiKey<IdsClaims, Empty>,
    bc_api: State<BlockchainApiClient>,
    doc_api: State<DocumentApiClient>,
    message: IdsMessage
) -> ApiResponse {
    debug!("transforming message to document...");
    // transform message to document
    let doc = Document::from(message);
    return match doc_api.create_document(&apikey.raw, &doc){
        Ok(hm) => {
            let doc_id = hm.doc_id;
            match bc_api.store_hash(&doc.pid, &doc_id, &hm.hash){
                Ok(_b) => {
                    ApiResponse::SuccessCreate(json!(HashMessage::new("true", "Log entry created", &doc_id, hm.hash.as_str())))
                },
                Err(e) => {
                    // we created a document, but couldn't store the hash, so we get rid of it (best effort)
                    let _b = doc_api.delete_document(&apikey.raw, &doc.pid, &doc_id);
                    error!("Could not store hash on block chain: {:?}", e);
                    ApiResponse::InternalError(String::from("Error while storing hash on blockchain"))
                }
            }
        },
        Err(e) => {
            error!("Error while creating document: {:?}", e);
            ApiResponse::BadRequest(String::from("Document already exists"))
        }
    }
}

#[post("/<_pid>", format = "json", data = "<message>", rank=50)]
fn unauth_log(server_info: State<ServerInfo>, message: Json<ClearingHouseMessage>, _pid: Option<String>) -> IdsResponse {
    let msg = message.into_inner();
    let ids_response_msg = IdsMessage::respond_to(msg.header, &server_info);
    IdsResponse::new(ApiResponse::Unauthorized(String::from("Token not valid!")),IdsMessage::error(ids_response_msg))
}

#[post("/<pid>", format = "json", data = "<message>")]
fn query_pid(
    server_info: State<ServerInfo>,
    apikey: ApiKey<IdsClaims, Empty>,
    db: State<ProcessStore>,
    doc_api: State<DocumentApiClient>,
    pid: String,
    message: Json<ClearingHouseMessage>
) -> IdsResponse {
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
    match db.is_authorized(&user, &pid) {
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
            match doc_api.get_documents_for_pid(&apikey.raw, &pid){
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
fn unauth_query_pid(server_info: State<ServerInfo>, _pid: String, message: Json<ClearingHouseMessage>) -> IdsResponse {
    let msg = message.into_inner();
    let ids_response_msg = IdsMessage::respond_to(msg.header, &server_info);
    IdsResponse::new(ApiResponse::Unauthorized(String::from("Token not valid!")),IdsMessage::error(ids_response_msg))
}

#[post("/<pid>/<id>", format = "json", data = "<message>")]
fn query_id(server_info: State<ServerInfo>, apikey: ApiKey<IdsClaims, Empty>, db: State<ProcessStore>, doc_api: State<DocumentApiClient>, pid: String, id: String, message: Json<ClearingHouseMessage>) -> IdsResponse {

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
    match db.is_authorized(&user, &pid) {
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
                Ok(doc) => {
                    // transform document to IDS message
                    let queried_message = IdsMessage::from(doc);
                    ApiResponse::SuccessOk(json!(queried_message))
                },
                Err(e) => {
                    error!("Error while retrieving message: {:?}", e);
                    ApiResponse::NotFound(format!("Error while retrieving message with id {}!", &id))
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
fn unauth_query_id(server_info: State<ServerInfo>, _pid: String, _id: String, message: Json<ClearingHouseMessage>) -> IdsResponse {
    let msg = message.into_inner();
    let ids_response_msg = IdsMessage::respond_to(msg.header, &server_info);
    IdsResponse::new(ApiResponse::Unauthorized(String::from("Token not valid!")),IdsMessage::error(ids_response_msg))
}

pub fn mount(rocket: rocket::Rocket, server_info: ServerInfo) -> rocket::Rocket {
    rocket
        .manage(server_info)
        .mount(format!("{}{}", ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_LOG_API).as_str(), routes![log, unauth_log])
        .mount(format!("{}{}", ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_QUERY_API).as_str(),
               routes![query_id, query_pid, unauth_query_id, unauth_query_pid])
}