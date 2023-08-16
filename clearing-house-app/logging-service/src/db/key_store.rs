use std::process::exit;
use core_lib::errors::*;
use mongodb::bson::doc;
use mongodb::Client;
use rocket::futures::TryStreamExt;
use core_lib::constants::{KEYRING_DB, MONGO_COLL_DOC_TYPES, MONGO_COLL_MASTER_KEY, MONGO_ID, MONGO_PID};
use core_lib::db::DataStoreApi;
use crate::model::crypto::MasterKey;
use crate::model::doc_type::DocumentType;

#[derive(Clone, Debug)]
pub struct KeyStore {
    pub(crate) client: mongodb::Client,
    database: mongodb::Database
}

impl DataStoreApi for KeyStore {
    fn new(client: Client) -> KeyStore{
        KeyStore {
            client: client.clone(),
            database: client.database(KEYRING_DB)
        }
    }
}

impl KeyStore {

    /// Only one master key may exist in the database.
    pub async fn store_master_key(&self, key: MasterKey) -> anyhow::Result<bool>{
        tracing::debug!("Storing new master key...");
        let coll = self.database.collection::<MasterKey>(MONGO_COLL_MASTER_KEY);
        tracing::debug!("... but first check if there's already one.");
        let result= coll.find(None, None).await
            .expect("Error retrieving the master keys")
            .try_collect().await.unwrap_or_else(|_| vec![]);

        if result.len() > 1{
            tracing::error!("Master Key table corrupted!");
            exit(1);
        }
        if result.len() == 1{
            tracing::error!("Master key already exists!");
            Ok(false)
        }
        else{
            //let db_key = bson::to_bson(&key)
            //    .expect("failed to serialize master key for database");
            match coll.insert_one(key, None).await{
                Ok(_r) => {
                    Ok(true)
                },
                Err(e) => {
                    tracing::error!("master key could not be stored: {:?}", &e);
                    panic!("master key could not be stored")
                }
            }
        }
    }

    /// Only one master key may exist in the database.
    pub async fn get_msk(&self) -> anyhow::Result<MasterKey> {
        let coll = self.database.collection::<MasterKey>(MONGO_COLL_MASTER_KEY);
        let result= coll.find(None, None).await
            .expect("Error retrieving the master keys")
            .try_collect().await.unwrap_or_else(|_| vec![]);

        if result.len() > 1{
            tracing::error!("Master Key table corrupted!");
            exit(1);
        }
        if result.len() == 1{
            Ok(result[0].clone())
        }
        else {
            tracing::error!("Master Key missing!");
            exit(1);
        }
    }

    // DOCTYPE
    pub async fn add_document_type(&self, doc_type: DocumentType) -> Result<()> {
        let coll = self.database.collection::<DocumentType>(MONGO_COLL_DOC_TYPES);
        match coll.insert_one(doc_type.clone(), None).await {
            Ok(_r) => {
                tracing::debug!("added new document type: {}", &_r.inserted_id);
                Ok(())
            },
            Err(e) => {
                tracing::error!("failed to log document type {}", &doc_type.id);
                Err(Error::from(e))
            }
        }
    }

    //TODO: Do we need to check that no documents of this type exist before we remove it from the db?
    pub async fn delete_document_type(&self, id: &String, pid: &String) -> Result<bool> {
        let coll = self.database.collection::<DocumentType>(MONGO_COLL_DOC_TYPES);
        let result = coll.delete_many(doc! { MONGO_ID: id, MONGO_PID: pid }, None).await?;
        if result.deleted_count >= 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }


    /// checks if the model exits
    pub async fn exists_document_type(&self, pid: &String, dt_id: &String) -> Result<bool> {
        let coll = self.database.collection::<DocumentType>(MONGO_COLL_DOC_TYPES);
        let result = coll.find_one(Some(doc! { MONGO_ID: dt_id, MONGO_PID: pid }), None).await?;
        match result {
            Some(_r) => Ok(true),
            None => {
                tracing::debug!("document type with id {} and pid {:?} does not exist!", &dt_id, &pid);
                Ok(false)
            }
        }
    }

    pub async fn get_all_document_types(&self) -> Result<Vec<DocumentType>> {
        let coll = self.database.collection::<DocumentType>(MONGO_COLL_DOC_TYPES);
        let result = coll.find(None, None).await?
            .try_collect().await.unwrap_or_else(|_| vec![]);
        Ok(result)
    }

    pub async fn get_document_type(&self, dt_id: &String) -> Result<Option<DocumentType>> {
        let coll = self.database.collection::<DocumentType>(MONGO_COLL_DOC_TYPES);
        tracing::debug!("get_document_type for dt_id: '{}'", dt_id);
        match coll.find_one(Some(doc! { MONGO_ID: dt_id}), None).await{
            Ok(result) => Ok(result),
            Err(e) => {
                tracing::error!("error while getting document type with id {}!", dt_id);
                Err(Error::from(e))
            }
        }
    }

    pub async fn update_document_type(&self, doc_type: DocumentType, id: &String) -> Result<bool> {
        let coll = self.database.collection::<DocumentType>(MONGO_COLL_DOC_TYPES);
        match coll.replace_one(doc! { MONGO_ID: id}, doc_type, None).await{
            Ok(r) => {
                if r.matched_count != 1 || r.modified_count != 1{
                    tracing::warn!("while replacing doc type {} matched '{}' dts and modified '{}'", id, r.matched_count, r.modified_count);
                }
                else{
                    tracing::debug!("while replacing doc type {} matched '{}' dts and modified '{}'", id, r.matched_count, r.modified_count);
                }
                Ok(true)
            },
            Err(e) => {
                tracing::error!("error while updating document type with id {}: {:#?}", id, e);
                Ok(false)
            }
        }
    }
}