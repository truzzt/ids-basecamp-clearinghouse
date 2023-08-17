//! # Services
//!
//! This module contains the Application Services that are used by the API Controllers. It is
//! responsible for the business logic of the application. The services are used by the API
//! Controllers to handle the requests and responses.
//!
use crate::model::document::Document;

pub(crate) mod keyring_service;
pub(crate) mod document_service;
pub(crate) mod logging_service;


#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct DocumentReceipt{
    pub timestamp: i64,
    pub pid: String,
    pub doc_id: String,
    pub chain_hash: String,
}

impl DocumentReceipt{
    pub fn new(timestamp: i64, pid: &str, doc_id: &str, chain_hash: &str) -> DocumentReceipt{
        DocumentReceipt{
            timestamp,
            pid: pid.to_string(),
            doc_id: doc_id.to_string(),
            chain_hash: chain_hash.to_string(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct QueryResult{
    pub date_from: i64,
    pub date_to: i64,
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub order: String,
    pub documents: Vec<Document>
}

impl QueryResult{
    pub fn new(date_from: i64, date_to: i64, page: Option<i32>, size: Option<i32>, order: String, documents: Vec<Document>) -> QueryResult{
        QueryResult{
            date_from,
            date_to,
            page,
            size,
            order,
            documents
        }
    }
}