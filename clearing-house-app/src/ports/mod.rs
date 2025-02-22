//! # Ports
//!
//! This module contains the ports of the logging service. Ports are used to communicate with other
//! services. In this case, the logging service implements REST-API endpoints to provide access to
//! the logging service.

use crate::AppState;
use crate::model::ids::RejectionMessage;

pub(crate) mod logging_api;

/// Router for the logging service
pub(crate) fn router() -> axum::routing::Router<AppState> {
    axum::Router::new().merge(logging_api::router())
}

/// Result type alias for the API
pub(crate) type ApiResult = Result<axum::response::Response, RejectionMessage>;
