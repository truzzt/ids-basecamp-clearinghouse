use crate::model::document::Document;
use crate::model::ids::{InfoModelDateTime, InfoModelId};
use crate::model::SortingOrder;

pub(crate) struct PostgresDocumentStore {
    db: sqlx::PgPool,
}

impl PostgresDocumentStore {
    pub(crate) async fn new(db: sqlx::PgPool, clear_db: bool) -> Self {
        if clear_db {
            info!("Clearing database 'documents'");
            sqlx::query("TRUNCATE documents")
                .execute(&db)
                .await
                .expect("Clearing database 'documents' failed");
        }

        Self { db }
    }
}

impl super::DocumentStore for PostgresDocumentStore {
    async fn add_document(&self, doc: Document) -> anyhow::Result<bool> {
        let doc = DocumentRow::from(doc);

        sqlx::query(
            r#"INSERT INTO documents
        (id, process_id, created_at, model_version, correlation_message,
        transfer_contract, issued, issuer_connector, content_version, recipient_connector,
        sender_agent, recipient_agent, payload, payload_type, message_id)
        VALUES
        ($1, (SELECT id from processes where process_id = $2), $3, $4, $5,
        $6, $7, $8, $9, $10,
        $11, $12, $13, $14, $15)"#,
        )
        .bind(doc.id) // 1
        .bind(doc.process_id) // 2
        .bind(doc.created_at) // 3
        .bind(doc.model_version) // 4
        .bind(doc.correlation_message) // 5
        .bind(doc.transfer_contract) // 6
        .bind(doc.issued) // 7
        .bind(doc.issuer_connector) // 8
        .bind(doc.content_version) // 9
        .bind(doc.recipient_connector) // 10
        .bind(doc.sender_agent) // 11
        .bind(doc.recipient_agent) // 12
        .bind(doc.payload) // 13
        .bind(doc.payload_type) // 14
        .bind(doc.message_id) // 15
        .execute(&self.db)
        .await?;

        Ok(true)
    }

    async fn exists_document(&self, id: &str) -> anyhow::Result<bool> {
        sqlx::query("SELECT id FROM documents WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map(|r| r.is_some())
            .map_err(|e| e.into())
    }

    async fn get_document(&self, id: &str, pid: &str) -> anyhow::Result<Option<Document>> {
        sqlx::query_as::<_, DocumentRow>(
            r#"SELECT documents.id, processes.process_id, documents.created_at, model_version, correlation_message,
        transfer_contract, issued, issuer_connector, content_version, recipient_connector,
        sender_agent, recipient_agent, payload, payload_type, message_id
        FROM documents
        LEFT JOIN processes ON processes.id = documents.process_id
        WHERE id = $1 AND processes.process_id = $2"#,
        )
        .bind(id)
        .bind(pid)
        .fetch_optional(&self.db)
        .await
        .map(|r| r.map(DocumentRow::into))
        .map_err(|e| e.into())
    }

    async fn get_documents_for_pid(
        &self,
        pid: &str,
        page: u64,
        size: u64,
        sort: &SortingOrder,
        (date_from, date_to): (&chrono::NaiveDateTime, &chrono::NaiveDateTime),
    ) -> anyhow::Result<Vec<Document>> {
        let sort_order = match sort {
            SortingOrder::Ascending => "ASC",
            SortingOrder::Descending => "DESC",
        };

        sqlx::query_as::<_, DocumentRow>(
            format!(
                r#"SELECT documents.id, processes.process_id, documents.created_at, model_version, correlation_message,
        transfer_contract, issued, issuer_connector, content_version, recipient_connector,
        sender_agent, recipient_agent, payload, payload_type, message_id
        FROM documents
        LEFT JOIN processes ON processes.id = documents.process_id
        WHERE processes.process_id = $1 AND documents.created_at BETWEEN $2 AND $3
        ORDER BY created_at {}
        LIMIT $4 OFFSET $5"#,
                sort_order
            )
            .as_str(),
        )
        .bind(pid)
        .bind(date_from)
        .bind(date_to)
        .bind(size as i64)
        .bind(((page - 1) * size) as i64)
        .fetch_all(&self.db)
        .await
        .map(|r| r.into_iter().map(DocumentRow::into).collect())
        .map_err(|e| e.into())
    }
}

#[derive(sqlx::FromRow)]
struct DocumentRow {
    id: uuid::Uuid,
    process_id: String,
    created_at: chrono::NaiveDateTime,
    model_version: String,
    correlation_message: Option<String>,
    transfer_contract: Option<String>,
    issued: sqlx::types::Json<InfoModelDateTime>,
    issuer_connector: sqlx::types::Json<InfoModelId>,
    content_version: Option<String>,
    recipient_connector: Option<sqlx::types::Json<Vec<InfoModelId>>>,
    sender_agent: String,
    recipient_agent: Option<sqlx::types::Json<Vec<InfoModelId>>>,
    payload: Option<Vec<u8>>,
    payload_type: Option<String>,
    message_id: Option<String>,
}

impl From<Document> for DocumentRow {
    fn from(value: Document) -> Self {
        use std::str::FromStr;
        Self {
            id: uuid::Uuid::from_str(&value.id).unwrap(),
            process_id: value.pid,
            created_at: value.ts.naive_utc(),
            model_version: value.content.model_version,
            correlation_message: value.content.correlation_message,
            transfer_contract: value.content.transfer_contract,
            issued: sqlx::types::Json(value.content.issued),
            issuer_connector: sqlx::types::Json(value.content.issuer_connector),
            content_version: value.content.content_version,
            recipient_connector: value.content.recipient_connector.map(sqlx::types::Json),
            sender_agent: value.content.sender_agent,
            recipient_agent: value.content.recipient_agent.map(sqlx::types::Json),
            payload: value.content.payload.map(|s| s.as_bytes().to_owned()),
            payload_type: value.content.payload_type,
            message_id: value.content.id,
        }
    }
}

impl Into<Document> for DocumentRow {
    fn into(self) -> Document {
        use chrono::TimeZone;

        Document {
            id: self.id.to_string(),
            pid: self.process_id,
            ts: chrono::Local.from_utc_datetime(&self.created_at),
            content: crate::model::ids::message::IdsMessage {
                model_version: self.model_version,
                correlation_message: self.correlation_message,
                transfer_contract: self.transfer_contract,
                issued: self.issued.0,
                issuer_connector: self.issuer_connector.0,
                content_version: self.content_version,
                recipient_connector: self.recipient_connector.map(|s| s.0),
                sender_agent: self.sender_agent,
                recipient_agent: self.recipient_agent.map(|s| s.0),
                payload: self
                    .payload
                    .map(|s| String::from_utf8_lossy(s.as_ref()).to_string()),
                payload_type: self.payload_type,
                id: self.message_id,
                ..Default::default()
            },
        }
    }
}
