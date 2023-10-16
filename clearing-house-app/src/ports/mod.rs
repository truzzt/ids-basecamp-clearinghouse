//! # Ports
//!
//! This module contains the ports of the logging service. Ports are used to communicate with other
//! services. In this case, the logging service implements REST-API endpoints to provide access to
//! the logging service.
use axum::response::Response;
use crate::AppState;

#[cfg(doc_type)]
pub(crate) mod doc_type_api;
pub(crate) mod logging_api;

/// Router for the logging service and the doc_type service
#[cfg(doc_type)]
pub(crate) fn router() -> axum::routing::Router<AppState> {
    axum::Router::new()
        .merge(ports::logging_api::router())
        .nest("/doctype", ports::doc_type_api::router());
}

/// Router for the logging service
#[cfg(not(doc_type))]
pub(crate) fn router() -> axum::routing::Router<AppState> {
    axum::Router::new()
        .merge(logging_api::router())
}

#[derive(Debug)]
pub(crate) enum ApiResponse<T: serde::Serialize> {
    PreFlight(()),
    BadRequest(String),
    SuccessCreate(T),
    SuccessOk(T),
    SuccessNoContent(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    InternalError(String),
}

impl<T: serde::Serialize> axum::response::IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::PreFlight(_) => (axum::http::StatusCode::OK, "").into_response(),
            ApiResponse::BadRequest(s) => (axum::http::StatusCode::BAD_REQUEST, s).into_response(),
            ApiResponse::SuccessCreate(v) => {
                (axum::http::StatusCode::CREATED, axum::response::Json(v)).into_response()
            }
            ApiResponse::SuccessOk(v) => {
                (axum::http::StatusCode::OK, axum::response::Json(v)).into_response()
            }
            ApiResponse::SuccessNoContent(s) => {
                (axum::http::StatusCode::NO_CONTENT, s).into_response()
            }
            ApiResponse::Unauthorized(s) => {
                (axum::http::StatusCode::UNAUTHORIZED, s).into_response()
            }
            ApiResponse::Forbidden(s) => (axum::http::StatusCode::FORBIDDEN, s).into_response(),
            ApiResponse::NotFound(s) => (axum::http::StatusCode::NOT_FOUND, s).into_response(),
            ApiResponse::InternalError(s) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, s).into_response()
            }
        }
    }
}
