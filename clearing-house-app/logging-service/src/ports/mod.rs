//! # Ports
//!
//! This module contains the ports of the logging service. Ports are used to communicate with other
//! services. In this case, the logging service implements REST-API endpoints to provide access to
//! the logging service.
pub(crate) mod logging_api;
pub(crate) mod doc_type_api;



#[derive(rocket::Responder, Debug)]
pub enum ApiResponse {
    #[response(status = 200)]
    PreFlight(()),
    #[response(status = 400, content_type = "text/plain")]
    BadRequest(String),
    #[response(status = 201, content_type = "json")]
    SuccessCreate(rocket::serde::json::Value),
    #[response(status = 200, content_type = "json")]
    SuccessOk(rocket::serde::json::Value),
    #[response(status = 204, content_type = "text/plain")]
    SuccessNoContent(String),
    #[response(status = 401, content_type = "text/plain")]
    Unauthorized(String),
    #[response(status = 403, content_type = "text/plain")]
    Forbidden(String),
    #[response(status = 404, content_type = "text/plain")]
    NotFound(String),
    #[response(status = 500, content_type = "text/plain")]
    InternalError(String),
}