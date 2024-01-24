use crate::model::process::Process;
use sqlx::Row;

pub(crate) struct PostgresProcessStore {
    db: sqlx::PgPool,
}

impl PostgresProcessStore {
    pub(crate) fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }
}

impl super::ProcessStore for PostgresProcessStore {
    async fn get_processes(&self) -> anyhow::Result<Vec<Process>> {
        sqlx::query_as::<_, Process>(
            r#"SELECT p.process_id, p.created_at, ARRAY_AGG(c.client_id) AS owners FROM processes p
        LEFT JOIN process_owners po ON p.id = po.process_id
        LEFT JOIN clients c ON po.client_id = c.id
        GROUP BY p.process_id, p.created_at"#,
        )
            .fetch_all(&self.db)
            .await
            .map_err(|e| e.into())
    }

    async fn delete_process(&self, pid: &str) -> anyhow::Result<bool> {
        sqlx::query("DELETE FROM processes WHERE process_id = $1 CASCADE")
            .bind(pid)
            .execute(&self.db)
            .await
            .map(|r| r.rows_affected() == 1)
            .map_err(|e| e.into())
    }

    async fn exists_process(&self, pid: &str) -> anyhow::Result<bool> {
        sqlx::query("SELECT process_id FROM processes WHERE process_id = $1")
            .bind(pid)
            .fetch_optional(&self.db)
            .await
            .map(|r| r.is_some())
            .map_err(|e| e.into())
    }

    async fn get_process(&self, pid: &str) -> anyhow::Result<Option<Process>> {
        sqlx::query_as::<_, Process>(
            r#"SELECT p.process_id, p.created_at, ARRAY_AGG(c.client_id) AS owners FROM processes p
        LEFT JOIN process_owners po ON p.id = po.process_id
        LEFT JOIN clients c ON po.client_id = c.id
        WHERE p.process_id = $1
        GROUP BY p.process_id, p.created_at"#,
        )
            .bind(pid)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| e.into())
    }

    async fn store_process(&self, process: Process) -> anyhow::Result<()> {
        let mut tx = self.db.begin().await?;

        // Create a process
        let process_row = sqlx::query(r#"INSERT INTO processes (process_id) VALUES ($1) RETURNING id"#)
            .bind(&process.id)
            .fetch_one(&mut *tx)
            .await?;

        let pid = process_row.get::<i32, _>("id");

        for o in process.owners {
            // Check if client exists
            let client_row = sqlx::query(r#"SELECT id FROM clients WHERE client_id = $1"#)
                .bind(&o)
                .fetch_optional(&mut *tx)
                .await?;

            // If not, create it
            let client_row = match client_row {
                Some(crow) => crow,
                None => {
                    sqlx::query(r#"INSERT INTO clients (client_id) VALUES ($1) RETURNING id"#)
                        .bind(&o)
                        .fetch_one(&mut *tx)
                        .await?
                }
            };

            // Get id of client
            let oid = client_row.get::<i32, _>("id");

            // Create process owner
            sqlx::query(r#"INSERT INTO process_owners (process_id, client_id) VALUES ($1, $2)"#)
                .bind(pid)
                .bind(oid)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}