use sqlx::{sqlite::SqlitePoolOptions, Error, SqlitePool};
use sqlx::Row;
use crate::db::SayaProvingDb;
use crate::db::AtlanticStatus;
pub struct SqliteDb {
    pool: SqlitePool,
}
impl SqliteDb {
    pub async fn new(path: &str) -> Result<Self, Error> {
        
        let pool = SqlitePoolOptions::new().connect(path).await?;
        Ok(Self { pool })
    }
    pub async fn create_database(&self) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS blocks (
                id INTEGER PRIMARY KEY,
                query_id TEXT NOT NULL,
                status TEXT NOT NULL CHECK (status IN ('IN_PROGRESS', 'FAILED'))
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
impl SayaProvingDb for SqliteDb {
    async fn check_status(&self, block: i32) -> Result<(i32, String, String), sqlx::Error> {
        let rows = sqlx::query("SELECT id, query_id, status FROM blocks WHERE id = ?1")
            .bind(block)
            .fetch_all(&self.pool)
            .await?;
        let result = &rows[0];
        let id: i32 = result.get("id");
        let query_id = result.get("query_id");
        let status = result.get("status");
        Ok((id, query_id, status))
    }
    async fn insert_block(
        &self,
        block_id: i32,
        query_id: &str,
        status: AtlanticStatus,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO blocks (id, query_id, status) VALUES (?1, ?2, ?3)")
            .bind(block_id)
            .bind(query_id)
            .bind(status.as_str())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}