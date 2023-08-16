#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

use std::path::Path;
use core_lib::util::{add_service_config};
use rocket::fairing::AdHoc;
use core_lib::constants::ENV_LOGGING_SERVICE_ID;
use db::config::doc_store::DatastoreConfigurator;
use db::config::keyring_store::KeyringDbConfigurator;
use db::config::process_store::ProcessStoreConfigurator;
use model::constants::SIGNING_KEY;

mod db;
mod model;
mod services;
mod crypto;
mod ports;

pub fn add_signing_key() -> AdHoc {
    AdHoc::try_on_ignite("Adding Signing Key", |rocket| async {
        let private_key_path = rocket.figment().extract_inner(SIGNING_KEY).unwrap_or(String::from("keys/private_key.der"));
        if Path::new(&private_key_path).exists() {
            Ok(rocket.manage(private_key_path))
        } else {
            error!("Signing key not found! Aborting startup! Please configure signing_key!");
            return Err(rocket);
        }
    })
}

#[derive(Debug, serde::Deserialize)]
struct CHConfig {
    process_database_url: String,
    keyring_database_url: String,
    document_database_url: String,
    clear_db: bool,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Read configuration
    let conf = config::Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .add_source(config::Environment::with_prefix("CH_APP_"))
        .build()
        .expect("Failure to read configuration! Exiting...");

    // setup logging
    // TODO: Setup tracing_subscriber

    let conf: CHConfig = conf.try_deserialize().expect("Failure to read configuration! Exiting...");
    println!("Config: {:?}", conf);


    let process_store =
        ProcessStoreConfigurator::init_process_store(String::from(conf.process_database_url), conf.clear_db)
            .await
            .expect("Failure to initialize process store! Exiting...");
    let keyring_store =
        KeyringDbConfigurator::init_keystore(String::from(conf.keyring_database_url), conf.clear_db)
            .await
            .expect("Failure to initialize keyring store! Exiting...");
    let doc_store =
        DatastoreConfigurator::init_datastore(String::from(conf.document_database_url), conf.clear_db)
            .await
            .expect("Failure to initialize document store! Exiting...");

    let keyring_service = services::keyring_service::KeyringService::new(keyring_store);
    let doc_service = services::document_service::DocumentService::new(doc_store, keyring_service.clone());
    let logging_service = services::logging_service::LoggingService::new(
        process_store,
        doc_service.clone(),
    );

    let _rocket = rocket::build()
        .manage(keyring_service)
        .manage(doc_service)
        .manage(logging_service)
        .attach(add_signing_key())
        .attach(add_service_config(ENV_LOGGING_SERVICE_ID.to_string()))
        .attach(ports::logging_api::mount_api())
        .attach(ports::doc_type_api::mount_api())
        .ignite().await?
        .launch().await?;

    Ok(())
}
