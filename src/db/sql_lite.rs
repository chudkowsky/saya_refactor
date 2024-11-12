use crate::db::AtlanticStatus;
use crate::db::SayaProvingDb;
use sqlx::Row;
use sqlx::{sqlite::SqlitePoolOptions, Error};
use sqlx::{Pool, Sqlite};
use std::fs;
use std::path::Path;

pub struct SqliteDb {
    pool: Pool<Sqlite>,
}

impl SqliteDb {
    pub async fn new(path: &str) -> Result<Self, Error> {
        // Check if there is a database file at the path
        if !Path::new(path).exists() {
            println!(
                "Database file not found. A new one will be created at: {}",
                path
            );
            fs::File::create(path)?;
        } else {
            println!("Database file found at: {}", path);
        }

        // Connect to the database
        let pool = SqlitePoolOptions::new()
            .connect(&format!("sqlite:{}", path))
            .await?;

        // Check if the blocks table exists
        let table_exists =
            sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='blocks';")
                .fetch_optional(&pool)
                .await?
                .is_some();
        // If the table doesn't exist or doesn't have the correct structure, create it
        if !table_exists || !Self::check_columns(&pool).await? {
            println!("Creating or updating the 'blocks' table...");
            Self::create_database(&pool).await?;
        } else {
            println!("Table 'blocks' with correct structure found.");
        }
        Ok(Self { pool })
    }

    // Function to check if the blocks table has the correct columns
    async fn check_columns(pool: &Pool<Sqlite>) -> Result<bool, Error> {
        let columns = sqlx::query("PRAGMA table_info(blocks);")
            .fetch_all(pool)
            .await?;

        // Check if the table has the expected columns: id, query_id, and status
        let mut has_id = false;
        let mut has_query_id = false;
        let mut has_status = false;

        for column in columns {
            let name: String = column.get("name");
            match name.as_str() {
                "id" => has_id = true,
                "query_id" => has_query_id = true,
                "status" => has_status = true,
                _ => {}
            }
        }

        Ok(has_id && has_query_id && has_status)
    }

    // Function to create the blocks table with the correct schema
    pub async fn create_database(pool: &Pool<Sqlite>) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS blocks (
                id INTEGER PRIMARY KEY,
                query_id TEXT NOT NULL,
                status TEXT NOT NULL CHECK (status IN ('IN_PROGRESS', 'FAILED'))
            )",
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

impl SayaProvingDb for SqliteDb {
    async fn check_status(&self, block: u32) -> Result<(u32, String, String), sqlx::Error> {
        let rows = sqlx::query("SELECT id, query_id, status FROM blocks WHERE id = ?1")
            .bind(block)
            .fetch_all(&self.pool)
            .await?;
        let result = &rows[0];
        let id = result.get("id");
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
    async fn list_pending_blocks(&self) -> Result<Vec<(u32, String, String)>, sqlx::Error> {
        let rows =
            sqlx::query("SELECT id, query_id, status FROM blocks WHERE status = 'IN_PROGRESS'")
                .fetch_all(&self.pool)
                .await?;
        let mut result = Vec::new();
        for row in rows {
            let id = row.get("id");
            let query_id = row.get("query_id");
            let status = row.get("status");
            result.push((id, query_id, status));
        }
        Ok(result)
    }
}
