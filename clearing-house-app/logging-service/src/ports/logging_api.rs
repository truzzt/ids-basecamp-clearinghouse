use core_lib::{
    api::{
        ApiResponse,
        crypto::{ChClaims, get_jwks},
    },
    model::SortingOrder,
};
use rocket::serde::json::{json, Json};
use rocket::fairing::AdHoc;
use rocket::State;

use crate::model::ids::request::ClearingHouseMessage;
use crate::model::constants::{ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_LOG_API, ROCKET_QUERY_API, ROCKET_PROCESS_API, ROCKET_PK_API};
use crate::services::logging_service::LoggingService;

#[post("/<pid>", format = "json", data = "<message>")]
async fn log(
    ch_claims: ChClaims,
    logging_api: &State<LoggingService>,
    key_path: &State<String>,
    message: Json<ClearingHouseMessage>,
    pid: String,
) -> ApiResponse {
    match logging_api.inner().log(ch_claims, key_path, message.into_inner(), pid).await {
        Ok(id) => ApiResponse::SuccessCreate(json!(id)),
        Err(e) => {
            error!("Error while logging: {:?}", e);
            ApiResponse::InternalError(String::from("Error while logging!"))
        }
    }
}

#[post("/<pid>", format = "json", data = "<message>")]
async fn create_process(
    ch_claims: ChClaims,
    logging_api: &State<LoggingService>,
    message: Json<ClearingHouseMessage>,
    pid: String,
) -> ApiResponse {
    match logging_api.inner().create_process(ch_claims, message.into_inner(), pid).await {
        Ok(id) => ApiResponse::SuccessCreate(json!(id)),
        Err(e) => {
            error!("Error while creating process: {:?}", e);
            ApiResponse::InternalError(String::from("Error while creating process!"))
        }
    }
}

#[post("/<_pid>", format = "json", rank = 50)]
async fn unauth(_pid: Option<String>) -> ApiResponse {
    ApiResponse::Unauthorized(String::from("Token not valid!"))
}

#[post("/<_pid>/<_id>", format = "json", rank = 50)]
async fn unauth_id(_pid: Option<String>, _id: Option<String>) -> ApiResponse {
    ApiResponse::Unauthorized(String::from("Token not valid!"))
}

#[post("/<pid>?<page>&<size>&<sort>&<date_to>&<date_from>", format = "json", data = "<message>")]
async fn query_pid(
    ch_claims: ChClaims,
    logging_api: &State<LoggingService>,
    page: Option<i32>,
    size: Option<i32>,
    sort: Option<SortingOrder>,
    date_to: Option<String>,
    date_from: Option<String>,
    pid: String,
    message: Json<ClearingHouseMessage>,
) -> ApiResponse {
    match logging_api.inner().query_pid(ch_claims, page, size, sort, date_to, date_from, pid, message.into_inner()).await {
        Ok(result) => ApiResponse::SuccessOk(json!(result)),
        Err(e) => {
            error!("Error while querying: {:?}", e);
            ApiResponse::InternalError(String::from("Error while querying!"))
        }
    }
}

#[post("/<pid>/<id>", format = "json", data = "<message>")]
async fn query_id(
    ch_claims: ChClaims,
    logging_api: &State<LoggingService>,
    pid: String,
    id: String,
    message: Json<ClearingHouseMessage>,
) -> ApiResponse {
    match logging_api.inner().query_id(ch_claims, pid, id, message.into_inner()).await {
        Ok(result) => ApiResponse::SuccessOk(json!(result)),
        Err(e) => {
            error!("Error while querying: {:?}", e);
            ApiResponse::InternalError(String::from("Error while querying!"))
        }
    }
}

#[get("/.well-known/jwks.json", format = "json")]
async fn get_public_sign_key(key_path: &State<String>) -> ApiResponse {
    match get_jwks(key_path.as_str()) {
        Some(jwks) => ApiResponse::SuccessOk(json!(jwks)),
        None => ApiResponse::InternalError(String::from("Error reading signing key"))
    }
}

pub fn mount_api() -> AdHoc {
    AdHoc::on_ignite("Mounting Clearing House API", |rocket| async {
        rocket
            .mount(format!("{}{}", ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_LOG_API).as_str(), routes![log, unauth])
            .mount(format!("{}", ROCKET_PROCESS_API).as_str(), routes![create_process, unauth])
            .mount(format!("{}{}", ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_QUERY_API).as_str(),
                   routes![query_id, query_pid, unauth, unauth_id])
            .mount(format!("{}", ROCKET_PK_API).as_str(), routes![get_public_sign_key])
    })
}