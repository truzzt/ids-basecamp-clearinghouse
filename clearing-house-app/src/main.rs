#![forbid(unsafe_code)]

#[macro_use]
extern crate tracing;

use std::net::SocketAddr;
use std::sync::Arc;
use crate::model::constants::{ENV_LOGGING_SERVICE_ID, SIGNING_KEY};
use crate::db::doc_store::DataStore;
use crate::db::key_store::KeyStore;
use crate::db::process_store::ProcessStore;
use crate::util::ServiceConfig;

mod config;
mod crypto;
mod db;
mod model;
mod ports;
mod services;
mod util;
mod errors;

#[derive(Clone)]
pub struct AppState {
    pub keyring_service: Arc<services::keyring_service::KeyringService>,
    pub doc_service: Arc<services::document_service::DocumentService>,
    pub logging_service: Arc<services::logging_service::LoggingService>,
    pub service_config: Arc<util::ServiceConfig>,
    pub signing_key_path: String,
}

fn init_service_config(service_id: String) -> anyhow::Result<ServiceConfig> {
    match std::env::var(&service_id) {
        Ok(id) => Ok(ServiceConfig { service_id: id }),
        Err(_e) => {
            anyhow::bail!(
                    "Service ID not configured. Please configure environment variable {}",
                    &service_id
                );
        }
    }
}

fn init_signing_key(signing_key_path: Option<&str>) -> anyhow::Result<String> {
    let private_key_path = signing_key_path
        .unwrap_or("keys/private_key.der");
    if std::path::Path::new(&private_key_path).exists() {
        Ok(private_key_path.to_string())
    } else {
        anyhow::bail!(
                "Signing key not found! Aborting startup! Please configure signing_key!"
            );
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Read configuration
    let conf = config::read_config(None);
    config::configure_logging(conf.log_level);

    let process_store =
        ProcessStore::init_process_store(conf.process_database_url, conf.clear_db)
            .await
            .expect("Failure to initialize process store! Exiting...");
    let keyring_store =
        KeyStore::init_keystore(conf.keyring_database_url, conf.clear_db)
            .await
            .expect("Failure to initialize keyring store! Exiting...");
    let doc_store =
        DataStore::init_datastore(conf.document_database_url, conf.clear_db)
            .await
            .expect("Failure to initialize document store! Exiting...");

    let keyring_service = Arc::new(services::keyring_service::KeyringService::new(keyring_store));
    let doc_service =
        Arc::new(services::document_service::DocumentService::new(doc_store, keyring_service.clone()));
    let logging_service =
        Arc::new(services::logging_service::LoggingService::new(process_store, doc_service.clone()));

    let service_config = Arc::new(init_service_config(ENV_LOGGING_SERVICE_ID.to_string())?);
    let signing_key = init_signing_key(conf.signing_key.as_deref())?;

    let app_state = AppState {
        signing_key_path: signing_key,
        service_config,
        keyring_service,
        doc_service,
        logging_service,
    };

    let app = axum::Router::new()
        .route("/log/message/:pid", axum::routing::post(ports::logging_api::log))
        .route("/process/:pid", axum::routing::post(ports::logging_api::create_process))
        .route("/messages/query/:pid", axum::routing::post(ports::logging_api::query_pid))
        .route("/messages/query/:pid/:id", axum::routing::post(ports::logging_api::query_id))
        .route("/.well-known/jwks.json", axum::routing::get(ports::logging_api::get_public_sign_key))
        .nest("/doctype", ports::doc_type_api::router())
        .with_state(app_state);


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

/// Signal handler to catch a Ctrl+C and initiate a graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
