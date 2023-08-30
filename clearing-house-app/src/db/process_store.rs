use anyhow::anyhow;
use crate::db::{DataStoreApi, init_database_client};
use crate::model::constants::{MONGO_COLL_PROCESSES, MONGO_COLL_TRANSACTIONS, MONGO_ID, MONGO_TC, PROCESS_DB, PROCESS_DB_CLIENT};
use crate::model::process::Process;
use crate::model::process::TransactionCounter;
use mongodb::bson::doc;
use mongodb::options::{CreateCollectionOptions, FindOneAndUpdateOptions, UpdateModifications, WriteConcern};
use mongodb::{Client, Database};
use rocket::futures::TryStreamExt;

#[derive(Clone)]
pub struct ProcessStore {
    pub(crate) client: Client,
    database: Database,
}

impl DataStoreApi for ProcessStore {
    fn new(client: Client) -> ProcessStore {
        ProcessStore {
            client: client.clone(),
            database: client.database(PROCESS_DB),
        }
    }
}

impl ProcessStore {
    pub async fn init_process_store(db_url: String, clear_db: bool) -> anyhow::Result<Self> {
        debug!("...using database url: '{:#?}'", &db_url);

        match init_database_client::<ProcessStore>(
            db_url.as_str(),
            Some(PROCESS_DB_CLIENT.to_string()),
        )
            .await
        {
            Ok(process_store) => {
                debug!("...check if database is empty...");
                match process_store
                    .client
                    .database(PROCESS_DB)
                    .list_collection_names(None)
                    .await
                {
                    Ok(colls) => {
                        debug!("... found collections: {:#?}", &colls);
                        if !colls.is_empty() && clear_db {
                            debug!(
                                "...database not empty and clear_db == true. Dropping database..."
                            );
                            match process_store.client.database(PROCESS_DB).drop(None).await {
                                Ok(_) => {
                                    debug!("... done.");
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to drop database"));
                                }
                            };
                        }
                        if colls.is_empty() || clear_db {
                            debug!("..database empty. Need to initialize...");
                            let mut write_concern = WriteConcern::default();
                            write_concern.journal = Some(true);
                            let mut options = CreateCollectionOptions::default();
                            options.write_concern = Some(write_concern);
                            debug!("...create collection {} ...", MONGO_COLL_TRANSACTIONS);
                            match process_store
                                .client
                                .database(PROCESS_DB)
                                .create_collection(MONGO_COLL_TRANSACTIONS, options)
                                .await
                            {
                                Ok(_) => {
                                    debug!("... done.");
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to create collection"));
                                }
                            };
                        }
                        debug!("... database initialized.");
                        Ok(process_store)
                    }
                    Err(_) => Err(anyhow!("Failed to list collections")),
                }
            }
            Err(_) => Err(anyhow!("Failed to initialize database client")),
        }
    }

    pub async fn get_transaction_counter(&self) -> anyhow::Result<Option<i64>> {
        debug!("Getting transaction counter...");
        let coll = self
            .database
            .collection::<TransactionCounter>(MONGO_COLL_TRANSACTIONS);
        match coll.find_one(None, None).await? {
            Some(t) => Ok(Some(t.tc)),
            None => Ok(Some(0)),
        }
    }

    pub async fn increment_transaction_counter(&self) -> anyhow::Result<Option<i64>> {
        debug!("Getting transaction counter...");
        let coll = self
            .database
            .collection::<TransactionCounter>(MONGO_COLL_TRANSACTIONS);
        let mods = UpdateModifications::Document(doc! {"$inc": {MONGO_TC: 1 }});
        let mut opts = FindOneAndUpdateOptions::default();
        opts.upsert = Some(true);
        match coll.find_one_and_update(doc! {}, mods, opts).await? {
            Some(t) => Ok(Some(t.tc)),
            None => Ok(Some(0)),
        }
    }

    pub async fn get_processes(&self) -> anyhow::Result<Vec<Process>> {
        debug!("Trying to get all processes...");
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        let result = coll
            .find(None, None)
            .await?
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![]);
        Ok(result)
    }

    pub async fn delete_process(&self, pid: &String) -> anyhow::Result<bool> {
        debug!("Trying to delete process with pid '{}'...", pid);
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        let result = coll.delete_one(doc! { MONGO_ID: pid }, None).await?;
        if result.deleted_count == 1 {
            debug!("... deleted one process.");
            Ok(true)
        } else {
            warn!("deleted_count={}", result.deleted_count);
            Ok(false)
        }
    }

    /// checks if the id exits
    pub async fn exists_process(&self, pid: &String) -> anyhow::Result<bool> {
        debug!("Check if process with pid '{}' exists...", pid);
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        let result = coll.find_one(Some(doc! { MONGO_ID: pid }), None).await?;
        match result {
            Some(_r) => {
                debug!("... found.");
                Ok(true)
            }
            None => {
                debug!("Process with pid '{}' does not exist!", pid);
                Ok(false)
            }
        }
    }

    pub async fn get_process(&self, pid: &String) -> anyhow::Result<Option<Process>> {
        debug!("Trying to get process with id {}...", pid);
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        match coll.find_one(Some(doc! { MONGO_ID: pid }), None).await {
            Ok(process) => Ok(process),
            Err(e) => {
                error!("Error while getting process: {:#?}!", &e);
                Err(e.into())
            }
        }
    }

    pub async fn is_authorized(&self, user: &String, pid: &String) -> anyhow::Result<bool> {
        debug!(
            "checking if user '{}' is authorized to access '{}'",
            user, pid
        );
        match self.get_process(pid).await {
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
            }
            _ => Err(anyhow!("User '{}' could not be authorized", &user).into()),
        }
    }

    // store process in db
    pub async fn store_process(&self, process: Process) -> anyhow::Result<bool> {
        debug!("Storing process with pid {:#?}...", &process.id);
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        match coll.insert_one(process, None).await {
            Ok(_r) => {
                debug!("...added new process: {}", &_r.inserted_id);
                Ok(true)
            }
            Err(e) => {
                error!("...failed to store process: {:#?}", &e);
                Err(e.into())
            }
        }
    }
}
