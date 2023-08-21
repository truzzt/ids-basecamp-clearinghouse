use crate::crypto;
use crate::crypto::restore_key_map;
use crate::db::key_store::KeyStore;
use crate::model::claims::ChClaims;
use crate::model::crypto::{KeyCtList, KeyMap, KeyMapListItem};
use crate::model::doc_type::DocumentType;
use anyhow::anyhow;

#[derive(Clone)]
pub struct KeyringService {
    db: KeyStore,
}

impl KeyringService {
    pub fn new(db: KeyStore) -> KeyringService {
        KeyringService { db }
    }

    pub async fn generate_keys(
        &self,
        ch_claims: ChClaims,
        _pid: String,
        dt_id: String,
    ) -> anyhow::Result<KeyMap> {
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
                                return Ok(key_map);
                            }
                            Err(e) => {
                                error!("Error while generating key map: {}", e);
                                return Err(anyhow!("Error while generating keys"));
                                // InternalError
                            }
                        }
                    }
                    Ok(None) => {
                        warn!("document type {} not found", &dt_id);
                        return Err(anyhow!("Document type not found!")); // BadRequest
                    }
                    Err(e) => {
                        warn!("Error while retrieving document type: {}", e);
                        return Err(anyhow!("Error while retrieving document type"));
                        // InternalError
                    }
                }
            }
            Err(e) => {
                error!("Error while retrieving master key: {}", e);
                return Err(anyhow!("Error while generating keys")); // InternalError
            }
        }
    }

    pub(crate) async fn decrypt_keys(
        &self,
        ch_claims: ChClaims,
        _pid: Option<String>,
        key_cts: &KeyCtList,
    ) -> anyhow::Result<Vec<KeyMapListItem>> {
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
                                    dec_error_count = dec_error_count + 1;
                                    None
                                }
                            })
                            .filter_map(|(id, key)| {
                                match restore_key_map(m_key.clone(), dt.clone(), key) {
                                    Ok(key_map) => Some(KeyMapListItem::new(id, key_map)),
                                    Err(e) => {
                                        error!("Error while generating key map: {}", e);
                                        map_error_count = map_error_count + 1;
                                        None
                                    }
                                }
                            })
                            .collect();

                        let error_count = map_error_count + dec_error_count;

                        // Currently, we don't tolerate errors while decrypting keys
                        if error_count > 0 {
                            return Err(anyhow!("Error while decrypting keys")); // InternalError
                        } else {
                            return Ok(key_maps);
                        }
                    }
                    Ok(None) => {
                        warn!("document type {} not found", &key_cts.dt);
                        return Err(anyhow!("Document type not found!")); // BadRequest
                    }
                    Err(e) => {
                        warn!("Error while retrieving document type: {}", e);
                        return Err(anyhow!("Document type not found!")); // NotFound
                    }
                }
            }
            Err(e) => {
                error!("Error while retrieving master key: {}", e);
                return Err(anyhow!("Error while decrypting keys")); // InternalError
            }
        }
    }

    pub async fn decrypt_key_map(
        &self,
        ch_claims: ChClaims,
        keys_ct: String,
        _pid: Option<String>,
        dt_id: String,
    ) -> anyhow::Result<KeyMap> {
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
                        let keys_ct = match hex::decode(keys_ct) {
                            Ok(key) => key,
                            Err(e) => {
                                error!("Error while decoding key ciphertext: {}", e);
                                return Err(anyhow!("Error while decrypting keys"));
                                // InternalError
                            }
                        };

                        match restore_key_map(key, dt, keys_ct) {
                            Ok(key_map) => {
                                return Ok(key_map);
                            }
                            Err(e) => {
                                error!("Error while generating key map: {}", e);
                                return Err(anyhow!("Error while restoring keys"));
                                // InternalError
                            }
                        }
                    }
                    Ok(None) => {
                        warn!("document type {} not found", &dt_id);
                        return Err(anyhow!("Document type not found!")); // BadRequest
                    }
                    Err(e) => {
                        warn!("Error while retrieving document type: {}", e);
                        return Err(anyhow!("Document type not found!")); // NotFound
                    }
                }
            }
            Err(e) => {
                error!("Error while retrieving master key: {}", e);
                return Err(anyhow!("Error while decrypting keys")); // InternalError
            }
        }
    }

    pub(crate) async fn decrypt_multiple_keys(
        &self,
        ch_claims: ChClaims,
        pid: Option<String>,
        cts: &KeyCtList,
    ) -> anyhow::Result<Vec<KeyMapListItem>> {
        self.decrypt_keys(ch_claims, pid, cts).await
    }

    pub(crate) async fn create_doc_type(
        &self,
        doc_type: DocumentType,
    ) -> anyhow::Result<DocumentType> {
        debug!("adding doctype: {:?}", &doc_type);
        match self
            .db
            .exists_document_type(&doc_type.pid, &doc_type.id)
            .await
        {
            Ok(true) => Err(anyhow!("doctype already exists!")), // BadRequest
            Ok(false) => {
                match self.db.add_document_type(doc_type.clone()).await {
                    Ok(()) => Ok(doc_type),
                    Err(e) => {
                        error!("Error while adding doctype: {:?}", e);
                        return Err(anyhow!("Error while adding document type!"));
                        // InternalError
                    }
                }
            }
            Err(e) => {
                error!("Error while adding document type: {:?}", e);
                return Err(anyhow!("Error while checking database!")); // InternalError
            }
        }
    }

    pub(crate) async fn update_doc_type(
        &self,
        id: String,
        doc_type: DocumentType,
    ) -> anyhow::Result<bool> {
        match self
            .db
            .exists_document_type(&doc_type.pid, &doc_type.id)
            .await
        {
            Ok(true) => Err(anyhow!("Doctype already exists!")), // BadRequest
            Ok(false) => {
                match self.db.update_document_type(doc_type, &id).await {
                    Ok(id) => Ok(id),
                    Err(e) => {
                        error!("Error while adding doctype: {:?}", e);
                        return Err(anyhow!("Error while storing document type!"));
                        // InternalError
                    }
                }
            }
            Err(e) => {
                error!("Error while adding document type: {:?}", e);
                return Err(anyhow!("Error while checking database!")); // InternalError
            }
        }
    }

    pub(crate) async fn delete_doc_type(&self, id: String, pid: String) -> anyhow::Result<String> {
        match self.db.delete_document_type(&id, &pid).await {
            Ok(true) => Ok(String::from("Document type deleted!")), // NoContent
            Ok(false) => Err(anyhow!("Document type does not exist!")), // NotFound
            Err(e) => {
                error!("Error while deleting doctype: {:?}", e);
                Err(anyhow!(
                    "Error while deleting document type with id {}!",
                    id
                )) // InternalError
            }
        }
    }

    pub(crate) async fn get_doc_type(
        &self,
        id: String,
        pid: String,
    ) -> anyhow::Result<Option<DocumentType>> {
        match self.db.get_document_type(&id).await {
            //TODO: would like to send "{}" instead of "null" when dt is not found
            Ok(dt) => Ok(dt),
            Err(e) => {
                error!("Error while retrieving doctype: {:?}", e);
                Err(anyhow!(
                    "Error while retrieving document type with id {} and pid {}!",
                    id,
                    pid
                )) // InternalError
            }
        }
    }

    pub(crate) async fn get_doc_types(&self) -> anyhow::Result<Vec<DocumentType>> {
        match self.db.get_all_document_types().await {
            //TODO: would like to send "{}" instead of "null" when dt is not found
            Ok(dt) => Ok(dt),
            Err(e) => {
                error!("Error while retrieving default doctypes: {:?}", e);
                Err(anyhow!("Error while retrieving all document types")) // InternalError
            }
        }
    }
}
