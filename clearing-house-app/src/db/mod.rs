#[cfg(feature = "mongodb")]
pub(crate) mod mongo_doc_store;
#[cfg(feature = "mongodb")]
pub(crate) mod mongo_process_store;
#[cfg(feature = "postgres")]
pub(crate) mod postgres_document_store;
#[cfg(feature = "postgres")]
pub(crate) mod postgres_process_store;

use crate::model::document::Document;
use crate::model::process::Process;
use crate::model::SortingOrder;

#[cfg(feature = "mongodb")]
pub async fn init_database_client(
    db_url: &str,
    client_name: Option<String>,
) -> anyhow::Result<mongodb::Client> {
    let mut client_options = match mongodb::options::ClientOptions::parse(&db_url.to_string()).await
    {
        Ok(co) => co,
        Err(_) => {
            anyhow::bail!("Can't parse database connection string");
        }
    };

    client_options.app_name = client_name;
    mongodb::Client::with_options(client_options).map_err(|e| e.into())
}

pub(crate) trait ProcessStore {
    async fn get_processes(&self) -> anyhow::Result<Vec<Process>>;
    async fn delete_process(&self, pid: &str) -> anyhow::Result<bool>;
    async fn exists_process(&self, pid: &str) -> anyhow::Result<bool>;
    async fn get_process(&self, pid: &str) -> anyhow::Result<Option<Process>>;
    async fn store_process(&self, process: Process) -> anyhow::Result<()>;
}

pub(crate) trait DocumentStore {
    async fn add_document(&self, doc: Document) -> anyhow::Result<bool>;
    async fn exists_document(&self, id: &str) -> anyhow::Result<bool>;
    async fn get_document(&self, id: &str, pid: &str) -> anyhow::Result<Option<Document>>;
    async fn get_documents_for_pid(
        &self,
        pid: &str,
        page: u64,
        size: u64,
        sort: &SortingOrder,
        date: (&chrono::NaiveDateTime, &chrono::NaiveDateTime),
    ) -> anyhow::Result<Vec<Document>>;
}
