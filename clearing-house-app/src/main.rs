#![forbid(unsafe_code)]
#![warn(clippy::unwrap_used)]

#[macro_use]
extern crate tracing;

use crate::db::doc_store::DataStore;
use crate::db::key_store::KeyStore;
use crate::db::process_store::ProcessStore;
use crate::model::constants::ENV_LOGGING_SERVICE_ID;
use crate::util::ServiceConfig;
use std::net::SocketAddr;
use std::sync::Arc;

mod config;
mod crypto;
mod db;
mod model;
mod ports;
mod services;
mod util;

/// Contains the application state
#[derive(Clone)]
pub(crate) struct AppState {
    #[cfg_attr(not(doc_type), allow(dead_code))]
    pub keyring_service: Arc<services::keyring_service::KeyringService>,
    pub logging_service: Arc<services::logging_service::LoggingService>,
    pub service_config: Arc<ServiceConfig>,
    pub signing_key_path: String,
}

impl AppState {
    /// Initialize the application state from config
    async fn init(conf: &config::CHConfig) -> anyhow::Result<Self> {
        trace!("Initializing Process store");
        let process_store =
            ProcessStore::init_process_store(&conf.process_database_url, conf.clear_db)
                .await
                .expect("Failure to initialize process store! Exiting...");
        trace!("Initializing Keyring store");
        let keyring_store = KeyStore::init_keystore(&conf.keyring_database_url, conf.clear_db)
            .await
            .expect("Failure to initialize keyring store! Exiting...");
        trace!("Initializing Document store");
        let doc_store = DataStore::init_datastore(&conf.document_database_url, conf.clear_db)
            .await
            .expect("Failure to initialize document store! Exiting...");

        trace!("Initializing services");
        let keyring_service = Arc::new(services::keyring_service::KeyringService::new(
            keyring_store,
        ));
        let doc_service = Arc::new(services::document_service::DocumentService::new(
            doc_store,
            keyring_service.clone(),
        ));
        let logging_service = Arc::new(services::logging_service::LoggingService::new(
            process_store,
            doc_service.clone(),
        ));

        let service_config = Arc::new(util::init_service_config(
            ENV_LOGGING_SERVICE_ID.to_string(),
        )?);
        let signing_key = util::init_signing_key(conf.signing_key.as_deref())?;

        Ok(Self {
            signing_key_path: signing_key,
            service_config,
            keyring_service,
            logging_service,
        })
    }
}

/// Main function: Reading config, initializing application state, starting server
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Read configuration
    let conf = config::read_config(None);
    config::configure_logging(&conf);

    info!("Config read successfully! Initializing application ...");

    // Initialize application state
    let app_state = AppState::init(&conf).await?;

    // Setup router
    let app = ports::router()
        .with_state(app_state);

    // Bind port and start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Starting server: Listening on {}", addr);
    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(util::shutdown_signal())
        .await?)
}
