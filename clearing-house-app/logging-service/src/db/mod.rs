pub(crate) mod key_store;
pub(crate) mod doc_store;
pub(crate) mod config;

use core_lib::constants::{MONGO_ID, MONGO_COLL_PROCESSES, PROCESS_DB, MONGO_COLL_TRANSACTIONS, MONGO_TC};
use core_lib::db::DataStoreApi;
use core_lib::errors::*;
use core_lib::model::process::Process;
use mongodb::bson::doc;
use mongodb::{Client, Database};
use rocket::fairing::Fairing;
use rocket::futures::TryStreamExt;
use mongodb::options::{UpdateModifications, FindOneAndUpdateOptions};
use crate::model::TransactionCounter;

#[derive(Clone)]
pub struct ProcessStore {
    client: Client,
    database: Database
}

impl DataStoreApi for ProcessStore {
    fn new(client: Client) -> ProcessStore{
        ProcessStore {
            client: client.clone(),
            database: client.database(PROCESS_DB)
        }
    }
}

impl ProcessStore {
    pub async fn get_transaction_counter(&self) -> Result<Option<i64>>{
        debug!("Getting transaction counter...");
        let coll = self.database.collection::<TransactionCounter>(MONGO_COLL_TRANSACTIONS);
        match coll.find_one(None, None).await?{
            Some(t) => Ok(Some(t.tc)),
            None => Ok(Some(0))
        }
    }

    pub async fn increment_transaction_counter(&self) -> Result<Option<i64>>{
        debug!("Getting transaction counter...");
        let coll = self.database.collection::<TransactionCounter>(MONGO_COLL_TRANSACTIONS);
        let mods = UpdateModifications::Document(doc!{"$inc": {MONGO_TC: 1 }});
        let mut opts = FindOneAndUpdateOptions::default();
        opts.upsert = Some(true);
        match coll.find_one_and_update(doc!{}, mods, opts).await?{
            Some(t) => Ok(Some(t.tc)),
            None => Ok(Some(0))
        }
    }

    pub async fn get_processes(&self) -> Result<Vec<Process>> {
        debug!("Trying to get all processes...");
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        let result = coll.find(None, None).await?
            .try_collect().await.unwrap_or_else(|_| vec![]);
        Ok(result)
    }

    pub async fn delete_process(&self, pid: &String) -> Result<bool> {
        debug!("Trying to delete process with pid '{}'...", pid);
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        let result = coll.delete_one(doc! { MONGO_ID: pid }, None).await?;
        if result.deleted_count == 1{
            debug!("... deleted one process.");
            Ok(true)
        }
        else{
            warn!("deleted_count={}", result.deleted_count);
            Ok(false)
        }
    }

    /// checks if the id exits
    pub async fn exists_process(&self, pid: &String) -> Result<bool> {
        debug!("Check if process with pid '{}' exists...", pid);
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        let result = coll.find_one(Some(doc! { MONGO_ID: pid }), None).await?;
        match result {
            Some(_r) => {
                debug!("... found.");
                Ok(true)
            },
            None => {
                debug!("Process with pid '{}' does not exist!", pid);
                Ok(false)
            }
        }
    }

    pub async fn get_process(&self, pid: &String) -> Result<Option<Process>> {
        debug!("Trying to get process with id {}...", pid);
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        match coll.find_one(Some(doc!{ MONGO_ID: pid }), None).await{
            Ok(process) => {
                Ok(process)
            },
            Err(e) => {
                error!("Error while getting process: {:#?}!", &e);
                Err(Error::from(e))
            }
        }
    }

    pub async fn is_authorized(&self, user: &String, pid: &String) -> Result<bool>{
        debug!("checking if user '{}' is authorized to access '{}'", user, pid);
        return match self.get_process(&pid).await{
            Ok(Some(process)) => {
                let authorized = process.owners.iter().any(|o| {
                    trace!("found owner {}", o);
                    user.eq(o)
                });
                Ok(authorized)
            }
            Ok(None) => {
                trace!("didn't find process");
                Ok(false)
            },
            _ => {
                Err(format!("User '{}' could not be authorized", &user).into())
            }
        }
    }

    // store process in db
    pub async fn store_process(&self, process: Process) -> Result<bool> {
        debug!("Storing process with pid {:#?}...", &process.id);
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        match coll.insert_one(process, None).await {
            Ok(_r) => {
                debug!("...added new process: {}", &_r.inserted_id);
                Ok(true)
            },
            Err(e) => {
                error!("...failed to store process: {:#?}", &e);
                Err(Error::from(e))
            }
        }
    }
}