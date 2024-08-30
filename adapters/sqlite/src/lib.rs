pub mod repositories;
use chrono::NaiveDateTime;
use proseforge_core::editor::{
    models::{
        Content, ContentId, CreateContentError, CreateContentRequest, GetContentError,
        GetContentRequest,
    },
    ports::ContentRepository,
};
use sqlx::Row;
use sqlx::{migrate::MigrateDatabase, Error as SqlxError, Sqlite, SqlitePool};
use sqlx::{
    query,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SqliteAdapterError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    // Add more specific error types if needed
}

#[derive(Clone, Debug)]
struct SqliteAdapter {
    pool: Arc<SqlitePool>,
}

impl SqliteAdapter {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;

        // Ensure the database is migrated to the latest version
        sqlx::migrate!().run(&pool).await?;

        Ok(SqliteAdapter {
            pool: Arc::new(pool),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};
    use sqlx::{query, SqlitePool};
    use tokio;

    const DATABASE_URL: &str = "sqlite::memory:";

    async fn setup() -> Result<SqlitePool, Box<dyn std::error::Error>> {
        // Connect to the in-memory SQLite database
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        // let options = SqliteConnectOptions::default().create_if_missing(true);
        // .extension("ulid0");
        let pool = SqlitePool::connect(DATABASE_URL).await?;
        // let pool: Pool<sqlx::Sqlite> = SqlitePool::connect_with(options).await?;

        Ok(pool)
    }

    #[tokio::test]
    async fn test_migrations() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup().await?;
        sqlx::migrate!()
            .run(&pool)
            .await
            .context("Failed to apply migrations.");

        let list_tables = query("SELECT name FROM sqlite_master WHERE type='table'")
            .fetch_all(&pool)
            .await?;
        let tables_count = list_tables.len();
        assert!(tables_count > 0);

        let table =
            sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='project'")
                .fetch_optional(&pool)
                .await;
        let table_exists = table?.is_some();

        assert!(table_exists, "Table 'project' should exist.");

        // Check if a column exists in the table
        let rows = query("PRAGMA table_info(project)").fetch_all(&pool).await?;
        let column_exists = rows.iter().any(|row| {
            let name: String = row.try_get("name").unwrap_or_default();
            name == "id"
        });

        assert!(
            column_exists,
            "Column 'id' should exist in table 'project'."
        );

        Ok(())
    }
}
