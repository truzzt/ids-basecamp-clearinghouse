pub(crate) mod key_store;
pub(crate) mod doc_store;
pub(crate) mod config;
pub(crate) mod process_store;

use error_chain::error_chain;
use mongodb::Client;
use mongodb::options::ClientOptions;
use crate::model::errors::*;

pub trait DataStoreApi{
    fn new(client: Client) -> Self;
}

pub async fn init_database_client<T: DataStoreApi>(db_url: &str, client_name: Option<String>) -> errors::Result<T>{
    let mut client_options;

    match ClientOptions::parse(&format!("{}", db_url)).await{
        Ok(co) => {client_options = co;}
        Err(_) => {
            error_chain::bail!("Can't parse database connection string");
        }
    };

    client_options.app_name = client_name;
    let client = Client::with_options(client_options)?;
    Ok(T::new(client))
}