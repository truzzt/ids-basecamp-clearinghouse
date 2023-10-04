use crate::model::claims::ExtractChClaims;
use crate::{model::claims::get_jwks, model::SortingOrder, ports::ApiResponse, AppState};
use biscuit::jwk::JWKSet;

use crate::model::ids::message::IdsMessage;
use crate::model::ids::request::ClearingHouseMessage;
use crate::model::ids::IdsQueryResult;
use crate::model::process::Receipt;

async fn log(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> ApiResponse<Receipt> {
    match state
        .logging_service
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

async fn create_process(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> ApiResponse<String> {
    match state
        .logging_service
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

#[derive(serde::Deserialize)]
struct QueryParams {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sort: Option<SortingOrder>,
    pub date_to: Option<String>,
    pub date_from: Option<String>,
}

async fn query_pid(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(params): axum::extract::Query<QueryParams>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> ApiResponse<IdsQueryResult> {
    match state
        .logging_service
        .query_pid(
            ch_claims,
            params.page,
            params.size,
            params.sort,
            params.date_to,
            params.date_from,
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

async fn query_id(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> ApiResponse<IdsMessage> {
    match state
        .logging_service
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

async fn get_public_sign_key(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResponse<JWKSet<biscuit::Empty>> {
    match get_jwks(state.signing_key_path.as_str()) {
        Some(jwks) => ApiResponse::SuccessOk(jwks),
        None => ApiResponse::InternalError(String::from("Error reading signing key")),
    }
}

pub(crate) fn router() -> axum::routing::Router<AppState> {
    axum::Router::new()
        .route("/messages/log/:pid", axum::routing::post(log))
        .route("/process/:pid", axum::routing::post(create_process))
        .route("/messages/query/:pid", axum::routing::post(query_pid))
        .route("/messages/query/:pid/:id", axum::routing::post(query_id))
        .route(
            "/.well-known/jwks.json",
            axum::routing::get(get_public_sign_key),
        )
}
