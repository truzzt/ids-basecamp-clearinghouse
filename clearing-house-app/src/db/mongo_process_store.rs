use crate::db::init_database_client;
use crate::model::constants::{
    MONGO_COLL_PROCESSES, MONGO_COLL_TRANSACTIONS, MONGO_ID, PROCESS_DB, PROCESS_DB_CLIENT,
};
use crate::model::process::Process;
use anyhow::anyhow;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::{CreateCollectionOptions, WriteConcern};
use mongodb::{Client, Database};

#[derive(Clone, Debug)]
pub struct MongoProcessStore {
    database: Database,
}

impl MongoProcessStore {
    fn new(client: Client) -> MongoProcessStore {
        MongoProcessStore {
            database: client.database(PROCESS_DB),
        }
    }

    pub async fn init_process_store(db_url: &str, clear_db: bool) -> anyhow::Result<Self> {
        debug!("...using database url: '{:#?}'", &db_url);

        match init_database_client(db_url, Some(PROCESS_DB_CLIENT.to_string())).await {
            Ok(process_store) => {
                debug!("...check if database is empty...");
                match process_store
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
                            match process_store.database(PROCESS_DB).drop(None).await {
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
                        Ok(Self::new(process_store))
                    }
                    Err(_) => Err(anyhow!("Failed to list collections")),
                }
            }
            Err(_) => Err(anyhow!("Failed to initialize database client")),
        }
    }
}

impl super::ProcessStore for MongoProcessStore {
    async fn get_processes(&self) -> anyhow::Result<Vec<Process>> {
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

    async fn delete_process(&self, pid: &str) -> anyhow::Result<bool> {
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
    #[tracing::instrument(skip_all)]
    async fn exists_process(&self, pid: &str) -> anyhow::Result<bool> {
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

    #[tracing::instrument(skip_all)]
    async fn get_process(&self, pid: &str) -> anyhow::Result<Option<Process>> {
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

    /// store process in db
    #[tracing::instrument(skip_all)]
    async fn store_process(&self, process: Process) -> anyhow::Result<()> {
        debug!("Storing process with pid {:#?}...", &process.id);
        let coll = self.database.collection::<Process>(MONGO_COLL_PROCESSES);
        match coll.insert_one(process, None).await {
            Ok(_r) => {
                debug!("...added new process: {}", &_r.inserted_id);
                Ok(())
            }
            Err(e) => {
                error!("...failed to store process: {:#?}", &e);
                Err(e.into())
            }
        }
    }
}
