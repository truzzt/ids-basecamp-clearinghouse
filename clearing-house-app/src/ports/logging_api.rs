
use biscuit::jwk::JWKSet;
use crate::{AppState, model::claims::get_jwks, model::SortingOrder, ports::ApiResponse};
use crate::model::claims::ExtractChClaims;

use crate::model::constants::{
    ROCKET_CLEARING_HOUSE_BASE_API, ROCKET_LOG_API, ROCKET_PK_API, ROCKET_PROCESS_API,
    ROCKET_QUERY_API,
};
use crate::model::ids::IdsQueryResult;
use crate::model::ids::message::IdsMessage;
use crate::model::ids::request::ClearingHouseMessage;
use crate::model::process::Receipt;

//#[rocket::post("/<pid>", format = "json", data = "<message>")]
pub async fn log(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> ApiResponse<Receipt> {
    match state.logging_service
        .log(ch_claims, state.signing_key_path.as_str(), message, pid)
        .await
    {
        Ok(id) => ApiResponse::SuccessCreate(id),
        Err(e) => {
            error!("Error while logging: {:?}", e);
            ApiResponse::InternalError(String::from("Error while logging!"))
        }
    }
}

//#[rocket::post("/<pid>", format = "json", data = "<message>")]
pub async fn create_process(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> ApiResponse<String> {
    match state.logging_service
        .create_process(ch_claims, message, pid)
        .await
    {
        Ok(id) => ApiResponse::SuccessCreate(id),
        Err(e) => {
            error!("Error while creating process: {:?}", e);
            ApiResponse::InternalError(String::from("Error while creating process!"))
        }
    }
}

//#[rocket::post("/<_pid>", format = "json", rank = 50)]
/*async fn unauth(_pid: Option<String>) -> ApiResponse {
    ApiResponse::Unauthorized(String::from("Token not valid!"))
}*/

//#[rocket::post("/<_pid>/<_id>", format = "json", rank = 50)]
/*async fn unauth_id(_pid: Option<String>, _id: Option<String>) -> ApiResponse {
    ApiResponse::Unauthorized(String::from("Token not valid!"))
}*/

/*#[rocket::post(
    "/<pid>?<page>&<size>&<sort>&<date_to>&<date_from>",
    format = "json",
    data = "<message>"
)]*/
pub async fn query_pid(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(page): axum::extract::Query<Option<i32>>,
    axum::extract::Query(size): axum::extract::Query<Option<i32>>,
    axum::extract::Query(sort): axum::extract::Query<Option<SortingOrder>>,
    axum::extract::Query(date_to): axum::extract::Query<Option<String>>,
    axum::extract::Query(date_from): axum::extract::Query<Option<String>>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> ApiResponse<IdsQueryResult> {
    match state.logging_service
        .query_pid(
            ch_claims,
            page,
            size,
            sort,
            date_to,
            date_from,
            pid,
            message,
        )
        .await
    {
        Ok(result) => ApiResponse::SuccessOk(result),
        Err(e) => {
            error!("Error while querying: {:?}", e);
            ApiResponse::InternalError(String::from("Error while querying!"))
        }
    }
}

//#[rocket::post("/<pid>/<id>", format = "json", data = "<message>")]
pub async fn query_id(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> ApiResponse<IdsMessage> {
    match state.logging_service
        .query_id(ch_claims, pid, id, message)
        .await
    {
        Ok(result) => ApiResponse::SuccessOk(result),
        Err(e) => {
            error!("Error while querying: {:?}", e);
            ApiResponse::InternalError(String::from("Error while querying!"))
        }
    }
}

//#[rocket::get("/.well-known/jwks.json", format = "json")]
pub async fn get_public_sign_key(axum::extract::State(state): axum::extract::State<AppState>) -> ApiResponse<JWKSet<biscuit::Empty>> {
    match get_jwks(state.signing_key_path.as_str()) {
        Some(jwks) => ApiResponse::SuccessOk(jwks),
        None => ApiResponse::InternalError(String::from("Error reading signing key")),
    }
}
