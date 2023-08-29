use crate::db::doc_store::DataStore;
use crate::db::init_database_client;
use crate::model::constants::{
    CLEAR_DB, DATABASE_URL, DOCUMENT_DB, DOCUMENT_DB_CLIENT, MONGO_COLL_DOCUMENT_BUCKET,
    MONGO_DOC_ARRAY, MONGO_PID, MONGO_TC, MONGO_TS,
};
use crate::model::document::Document;
use anyhow::anyhow;
use mongodb::bson::doc;
use mongodb::options::{CreateCollectionOptions, IndexOptions, WriteConcern};
use mongodb::IndexModel;
use rocket::{fairing, Build, Rocket};

#[derive(Clone, Debug)]
pub struct DatastoreConfigurator;

#[rocket::async_trait]
impl fairing::Fairing for DatastoreConfigurator {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Configuring Document Database",
            kind: fairing::Kind::Ignite,
        }
    }
    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let db_url: String = rocket
            .figment()
            .extract_inner(DATABASE_URL)
            .clone()
            .unwrap();
        let clear_db = match rocket.figment().extract_inner(CLEAR_DB) {
            Ok(value) => {
                debug!("clear_db: '{}' found.", &value);
                value
            }
            Err(_) => false,
        };

        match Self::init_datastore(db_url, clear_db).await {
            Ok(datastore) => Ok(rocket.manage(datastore)),
            Err(_) => Err(rocket),
        }
    }
}

impl DatastoreConfigurator {
    pub async fn init_datastore(db_url: String, clear_db: bool) -> anyhow::Result<DataStore> {
        debug!("Using mongodb url: '{:#?}'", &db_url);
        match init_database_client::<DataStore>(
            db_url.as_str(),
            Some(DOCUMENT_DB_CLIENT.to_string()),
        )
        .await
        {
            Ok(datastore) => {
                debug!("Check if database is empty...");
                match datastore
                    .client
                    .database(DOCUMENT_DB)
                    .list_collection_names(None)
                    .await
                {
                    Ok(colls) => {
                        debug!("... found collections: {:#?}", &colls);
                        let number_of_colls =
                            match colls.contains(&MONGO_COLL_DOCUMENT_BUCKET.to_string()) {
                                true => colls.len(),
                                false => 0,
                            };

                        if number_of_colls > 0 && clear_db {
                            debug!("Database not empty and clear_db == true. Dropping database...");
                            match datastore.client.database(DOCUMENT_DB).drop(None).await {
                                Ok(_) => {
                                    debug!("... done.");
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to drop database"));
                                }
                            };
                        }
                        if number_of_colls == 0 || clear_db {
                            debug!("Database empty. Need to initialize...");
                            let mut write_concern = WriteConcern::default();
                            write_concern.journal = Some(true);
                            let mut options = CreateCollectionOptions::default();
                            options.write_concern = Some(write_concern);
                            debug!("Create collection {} ...", MONGO_COLL_DOCUMENT_BUCKET);
                            match datastore
                                .client
                                .database(DOCUMENT_DB)
                                .create_collection(MONGO_COLL_DOCUMENT_BUCKET, options)
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

                            // This purpose of this index is to ensure that the transaction counter is unique
                            let mut index_options = IndexOptions::default();
                            index_options.unique = Some(true);
                            let mut index_model = IndexModel::default();
                            index_model.keys = doc! {format!("{}.{}",MONGO_DOC_ARRAY, MONGO_TC): 1};
                            index_model.options = Some(index_options);

                            debug!("Create unique index for {} ...", MONGO_COLL_DOCUMENT_BUCKET);
                            match datastore
                                .client
                                .database(DOCUMENT_DB)
                                .collection::<Document>(MONGO_COLL_DOCUMENT_BUCKET)
                                .create_index(index_model, None)
                                .await
                            {
                                Ok(result) => {
                                    debug!("... index {} created", result.index_name);
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to create index"));
                                }
                            }

                            // This creates a compound index over pid and the timestamp to enable paging using buckets
                            let mut compound_index_model = IndexModel::default();
                            compound_index_model.keys = doc! {MONGO_PID: 1, MONGO_TS: 1};

                            debug!("Create unique index for {} ...", MONGO_COLL_DOCUMENT_BUCKET);
                            match datastore
                                .client
                                .database(DOCUMENT_DB)
                                .collection::<Document>(MONGO_COLL_DOCUMENT_BUCKET)
                                .create_index(compound_index_model, None)
                                .await
                            {
                                Ok(result) => {
                                    debug!("... index {} created", result.index_name);
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to create compound index"));
                                }
                            }
                        }
                        debug!("... database initialized.");
                        Ok(datastore)
                    }
                    Err(_) => Err(anyhow!("Failed to list collections")),
                }
            }
            Err(_) => Err(anyhow!("Failed to initialize database client")),
        }
    }
}
