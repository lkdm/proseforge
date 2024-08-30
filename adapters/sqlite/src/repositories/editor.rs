use chrono::NaiveDateTime;
use proseforge_common::Id;
use proseforge_core::{
    editor::{
        models::{
            CreateDocumentError, CreateDocumentRequest, Document, GetDocumentError,
            GetDocumentRequest, UpdateDocumentError, UpdateDocumentRequest,
        },
        ports::DocumentRepository,
    },
    node::Timestamp,
};
use sqlx::{query, Row};

use crate::SqliteAdapter;

// PRR: ProjectRepository,
// CMP: ComponentRepository,
// COR: ContentRepository,

impl DocumentRepository for SqliteAdapter {
    async fn create_document(
        &self,
        _req: &CreateDocumentRequest,
    ) -> Result<Id, CreateDocumentError> {
        let pool = self.pool.clone();
        let content = _req.content();
        let id = Id::new();
        let row = query("INSERT INTO document (id, content) VALUES (?, ?) RETURNING id")
            .bind::<String>(id.into())
            .bind::<String>(content.into())
            .fetch_one(&*pool)
            .await?;
        let raw_id: String = row.try_get("id")?;
        Ok(Id::from(raw_id))
    }
    async fn get_document(&self, req: &GetDocumentRequest) -> Result<Document, GetDocumentError> {
        let pool = self.pool.clone();
        let row = query(
            "
            SELECT id, project_id, content, created_at, modified_at, deleted_at
            FROM document
            WHERE id = ?
        ",
        )
        .bind::<String>(req.id.clone().into())
        .fetch_one(&*pool)
        .await?;

        let id: String = row.try_get("id")?;
        let content: String = row.try_get("content")?;
        let project_id: String = row.try_get("project_id")?;
        let created_at: String = row.try_get("created_at")?;
        let modified_at: Option<String> = row.try_get("modified_at")?;
        let deleted_at: Option<String> = row.try_get("deleted_at")?;

        let document = Document::builder(project_id)
            .with_id(id)
            .with_content(content)
            .with_created_at(created_at)
            .with_modified_at(modified_at)
            .with_deleted_at(deleted_at)
            .build();

        Ok(document)
    }

    async fn update_document(
        &self,
        req: &UpdateDocumentRequest,
    ) -> Result<(), UpdateDocumentError> {
        let pool = self.pool.clone();
        let id = req.id();
        let content = req.content();
        query(
            "
                UPDATE document
                SET content = ?, modified_at = ?
                WHERE id = ?
            ",
        )
        .bind::<String>(content.into())
        .bind::<String>(Timestamp::now().into()) // Set `modified_at` to the current time
        .bind::<String>(id.into())
        .execute(&*pool)
        .await?; // Handle error as needed
        Ok(())
    }

    async fn delete_document(
        &self,
        req: &DeleteDocumentRequest,
    ) -> Result<(), DeleteDocumentError> {
        let pool = self.pool.clone();
        let affected_rows = query("DELETE FROM document WHERE id = ?")
            .bind::<String>(req.id.clone().into())
            .execute(&*pool)
            .await?
            .rows_affected();

        if affected_rows == 0 {
            Err(DeleteDocumentError::NotFound) // Define this variant in your error enum
        } else {
            Ok(())
        }
    }
}

mod tests {
    use super::*;
    use anyhow::{Context, Result};
    use sqlx::{query, SqlitePool};
    use tokio;

    const DATABASE_URL: &str = "sqlite::memory:";

    async fn setup() -> Result<SqlitePool, Box<dyn std::error::Error>> {
        // Connect to the in-memory SQLite database
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        let pool = SqlitePool::connect(DATABASE_URL).await?;
        sqlx::migrate!()
            .run(&pool)
            .await
            .context("Failed to apply migrations.");

        Ok(pool)
    }

    #[tokio::test]
    /// Tests for the existence of the table
    async fn test_for_table() -> Result<(), Box<dyn std::error::Error>> {
        let pool = setup().await?;
        let table = sqlx::query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='editor_document'",
        )
        .fetch_optional(&pool)
        .await;
        let table_exists = table?.is_some();

        assert!(table_exists, "Table 'editor_document' should exist.");
        Ok(())
    }
}
