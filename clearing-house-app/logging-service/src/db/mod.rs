pub(crate) mod config;
pub(crate) mod doc_store;
pub(crate) mod key_store;
pub(crate) mod process_store;

use crate::model::errors::*;
use mongodb::options::ClientOptions;
use mongodb::Client;

pub trait DataStoreApi {
    fn new(client: Client) -> Self;
}

pub async fn init_database_client<T: DataStoreApi>(
    db_url: &str,
    client_name: Option<String>,
) -> errors::Result<T> {
    let mut client_options;

    match ClientOptions::parse(&db_url.to_string()).await {
        Ok(co) => {
            client_options = co;
        }
        Err(_) => {
            error_chain::bail!("Can't parse database connection string");
        }
    };

    client_options.app_name = client_name;
    let client = Client::with_options(client_options)?;
    Ok(T::new(client))
}
