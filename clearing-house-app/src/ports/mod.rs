//! # Ports
//!
//! This module contains the ports of the logging service. Ports are used to communicate with other
//! services. In this case, the logging service implements REST-API endpoints to provide access to
//! the logging service.
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

/// Result type alias for the API
pub(crate) type ApiResult<T, E> = Result<(axum::http::StatusCode, axum::response::Json<T>), E>;