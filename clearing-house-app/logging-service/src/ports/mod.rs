//! # Ports
//!
//! This module contains the ports of the logging service. Ports are used to communicate with other
//! services. In this case, the logging service implements REST-API endpoints to provide access to
//! the logging service.
pub(crate) mod logging_api;
pub(crate) mod doc_type_api;
