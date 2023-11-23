use crate::crypto;
use crate::crypto::restore_key_map;
use crate::db::key_store::KeyStore;
use crate::model::claims::ChClaims;
use crate::model::crypto::{KeyCtList, KeyMap, KeyMapListItem};

#[derive(Debug, thiserror::Error)]
pub enum KeyringServiceError {
    #[error("Keymap generation error")]
    KeymapGenerationFailed,
    #[error("Keymap restoration error")]
    KeymapRestorationFailed,
    #[error("Document type not found")]
    DocumentTypeNotFound,
    #[error("Error during database operation: {description}: {source}")]
    DatabaseError {
        source: anyhow::Error,
        description: String,
    },
    #[error("Error while decrypting keys")]
    DecryptionError,
    #[cfg_attr(not(doc_type), allow(dead_code))]
    #[error("Document type already exists")]
    DocumentTypeAlreadyExists,
}

impl axum::response::IntoResponse for KeyringServiceError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;
        match self {
            Self::KeymapGenerationFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Self::KeymapRestorationFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Self::DocumentTypeNotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::DatabaseError {
                source,
                description,
            } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{}: {}", description, source),
            )
                .into_response(),
            Self::DecryptionError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Self::DocumentTypeAlreadyExists => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeyringService {
    db: KeyStore,
}

impl KeyringService {
    pub fn new(db: KeyStore) -> KeyringService {
        KeyringService { db }
    }

