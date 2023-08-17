use anyhow::anyhow;
use mongodb::options::{CreateCollectionOptions, WriteConcern};
use rocket::{Build, Rocket};
use rocket::fairing::Kind;
use crate::model::constants::{CLEAR_DB, DATABASE_URL, MONGO_COLL_TRANSACTIONS, PROCESS_DB, PROCESS_DB_CLIENT};
use crate::db::init_database_client;
use crate::db::process_store::ProcessStore;

#[derive(Clone, Debug)]
pub struct ProcessStoreConfigurator;

#[rocket::async_trait]
impl rocket::fairing::Fairing for ProcessStoreConfigurator {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Configuring Process Database",
            kind: Kind::Ignite
        }
    }
    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        debug!("Preparing to initialize database...");
        let db_url: String = rocket.figment().extract_inner(DATABASE_URL).clone().unwrap();
        let clear_db = match rocket.figment().extract_inner(CLEAR_DB){
            Ok(value) => {
                debug!("...clear_db: {} found. ", &value);
                value
            },
            Err(_) => {
                false
            }
        };

        match Self::init_process_store(db_url, clear_db).await {
            Ok(process_store) => {
                debug!("...done.");
                Ok(rocket.manage(process_store))
            },
            Err(_) => Err(rocket)
        }
    }
}

impl ProcessStoreConfigurator {
    pub async fn init_process_store(db_url: String, clear_db: bool) -> anyhow::Result<ProcessStore> {
        debug!("...using database url: '{:#?}'", &db_url);

        match init_database_client::<ProcessStore>(&db_url.as_str(), Some(PROCESS_DB_CLIENT.to_string())).await{
            Ok(process_store) => {
                debug!("...check if database is empty...");
                match process_store.client.database(PROCESS_DB)
                    .list_collection_names(None)
                    .await{
                    Ok(colls) => {
                        debug!("... found collections: {:#?}", &colls);
                        if colls.len() > 0 && clear_db{
                            debug!("...database not empty and clear_db == true. Dropping database...");
                            match process_store.client.database(PROCESS_DB).drop(None).await{
                                Ok(_) => {
                                    debug!("... done.");
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to drop database"));
                                }
                            };
                        }
                        if colls.len() == 0 || clear_db{
                            debug!("..database empty. Need to initialize...");
                            let mut write_concern = WriteConcern::default();
                            write_concern.journal = Some(true);
                            let mut options = CreateCollectionOptions::default();
                            options.write_concern = Some(write_concern);
                            debug!("...create collection {} ...", MONGO_COLL_TRANSACTIONS);
                            match process_store.client.database(PROCESS_DB).create_collection(MONGO_COLL_TRANSACTIONS, options).await{
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
                    Err(_) => {
                        Err(anyhow!("Failed to list collections"))
                    }
                }
            },
            Err(_) => Err(anyhow!("Failed to initialize database client"))
        }
    }

}