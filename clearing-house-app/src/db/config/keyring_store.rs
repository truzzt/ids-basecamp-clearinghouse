use crate::db::init_database_client;
use crate::db::key_store::KeyStore;
use crate::model::constants::{
    CLEAR_DB, DATABASE_URL, FILE_DEFAULT_DOC_TYPE, KEYRING_DB, KEYRING_DB_CLIENT,
};
use crate::model::crypto::MasterKey;
use crate::model::doc_type::DocumentType;
use crate::util::read_file;
use anyhow::anyhow;
use rocket::fairing::Kind;
use rocket::{fairing, Build, Rocket};

#[derive(Clone, Debug)]
pub struct KeyringDbConfigurator;

#[rocket::async_trait]
impl fairing::Fairing for KeyringDbConfigurator {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Configuring Keyring Database",
            kind: Kind::Ignite,
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

        match Self::init_keystore(db_url, clear_db).await {
            Ok(keystore) => Ok(rocket.manage(keystore)),
            Err(_) => Err(rocket),
        }
    }
}

impl KeyringDbConfigurator {
    pub async fn init_keystore(db_url: String, clear_db: bool) -> anyhow::Result<KeyStore> {
        debug!("Using database url: '{:#?}'", &db_url);

        match init_database_client::<KeyStore>(
            db_url.as_str(),
            Some(KEYRING_DB_CLIENT.to_string()),
        )
        .await
        {
            Ok(keystore) => {
                debug!("Check if database is empty...");
                match keystore
                    .client
                    .database(KEYRING_DB)
                    .list_collection_names(None)
                    .await
                {
                    Ok(colls) => {
                        debug!("... found collections: {:#?}", &colls);
                        if !colls.is_empty() && clear_db {
                            debug!("Database not empty and clear_db == true. Dropping database...");
                            match keystore.client.database(KEYRING_DB).drop(None).await {
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
                            debug!("Database empty. Need to initialize...");
                            debug!("Adding initial document type...");
                            match serde_json::from_str::<DocumentType>(
                                &read_file(FILE_DEFAULT_DOC_TYPE).unwrap_or(String::new()),
                            ) {
                                Ok(dt) => match keystore.add_document_type(dt).await {
                                    Ok(_) => {
                                        debug!("... done.");
                                    }
                                    Err(e) => {
                                        error!(
                                            "Error while adding initial document type: {:#?}",
                                            e
                                        );
                                        return Err(anyhow!(
                                            "Error while adding initial document type"
                                        ));
                                    }
                                },
                                _ => {
                                    error!("Error while loading initial document type");
                                    return Err(anyhow!(
                                        "Error while loading initial document type"
                                    ));
                                }
                            };
                            debug!("Creating master key...");
                            // create master key
                            match keystore.store_master_key(MasterKey::new_random()).await {
                                Ok(true) => {
                                    debug!("... done.");
                                }
                                _ => {
                                    error!("... failed to create master key");
                                    return Err(anyhow!("Failed to create master key"));
                                }
                            };
                        }
                        debug!("... database initialized.");
                        Ok(keystore)
                    }
                    Err(_) => Err(anyhow!("Failed to list collections")),
                }
            }
            Err(_) => Err(anyhow!("Failed to initialize database client")),
        }
    }
}