    #[tracing::instrument(skip_all)]
    pub async fn generate_keys(
        &self,
        ch_claims: ChClaims,
        _pid: String,
        dt_id: String,
    ) -> Result<KeyMap, KeyringServiceError> {
        trace!("generate_keys");
        trace!("...user '{:?}'", &ch_claims.client_id);
        match self.db.get_msk().await {
            Ok(key) => {
                // check that doc type exists for pid
                match self.db.get_document_type(&dt_id).await {
                    Ok(Some(dt)) => {
                        // generate new random key map
                        match crypto::generate_key_map(key, dt) {
                            Ok(key_map) => {
                                trace!("response: {:?}", &key_map);
                                Ok(key_map)
                            }
                            Err(e) => {
                                error!("Error while generating key map: {}", e);
                                Err(KeyringServiceError::KeymapGenerationFailed)
                            }
                        }
                    }
                    Ok(None) => {
                        warn!("document type {} not found", &dt_id);
                        Err(KeyringServiceError::DocumentTypeNotFound)
                    }
                    Err(e) => {
                        warn!("Error while retrieving document type: {}", e);
                        Err(KeyringServiceError::DatabaseError {
                            source: e,
                            description: "Error while retrieving document type".to_string(),
                        })
                    }
                }
            }
            Err(e) => {
                error!("Error while retrieving master key: {}", e);
                Err(KeyringServiceError::DatabaseError {
                    source: e,
                    description: "Error while retrieving master key".to_string(),
                })
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub(crate) async fn decrypt_keys(
        &self,
        ch_claims: ChClaims,
        _pid: Option<String>,
        key_cts: &KeyCtList,
    ) -> Result<Vec<KeyMapListItem>, KeyringServiceError> {
        trace!("decrypt_keys");
        trace!("...user '{:?}'", &ch_claims.client_id);
        debug!("number of cts to decrypt: {}", &key_cts.cts.len());

        // get master key
        match self.db.get_msk().await {
            Ok(m_key) => {
                // check that doc type exists for pid
                match self.db.get_document_type(&key_cts.dt).await {
                    Ok(Some(dt)) => {
                        let mut dec_error_count = 0;
                        let mut map_error_count = 0;
                        // validate keys_ct input
                        let key_maps: Vec<KeyMapListItem> = key_cts
                            .cts
                            .iter()
                            .filter_map(|key_ct| match hex::decode(key_ct.ct.clone()) {
                                Ok(key) => Some((key_ct.id.clone(), key)),
                                Err(e) => {
                                    error!("Error while decoding key ciphertext: {}", e);
                                    dec_error_count += 1;
                                    None
                                }
                            })
                            .filter_map(|(id, key)| {
                                match restore_key_map(m_key.clone(), dt.clone(), key) {
                                    Ok(key_map) => Some(KeyMapListItem::new(id, key_map)),
                                    Err(e) => {
                                        error!("Error while generating key map: {}", e);
                                        map_error_count += 1;
                                        None
                                    }
                                }
                            })
                            .collect();

                        let error_count = map_error_count + dec_error_count;

                        // Currently, we don't tolerate errors while decrypting keys
                        if error_count > 0 {
                            Err(KeyringServiceError::DecryptionError)
                        } else {
                            Ok(key_maps)
                        }
                    }
                    Ok(None) => {
                        warn!("document type {} not found", &key_cts.dt);
                        Err(KeyringServiceError::DocumentTypeNotFound)
                    }
                    Err(e) => {
                        warn!("Error while retrieving document type: {}", e);
                        Err(KeyringServiceError::DatabaseError {
                            source: e,
                            description: "Error while retrieving document type".to_string(),
                        })
                    }
                }
            }
            Err(e) => {
                error!("Error while retrieving master key: {}", e);
                Err(KeyringServiceError::DecryptionError)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn decrypt_key_map(
        &self,
        ch_claims: ChClaims,
        keys_ct: String,
        _pid: Option<String>,
        dt_id: String,
    ) -> Result<KeyMap, KeyringServiceError> {
        trace!("decrypt_key_map");
        trace!("...user '{:?}'", &ch_claims.client_id);
        trace!("ct: {}", &keys_ct);
        // get master key
        match self.db.get_msk().await {
            Ok(key) => {
                // check that doc type exists for pid
                match self.db.get_document_type(&dt_id).await {
                    Ok(Some(dt)) => {
                        // validate keys_ct input
                        let keys_ct = hex::decode(keys_ct).map_err(|e| {
                            error!("Error while decoding key ciphertext: {}", e);
                            KeyringServiceError::DecryptionError
                        })?;

                        match restore_key_map(key, dt, keys_ct) {
                            Ok(key_map) => Ok(key_map),
                            Err(e) => {
                                error!("Error while generating key map: {}", e);
                                Err(KeyringServiceError::KeymapRestorationFailed)
                            }
                        }
                    }
                    Ok(None) => {
                        warn!("document type {} not found", &dt_id);
                        Err(KeyringServiceError::DocumentTypeNotFound)
                    }
                    Err(e) => {
                        warn!("Error while retrieving document type: {}", e);
                        Err(KeyringServiceError::DatabaseError {
                            source: e,
                            description: "Error while retrieving document type".to_string(),
                        })
                    }
                }
            }
            Err(e) => {
                error!("Error while retrieving master key: {}", e);
                Err(KeyringServiceError::DecryptionError)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub(crate) async fn decrypt_multiple_keys(
        &self,
        ch_claims: ChClaims,
        pid: Option<String>,
        cts: &KeyCtList,
    ) -> Result<Vec<KeyMapListItem>, KeyringServiceError> {
        self.decrypt_keys(ch_claims, pid, cts).await
    }

    #[cfg(doc_type)]
    pub(crate) async fn create_doc_type(
        &self,
        doc_type: crate::model::doc_type::DocumentType,
    ) -> Result<crate::model::doc_type::DocumentType, KeyringServiceError> {
        debug!("adding doctype: {:?}", &doc_type);
        match self
            .db
            .exists_document_type(&doc_type.pid, &doc_type.id)
            .await
        {
            Ok(true) => Err(KeyringServiceError::DocumentTypeAlreadyExists), // BadRequest
            Ok(false) => match self.db.add_document_type(doc_type.clone()).await {
                Ok(()) => Ok(doc_type),
                Err(e) => {
                    error!("Error while adding doctype: {:?}", e);
                    Err(KeyringServiceError::DatabaseError {
                        source: e,
                        description: "Error while adding doctype".to_string(),
                    })
                }
            },
            Err(e) => {
                error!("Error while adding document type: {:?}", e);
                Err(KeyringServiceError::DatabaseError {
                    source: e,
                    description: "Error while checking doctype".to_string(),
                })
            }
        }
    }

    #[cfg(doc_type)]
    pub(crate) async fn update_doc_type(
        &self,
        id: String,
        doc_type: crate::model::doc_type::DocumentType,
    ) -> Result<bool, KeyringServiceError> {
        match self
            .db
            .exists_document_type(&doc_type.pid, &doc_type.id)
            .await
        {
            Ok(true) => Err(KeyringServiceError::DocumentTypeAlreadyExists),
            Ok(false) => match self.db.update_document_type(doc_type, &id).await {
                Ok(id) => Ok(id),
                Err(e) => {
                    error!("Error while adding doctype: {:?}", e);
                    Err(KeyringServiceError::DatabaseError {
                        source: e,
                        description: "Error while storing document type!".to_string(),
                    })
                }
            },
            Err(e) => {
                error!("Error while adding document type: {:?}", e);
                Err(KeyringServiceError::DatabaseError {
                    source: e,
                    description: "Error while checking doctype".to_string(),
                })
            }
        }
    }

    #[cfg(doc_type)]
    pub(crate) async fn delete_doc_type(
        &self,
        id: String,
        pid: String,
    ) -> Result<String, KeyringServiceError> {
        match self.db.delete_document_type(&id, &pid).await {
            Ok(true) => Ok(String::from("Document type deleted!")), // NoContent
            Ok(false) => Err(KeyringServiceError::DocumentTypeNotFound),
            Err(e) => {
                error!("Error while deleting doctype: {:?}", e);
                Err(KeyringServiceError::DatabaseError {
                    source: e,
                    description: format!("Error while deleting document type with id {}!", id),
                })
            }
        }
    }

    #[cfg(doc_type)]
    pub(crate) async fn get_doc_type(
        &self,
        id: String,
        pid: String,
    ) -> Result<Option<crate::model::doc_type::DocumentType>, KeyringServiceError> {
        match self.db.get_document_type(&id).await {
            //TODO: would like to send "{}" instead of "null" when dt is not found
            Ok(dt) => Ok(dt),
            Err(e) => {
                error!("Error while retrieving doctype: {:?}", e);
                Err(KeyringServiceError::DatabaseError {
                    source: e,
                    description: format!(
                        "Error while retrieving document type with id {} and pid {}!",
                        id, pid
                    ),
                })
            }
        }
    }

    #[cfg(doc_type)]
    pub(crate) async fn get_doc_types(
        &self,
    ) -> Result<Vec<crate::model::doc_type::DocumentType>, KeyringServiceError> {
        match self.db.get_all_document_types().await {
            //TODO: would like to send "{}" instead of "null" when dt is not found
            Ok(dt) => Ok(dt),
            Err(e) => {
                error!("Error while retrieving default doc_types: {:?}", e);
                Err(KeyringServiceError::DatabaseError {
                    source: e,
                    description: "Error while retrieving all document types".to_string(),
                })
            }
        }
    }
}
