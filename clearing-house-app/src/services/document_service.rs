use crate::db::doc_store::DataStore;
use crate::model::claims::ChClaims;
use crate::model::constants::{
    DEFAULT_DOC_TYPE, DEFAULT_NUM_RESPONSE_ENTRIES, MAX_NUM_RESPONSE_ENTRIES, PAYLOAD_PART,
};
use crate::model::crypto::{KeyCt, KeyCtList};
use crate::model::document::Document;
use crate::model::{parse_date, validate_and_sanitize_dates, SortingOrder};
use crate::services::keyring_service::KeyringService;
use crate::services::{DocumentReceipt, QueryResult};
use std::convert::TryFrom;
use std::sync::Arc;

/// Error type for DocumentService
#[derive(thiserror::Error, Debug)]
pub enum DocumentServiceError {
    #[error("Document already exists!")]
    DocumentAlreadyExists,
    #[error("Document contains no payload!")]
    MissingPayload,
    #[error("Error during database operation: {description}: {source}")]
    DatabaseError {
        source: anyhow::Error,
        description: String,
    },
    #[error("Error while retrieving keys from keyring!")]
    KeyringServiceError(#[from] crate::services::keyring_service::KeyringServiceError),
    #[error("Invalid dates in query!")]
    InvalidDates,
    #[error("Document not found!")]
    NotFound,
    #[error("Key Ciphertext corrupted!")]
    CorruptedCiphertext(#[from] hex::FromHexError),
    #[error("Error while encrypting!")]
    EncryptionError,
}

impl axum::response::IntoResponse for DocumentServiceError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;
        match self {
            Self::DocumentAlreadyExists => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Self::MissingPayload => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::DatabaseError {
                source,
                description,
            } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{}: {}", description, source),
            )
                .into_response(),
            Self::KeyringServiceError(e) => e.into_response(),
            Self::InvalidDates => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::CorruptedCiphertext(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::EncryptionError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DocumentService {
    db: DataStore,
    key_api: Arc<KeyringService>,
}

impl DocumentService {
    pub fn new(db: DataStore, key_api: Arc<KeyringService>) -> Self {
        Self { db, key_api }
    }

    #[tracing::instrument(skip_all)]
    pub(crate) async fn create_enc_document(
        &self,
        ch_claims: ChClaims,
        doc: Document,
    ) -> Result<DocumentReceipt, DocumentServiceError> {
        trace!("...user '{:?}'", &ch_claims.client_id);
        // data validation
        let payload: Vec<String> = doc
            .parts
            .iter()
            .filter(|p| *PAYLOAD_PART == p.name)
            .map(|p| p.content.clone())
            .collect();

        // If the document contains more than 1 payload we panic. This should never happen!
        assert!(
            payload.len() <= 1,
            "Document contains two or more payloads!"
        );
        if payload.is_empty() {
            return Err(DocumentServiceError::MissingPayload);
        }

        // check if doc id already exists
        match self.db.exists_document(&doc.id).await {
            Ok(true) => {
                warn!("Document exists already!");
                Err(DocumentServiceError::DocumentAlreadyExists)
            }
            _ => {
                trace!("getting keys");

                // TODO: This needs some attention, because keyring api called `create_service_token` on `ch_claims`
                let keys = match self
                    .key_api
                    .generate_keys(ch_claims, doc.pid.clone(), doc.dt_id.clone())
                    .await
                {
                    Ok(key_map) => {
                        debug!("got keys");
                        Ok(key_map)
                    }
                    Err(e) => {
                        error!("Error while retrieving keys: {:?}", e);
                        Err(DocumentServiceError::KeyringServiceError(e))
                    }
                }?;

                debug!("start encryption");
                let enc_doc = match doc.encrypt(keys) {
                    Ok(ct) => {
                        debug!("got ct");
                        Ok(ct)
                    }
                    Err(e) => {
                        error!("Error while encrypting: {:?}", e);
                        Err(DocumentServiceError::EncryptionError)
                    }
                }?;

                // prepare the success result message
                let receipt =
                    DocumentReceipt::new(enc_doc.ts, &enc_doc.pid, &enc_doc.id);

                trace!("storing document ....");
                // store document
                match self.db.add_document(enc_doc).await {
                    Ok(_b) => Ok(receipt),
                    Err(e) => {
                        error!("Error while adding: {:?}", e);
                        Err(DocumentServiceError::DatabaseError {
                            source: e,
                            description: "Error while adding document".to_string(),
                        })
                    }
                }
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub(crate) async fn get_enc_documents_for_pid(
        &self,
        ch_claims: ChClaims,
        page: Option<u64>,
        size: Option<u64>,
        sort: Option<SortingOrder>,
        (date_from, date_to): (Option<String>, Option<String>),
        pid: String,
    ) -> Result<QueryResult, DocumentServiceError> {
        debug!("Trying to retrieve documents for pid '{}'...", &pid);
        trace!("...user '{:?}'", &ch_claims.client_id);
        debug!(
            "...page: {:#?}, size:{:#?} and sort:{:#?}",
            page, size, sort
        );

        let dt_id = String::from(DEFAULT_DOC_TYPE);
        let sanitized_page = Self::sanitize_page(page);
        let sanitized_size = Self::sanitize_size(size);

        // Sorting order is already validated and defaults to descending
        let sanitized_sort = sort.unwrap_or(SortingOrder::Descending);

        // Parsing the dates for duration queries
        let parsed_date_from = parse_date(date_from, false);
        let parsed_date_to = parse_date(date_to, true);

        // Validation of dates with various checks. If none given dates default to date_now (date_to) and (date_now - 2 weeks) (date_from)
        let Ok((sanitized_date_from, sanitized_date_to)) =
            validate_and_sanitize_dates(parsed_date_from, parsed_date_to, None)
        else {
            debug!("date validation failed!");
            return Err(DocumentServiceError::InvalidDates);
        };

        //new behavior: if pages are "invalid" return {}. Do not adjust page
        //either call db with type filter or without to get cts
        debug!(
            "... using pagination with page: {}, size:{} and sort:{:#?}",
            sanitized_page, sanitized_size, &sanitized_sort
        );

        let cts = match self
            .db
            .get_documents_for_pid(
                &dt_id,
                &pid,
                sanitized_page,
                sanitized_size,
                &sanitized_sort,
                (&sanitized_date_from, &sanitized_date_to),
            )
            .await
        {
            Ok(cts) => cts,
            Err(e) => {
                error!("Error while retrieving document: {:?}", e);
                return Err(DocumentServiceError::DatabaseError {
                    source: e,
                    description: "Error while retrieving document".to_string(),
                });
            }
        };

        let result_size = i32::try_from(sanitized_size).ok();
        let result_page = i32::try_from(sanitized_page).ok();
        let result_sort = match sanitized_sort {
            SortingOrder::Ascending => String::from("asc"),
            SortingOrder::Descending => String::from("desc"),
        };

        let mut result = QueryResult::new(
            sanitized_date_from.timestamp(),
            sanitized_date_to.timestamp(),
            result_page,
            result_size,
            result_sort,
            vec![],
        );

        // The db might contain no documents in which case we get an empty vector
        if cts.is_empty() {
            debug!("Queried empty pid: {}", &pid);
            Ok(result)
        } else {
            // Documents found for pid, now decrypting them
            debug!(
                "Found {} documents. Getting keys from keyring...",
                cts.len()
            );
            let key_cts: Vec<KeyCt> = cts
                .iter()
                .map(|e| KeyCt::new(e.id.clone(), e.keys_ct.clone()))
                .collect();
            // caution! we currently only support a single dt per call, so we use the first dt we found
            let key_cts_list = KeyCtList::new(cts[0].dt_id.clone(), key_cts);
            // decrypt cts
            // TODO: This method needs some attention, because keyring api called `create_service_token` on `ch_claims`
            let key_maps = match self
                .key_api
                .decrypt_multiple_keys(ch_claims, Some(pid), &key_cts_list)
                .await
            {
                Ok(key_map) => key_map,
                Err(e) => {
                    error!("Error while retrieving keys from keyring: {:?}", e);
                    return Err(DocumentServiceError::KeyringServiceError(e));
                }
            };
            debug!("... keys received. Starting decryption...");
            let pts_bulk: Vec<Document> = cts
                .iter()
                .zip(key_maps.iter())
                .filter_map(|(ct, key_map)| {
                    if ct.id != key_map.id {
                        error!("Document and map don't match");
                    };
                    match ct.decrypt(key_map.map.keys.clone()) {
                        Ok(d) => Some(d),
                        Err(e) => {
                            warn!("Got empty document from decryption! {:?}", e);
                            None
                        }
                    }
                })
                .collect();
            debug!("...done.");

            result.documents = pts_bulk;
            Ok(result)
        }
    }

    #[tracing::instrument(skip_all)]
    pub(crate) async fn get_enc_document(
        &self,
        ch_claims: ChClaims,
        pid: String,
        id: String,
        hash: Option<String>,
    ) -> Result<Document, DocumentServiceError> {
        trace!("...user '{:?}'", &ch_claims.client_id);
        trace!(
            "trying to retrieve document with id '{}' for pid '{}'",
            &id,
            &pid
        );
        if let Some(hash) = hash {
            debug!("integrity check with hash: {}", hash);
        }

        match self.db.get_document(&id, &pid).await {
            //TODO: would like to send "{}" instead of "null" when dt is not found
            Ok(Some(ct)) => {
                match hex::decode(&ct.keys_ct) {
                    Ok(key_ct) => {
                        // TODO: This method needs some attention, because keyring api called `create_service_token` on `ch_claims`
                        match self
                            .key_api
                            .decrypt_key_map(
                                ch_claims,
                                hex::encode_upper(key_ct),
                                Some(pid),
                                ct.dt_id.clone(),
                            )
                            .await
                        {
                            Ok(key_map) => {
                                //TODO check the hash
                                match ct.decrypt(key_map.keys) {
                                    Ok(d) => Ok(d),
                                    Err(e) => {
                                        warn!("Got empty document from decryption! {:?}", e);
                                        Err(DocumentServiceError::NotFound)
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Error while retrieving keys from keyring: {:?}", e);
                                Err(DocumentServiceError::KeyringServiceError(e))
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error while decoding ciphertext: {:?}", e);
                        Err(DocumentServiceError::CorruptedCiphertext(e)) // InternalError
                    }
                }
            }
            Ok(None) => {
                debug!("Nothing found in db!");
                Err(DocumentServiceError::NotFound) // NotFound
            }
            Err(e) => {
                error!("Error while retrieving document: {:?}", e);
                Err(DocumentServiceError::DatabaseError {
                    source: e,
                    description: "Error while retrieving document".to_string(),
                })
            }
        }
    }

    #[inline]
    fn sanitize_page(page: Option<u64>) -> u64 {
        // Parameter validation for pagination:
        // Valid pages start from 1
        match page {
            Some(p) => {
                if p > 0 {
                    p
                } else {
                    warn!("...invalid page requested. Falling back to 1.");
                    1
                }
            }
            None => 1,
        }
    }

    #[inline]
    fn sanitize_size(size: Option<u64>) -> u64 {
        // Valid sizes are between 1 and MAX_NUM_RESPONSE_ENTRIES (1000)
        match size {
            Some(s) => {
                if s > 0 && s <= MAX_NUM_RESPONSE_ENTRIES {
                    s
                } else {
                    warn!("...invalid size requested. Falling back to default.");
                    DEFAULT_NUM_RESPONSE_ENTRIES
                }
            }
            None => DEFAULT_NUM_RESPONSE_ENTRIES,
        }
    }
}
