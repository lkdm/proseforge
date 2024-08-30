use anyhow::Context;
use anyhow::Result;
use proseforge_common::Id;
use proseforge_core::features::project::{
    models::document::{
        CreateDocumentError, CreateDocumentRequest, DeleteDocumentError, DeleteDocumentRequest,
        Document, GetDocumentError, GetDocumentRequest, UpdateDocumentError, UpdateDocumentRequest,
    },
    ports::DocumentRepository,
};
use sqlx::{query, Row};

use crate::SqliteAdapter;

impl DocumentRepository for SqliteAdapter {
    async fn create_document(
        &self,
        _req: &CreateDocumentRequest,
    ) -> Result<Id, CreateDocumentError> {
        let pool = self.pool.clone();
        let project_id = _req.project_id();
        let document = Document::builder(project_id).with_defaults().build();
        print!("{:?}", document);
        let row = query("INSERT INTO document (id, project_id, content, modified_at, created_at) VALUES (?, ?, ?, ?, ?) RETURNING id")
            .bind::<String>(document.id().into())
            .bind::<String>(document.project_id().into())
            .bind::<String>(document.content().into())
            .bind::<String>(document.modified_at().into())
            .bind::<String>(document.created_at().into())
            .fetch_one(&*pool)
            .await
            .map_err(|e| CreateDocumentError::UnexpectedError(e.to_string()))?;
        let raw_id: String = row
            .try_get("id")
            .map_err(|e| CreateDocumentError::UnexpectedError(e.to_string()))?;
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
        .bind::<String>(req.id().into())
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
        query(
            "
                UPDATE document
                SET content = ?, modified_at = ?
                WHERE id = ?
            ",
        )
        .bind::<String>(req.content().into())
        .bind::<String>(req.modified_at().into())
        .bind::<String>(req.id().into())
        .execute(&*pool)
        .await?;
        Ok(())
    }

    async fn delete_document(
        &self,
        req: &DeleteDocumentRequest,
    ) -> Result<(), DeleteDocumentError> {
        let pool = self.pool.clone();
        let affected_rows = query(
            "
            UPDATE document
            SET deleted_at = ?
            WHERE id = ?
            ",
        )
        .bind::<String>(req.deleted_at().into())
        .bind::<String>(req.id().into())
        .execute(&*pool)
        .await?
        .rows_affected();

        if affected_rows == 0 {
            Err(DeleteDocumentError::UnexpectedError(
                "Unexpected error".into(),
            ))
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
    fn project_id() -> Id {
        Id::from_string("01J6JH8BPPXN3HSJHP8JXS12KX").expect("Failed to create project id")
    }

    /// Setup the in-memory SQLite database
    async fn setup() -> Result<SqliteAdapter, Box<dyn std::error::Error>> {
        // Connect to the in-memory SQLite database
        // std::env::set_var("DATABASE_URL", "sqlite::memory:");
        let adapter = SqliteAdapter::new(DATABASE_URL)
            .await
            .context("Failed to create adapter")?;

        Ok(adapter)
    }

    /// Seed the database with test data
    async fn seed(adapter: SqliteAdapter) -> Result<(), Box<dyn std::error::Error>> {
        let pool = adapter.pool();
        let pool_ref = &*pool;
        query(
            "
            INSERT INTO project (id, title, kind, created_at, modified_at)
            VALUES (?, 'Test Project', 'shortstory', '2021-01-01', '2021-01-01')
            ",
        )
        .bind::<String>(project_id().into())
        .execute(pool_ref)
        .await?;
        Ok(())
    }

    #[tokio::test]
    /// Tests for the existence of the table
    async fn test_for_table() -> Result<(), Box<dyn std::error::Error>> {
        let adapter = setup().await?;
        let pool = adapter.pool();
        let pool_ref = &*pool;
        let table =
            sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='document'")
                .fetch_optional(pool_ref)
                .await;
        let table_exists = table?.is_some();

        assert!(table_exists, "Table 'document' should exist.");
        Ok(())
    }

    #[tokio::test]
    async fn test_create_document() -> Result<(), Box<dyn std::error::Error>> {
        let adapter = setup().await?;
        seed(adapter.clone()).await?;
        let req = CreateDocumentRequest::new(project_id().into());

        let result = adapter.create_document(&req).await;

        match result {
            Ok(_) => assert!(true),
            Err(e) => {
                eprintln!("Error creating document: {:?}", e);
                assert!(false, "Test failed.");
            }
        }
        Ok(())
    }
}
