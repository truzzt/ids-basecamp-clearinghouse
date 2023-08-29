#![forbid(unsafe_code)]

#[macro_use]
extern crate tracing;

use crate::model::constants::ENV_LOGGING_SERVICE_ID;
use db::config::doc_store::DatastoreConfigurator;
use db::config::keyring_store::KeyringDbConfigurator;
use db::config::process_store::ProcessStoreConfigurator;

mod config;
mod crypto;
mod db;
mod model;
mod ports;
mod services;
mod util;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Read configuration
    let conf = config::read_config(None);
    config::configure_logging(conf.log_level);

    let process_store =
        ProcessStoreConfigurator::init_process_store(conf.process_database_url, conf.clear_db)
            .await
            .expect("Failure to initialize process store! Exiting...");
    let keyring_store =
        KeyringDbConfigurator::init_keystore(conf.keyring_database_url, conf.clear_db)
            .await
            .expect("Failure to initialize keyring store! Exiting...");
    let doc_store =
        DatastoreConfigurator::init_datastore(conf.document_database_url, conf.clear_db)
            .await
            .expect("Failure to initialize document store! Exiting...");

    let keyring_service = services::keyring_service::KeyringService::new(keyring_store);
    let doc_service =
        services::document_service::DocumentService::new(doc_store, keyring_service.clone());
    let logging_service =
        services::logging_service::LoggingService::new(process_store, doc_service.clone());

    let _rocket = rocket::build()
        .manage(keyring_service)
        .manage(doc_service)
        .manage(logging_service)
        .attach(util::add_signing_key())
        .attach(util::add_service_config(ENV_LOGGING_SERVICE_ID.to_string()))
        .attach(ports::logging_api::mount_api())
        .attach(ports::doc_type_api::mount_api())
        .ignite()
        .await?
        .launch()
        .await?;

    Ok(())
}
