use crate::db::doc_store::DataStore;
use crate::model::claims::ChClaims;
use crate::model::constants::{
    DEFAULT_DOC_TYPE, DEFAULT_NUM_RESPONSE_ENTRIES, MAX_NUM_RESPONSE_ENTRIES, PAYLOAD_PART,
};
use crate::model::crypto::{KeyCt, KeyCtList};
use crate::model::document::Document;
use crate::model::{
    parse_date, validate_and_sanitize_dates, SortingOrder,
};
use crate::services::keyring_service::KeyringService;
use crate::services::{DocumentReceipt, QueryResult};
use anyhow::anyhow;
use std::convert::TryFrom;

#[derive(Clone)]
pub struct DocumentService {
    db: DataStore,
    key_api: KeyringService,
}

impl DocumentService {
    pub fn new(db: DataStore, key_api: KeyringService) -> Self {
        Self { db, key_api }
    }

    pub(crate) async fn create_enc_document(
        &self,
        ch_claims: ChClaims,
        doc: Document,
    ) -> anyhow::Result<DocumentReceipt> {
        trace!("...user '{:?}'", &ch_claims.client_id);
        // data validation
        let payload: Vec<String> = doc
            .parts
            .iter()
            .filter(|p| *PAYLOAD_PART == p.name)
            .map(|p| p.content.as_ref().unwrap().clone())
            .collect();
        if payload.len() > 1 {
            return Err(anyhow!("Document contains two or more payloads!")); // BadRequest
        } else if payload.is_empty() {
            return Err(anyhow!("Document contains no payload!")); // BadRequest
        }

        // check if doc id already exists
        match self.db.exists_document(&doc.id).await {
            Ok(true) => {
                warn!("Document exists already!");
                Err(anyhow!("Document exists already!")) // BadRequest
            }
            _ => {
                debug!("Document does not exists!");
                debug!("getting keys");

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
                        Err(anyhow!("Error while retrieving keys!")) // InternalError
                    }
                }?;

                debug!("start encryption");
                let mut enc_doc = match doc.encrypt(keys) {
                    Ok(ct) => {
                        debug!("got ct");
                        Ok(ct)
                    }
                    Err(e) => {
                        error!("Error while encrypting: {:?}", e);
                        Err(anyhow!("Error while encrypting!")) // InternalError
                    }
                }?;

                // chain the document to previous documents
                debug!("add the chain hash...");
                // get the document with the previous tc
                match self.db.get_document_with_previous_tc(doc.tc).await {
                    Ok(Some(previous_doc)) => {
                        enc_doc.hash = previous_doc.hash();
                    }
                    Ok(None) => {
                        if doc.tc == 0 {
                            info!("No entries found for pid {}. Beginning new chain!", doc.pid);
                        } else {
                            // If this happens, db didn't find a tc entry that should exist.
                            return Err(anyhow!("Error while creating the chain hash!"));
                            // InternalError
                        }
                    }
                    Err(e) => {
                        error!("Error while creating the chain hash: {:?}", e);
                        return Err(anyhow!("Error while creating the chain hash!"));
                    }
                }

                // prepare the success result message

                let receipt =
                    DocumentReceipt::new(enc_doc.ts, &enc_doc.pid, &enc_doc.id, &enc_doc.hash);

                debug!("storing document ....");
                // store document
                match self.db.add_document(enc_doc).await {
                    Ok(_b) => Ok(receipt),
                    Err(e) => {
                        error!("Error while adding: {:?}", e);
                        Err(anyhow!("Error while storing document!"))
                    }
                }
            }
        }
    }

    pub(crate) async fn get_enc_documents_for_pid(
        &self,
        ch_claims: ChClaims,
        doc_type: Option<String>,
        page: Option<i32>, // TODO: Why i32? This should be and unsigned int
        size: Option<i32>, // TODO: Why i32? This should be and unsigned int
        sort: Option<SortingOrder>,
        date_from: Option<String>,
        date_to: Option<String>,
        pid: String,
    ) -> anyhow::Result<QueryResult> {
        debug!("Trying to retrieve documents for pid '{}'...", &pid);
        trace!("...user '{:?}'", &ch_claims.client_id);
        debug!(
            "...page: {:#?}, size:{:#?} and sort:{:#?}",
            page, size, sort
        );

        // Parameter validation for pagination:
        // Valid pages start from 1
        // Max page number as of yet unknown
        let sanitized_page = match page {
            Some(p) => {
                if p > 0 {
                    u64::try_from(p).unwrap()
                } else {
                    warn!("...invalid page requested. Falling back to 1.");
                    1
                }
            }
            None => 1,
        };

        // Valid sizes are between 1 and MAX_NUM_RESPONSE_ENTRIES (1000)
        let sanitized_size = match size {
            Some(s) => {
                if s > 0 && s <= i32::try_from(MAX_NUM_RESPONSE_ENTRIES).unwrap() {
                    u64::try_from(s).unwrap()
                } else {
                    warn!("...invalid size requested. Falling back to default.");
                    DEFAULT_NUM_RESPONSE_ENTRIES
                }
            }
            None => DEFAULT_NUM_RESPONSE_ENTRIES,
        };

        // Sorting order is already validated and defaults to descending
        let sanitized_sort = sort.unwrap_or(SortingOrder::Descending);

        // Parsing the dates for duration queries
        let parsed_date_from = parse_date(date_from, false);
        let parsed_date_to = parse_date(date_to, true);

        // Validation of dates with various checks. If none given dates default to date_now (date_to) and (date_now - 2 weeks) (date_from)
        let Ok((sanitized_date_from, sanitized_date_to)) = validate_and_sanitize_dates(parsed_date_from, parsed_date_to, None) else {
            debug!("date validation failed!");
            return Err(anyhow!("Invalid date parameter!")); // BadRequest
        };

        //new behavior: if pages are "invalid" return {}. Do not adjust page
        //either call db with type filter or without to get cts
        let start = chrono::Local::now();
        debug!(
            "... using pagination with page: {}, size:{} and sort:{:#?}",
            sanitized_page, sanitized_size, &sanitized_sort
        );

        let dt_id = match doc_type {
            Some(dt) => dt,
            None => String::from(DEFAULT_DOC_TYPE),
        };
        let cts = match self
            .db
            .get_documents_for_pid(
                &dt_id,
                &pid,
                sanitized_page,
                sanitized_size,
                &sanitized_sort,
                &sanitized_date_from,
                &sanitized_date_to,
            )
            .await
        {
            Ok(cts) => cts,
            Err(e) => {
                error!("Error while retrieving document: {:?}", e);
                return Err(anyhow!("Error while retrieving document for {}", &pid));
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
                    return Err(anyhow!("Error while retrieving keys from keyring"));
                    // InternalError
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
            let end = chrono::Local::now();
            let diff = end - start;
            info!("Total time taken to run in ms: {}", diff.num_milliseconds());
            result.documents = pts_bulk;
            Ok(result)
        }
    }

    pub(crate) async fn get_enc_document(
        &self,
        ch_claims: ChClaims,
        pid: String,
        id: String,
        hash: Option<String>,
    ) -> anyhow::Result<Document> {
        trace!("...user '{:?}'", &ch_claims.client_id);
        trace!(
            "trying to retrieve document with id '{}' for pid '{}'",
            &id,
            &pid
        );
        if hash.is_some() {
            debug!("integrity check with hash: {}", hash.as_ref().unwrap());
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
                                        Err(anyhow!("Document {} not found!", &id))
                                        // NotFound
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Error while retrieving keys from keyring: {:?}", e);
                                Err(anyhow!("Error while retrieving keys"))
                                // InternalError
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error while decoding ciphertext: {:?}", e);
                        Err(anyhow!("Key Ciphertext corrupted")) // InternalError
                    }
                }
            }
            Ok(None) => {
                debug!("Nothing found in db!");
                Err(anyhow!("Document {} not found!", &id)) // NotFound
            }
            Err(e) => {
                error!("Error while retrieving document: {:?}", e);
                Err(anyhow!("Error while retrieving document {}", &id)) // InternalError
            }
        }
    }
}
