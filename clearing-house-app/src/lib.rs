#[macro_use]
extern crate tracing;

use crate::model::constants::ENV_LOGGING_SERVICE_ID;
use crate::util::ServiceConfig;
use std::sync::Arc;

mod config;
mod db;
pub mod model;
mod ports;
mod services;
pub mod util;

#[cfg(feature = "postgres")]
type PostgresLoggingService = services::logging_service::LoggingService<
    db::postgres_process_store::PostgresProcessStore,
    db::postgres_document_store::PostgresDocumentStore,
>;
#[cfg(feature = "mongodb")]
type MongoLoggingService = services::logging_service::LoggingService<
    db::mongo_process_store::MongoProcessStore,
    db::mongo_doc_store::MongoDocumentStore,
>;

/// Contains the application state
#[derive(Clone)]
pub(crate) struct AppState {
    pub logging_service: Arc<PostgresLoggingService>,
    pub service_config: Arc<ServiceConfig>,
    pub signing_key_path: String,
}

impl AppState {
    /// Initialize the application state from config
    async fn init(conf: &config::CHConfig) -> anyhow::Result<Self> {
        trace!("Initializing Process store");
        /*let process_store =
        crate::db::mongo_process_store::MongoProcessStore::init_process_store(&conf.database_url, conf.clear_db)
            .await
            .expect("Failure to initialize process store! Exiting...");*/
        let process_store = db::postgres_process_store::PostgresProcessStore::new(
            sqlx::PgPool::connect("postgres://my_user:my_password@localhost:5432/ch")
                .await
                .unwrap(),
        );

        trace!("Initializing Document store");
        /*let doc_store = MongoDocumentStore::init_datastore(&conf.document_database_url, conf.clear_db)
        .await
        .expect("Failure to initialize document store! Exiting...");*/

        let doc_store = db::postgres_document_store::PostgresDocumentStore::new(
            sqlx::PgPool::connect("postgres://my_user:my_password@localhost:5432/ch")
                .await
                .unwrap(),
        );

        trace!("Initializing services");
        let doc_service = Arc::new(services::document_service::DocumentService::new(doc_store));
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
            logging_service,
        })
    }
}

pub async fn app() -> anyhow::Result<axum::Router> {
    // Read configuration
    let conf = config::read_config(None);
    config::configure_logging(&conf);

    tracing::info!("Config read successfully! Initializing application ...");

    // Initialize application state
    let app_state = AppState::init(&conf).await?;

    // Setup router
    Ok(ports::router().with_state(app_state))
}
