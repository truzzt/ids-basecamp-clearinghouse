use axum::http::StatusCode;
use axum::Json;
use crate::model::claims::ExtractChClaims;
use crate::{model::claims::get_jwks, model::SortingOrder, AppState};
use biscuit::jwk::JWKSet;

use crate::model::ids::message::IdsMessage;
use crate::model::ids::request::ClearingHouseMessage;
use crate::model::ids::IdsQueryResult;
use crate::model::process::Receipt;
use crate::services::logging_service::LoggingServiceError;

type LoggingApiResult<T> = super::ApiResult<T, LoggingServiceError>;

async fn log(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> LoggingApiResult<Receipt> {
    match state
        .logging_service
        .log(ch_claims, state.signing_key_path.as_str(), message, pid)
        .await
    {
        Ok(id) => Ok((StatusCode::CREATED, Json(id))),
        Err(e) => {
            error!("Error while logging: {:?}", e);
            Err(e)
        }
    }
}

#[derive(serde::Serialize)]
struct CreateProcessResponse {
    pub pid: String,
}

async fn create_process(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> LoggingApiResult<CreateProcessResponse> {
    match state
        .logging_service
        .create_process(ch_claims, message, pid)
        .await
    {
        Ok(id) => Ok((StatusCode::CREATED, Json(CreateProcessResponse { pid: id }))),
        Err(e) => {
            error!("Error while creating process: {:?}", e);
            Err(e)
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
    axum::extract::Json(_): axum::extract::Json<ClearingHouseMessage>,
) -> LoggingApiResult<IdsQueryResult> {
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
        )
        .await
    {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => {
            error!("Error while querying: {:?}", e);
            Err(e)
        }
    }
}

async fn query_id(
    ExtractChClaims(ch_claims): ExtractChClaims,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(message): axum::extract::Json<ClearingHouseMessage>,
) -> LoggingApiResult<IdsMessage> {
    match state
        .logging_service
        .query_id(ch_claims, pid, id, message)
        .await
    {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => {
            error!("Error while querying: {:?}", e);
            Err(e)
        }
    }
}

async fn get_public_sign_key(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> super::ApiResult<JWKSet<biscuit::Empty>, &'static str> {
    match get_jwks(state.signing_key_path.as_str()) {
        Some(jwks) => Ok((StatusCode::OK, Json(jwks))),
        None => Err("Error reading signing key"),
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
