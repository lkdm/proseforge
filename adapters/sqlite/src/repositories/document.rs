use anyhow::Result;
use proseforge_common::Id;
use proseforge_core::features::project::models::document::CreateDocumentRequest;
use proseforge_core::features::project::models::document::GetDocumentError;
use proseforge_core::features::project::models::document::GetDocumentRequest;
use proseforge_core::features::project::{
    models::document::{
        CreateDocumentError, DeleteDocumentError, DeleteDocumentRequest, Document,
        UpdateDocumentError, UpdateDocumentRequest,
    },
    ports::ProjectRepository,
};
use sqlx::{query, Row};

use crate::SqliteAdapter;

impl ProjectRepository for SqliteAdapter {
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

        let c: String = row.try_get("created_at")?;
        print!("{:?}", c);

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

    // async fn create_component(
    //     &self,
    //     req: &CreateComponentRequest,
    // ) -> Result<ProjectComponent, CreateComponentError> {
    //     let pool = self.pool.clone();
    //     let component = ProjectComponent::builder(req.project_id())
    //         .with_kind(req.kind())
    //         .with_display_order(req.display_order())
    //         .with_title(Some(req.title()))
    //         .with_summary(req.summary())
    //         .with_parent_id(req.parent_id())
    //         .with_document_id(req.document_id())
    //         .build();

    //     let row = query("
    //             INSERT INTO component (id, kind, display_order, title, summary, project_id, parent_id, document_id, created_at, modified_at)
    //             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id
    //         ")
    //         .bind::<String>(component.id().into())
    //         .bind::<String>(component.kind().into())
    //         .bind(component.display_order())
    //         .bind::<String>(component.title().into())
    //         .bind::<String>(component.summary().into())
    //         .bind::<String>(component.project_id().into())
    //         .bind::<Option<String>>(component.parent_id().map(|id| id.into()))
    //         .bind::<Option<String>>(component.document_id().map(|id| id.into()))
    //         .bind::<String>(component.created_at().into())
    //         .bind::<String>(component.modified_at().into())
    //         .fetch_one(&*pool)
    //         .await
    //         .map_err(|e| CreateComponentError::UnexpectedError(e.to_string()))?;

    //     let id: String = row
    //         .try_get("id")
    //         .map_err(|e| CreateComponentError::UnexpectedError(e.to_string()))?;
    //     Ok(ProjectComponent::builder(req.project_id())
    //         .with_id(Id::from(id))
    //         .build())
    // }
    // async fn get_component(
    //     &self,
    //     req: &GetComponentRequest,
    // ) -> Result<ProjectComponent, GetComponentError> {
    //     let pool = self.pool.clone();
    //     let row = query("
    //             SELECT id, kind, display_order, title, summary, project_id, parent_id, document_id, created_at, modified_at, deleted_at
    //             FROM component
    //             WHERE id = ?
    //         ")
    //         .bind::<String>(req.id().into())
    //         .fetch_one(&*pool)
    //         .await?;

    //     let id: String = row.try_get("id")?;
    //     let kind: String = row.try_get("kind")?;
    //     let display_order: i32 = row.try_get("display_order")?;
    //     let title: Option<String> = row.try_get("title")?;
    //     let summary: Option<String> = row.try_get("summary")?;
    //     let project_id: String = row.try_get("project_id")?;
    //     let parent_id: Option<String> = row.try_get("parent_id")?;
    //     let document_id: Option<String> = row.try_get("document_id")?;
    //     let created_at: String = row.try_get("created_at")?;
    //     let modified_at: String = row.try_get("modified_at")?;
    //     let deleted_at: Option<String> = row.try_get("deleted_at")?;

    //     let component = ProjectComponent::builder(project_id)
    //         .with_id(id.into())
    //         .with_kind(kind.into())
    //         .with_display_order(display_order)
    //         .with_title(title)
    //         .with_summary(summary)
    //         .with_parent_id(parent_id.map(|id| id.into()))
    //         .with_document_id(document_id.map(|id| id.into()))
    //         .with_created_at(created_at.into())
    //         .with_modified_at(modified_at.into())
    //         .with_deleted_at(deleted_at.map(|dt| dt.into()))
    //         .build();

    //     Ok(component)
    // }
    // async fn update_component(
    //     &self,
    //     req: &UpdateComponentRequest,
    // ) -> Result<(), UpdateComponentError> {
    //     let pool = self.pool.clone();
    //     query("
    //             UPDATE component
    //             SET kind = ?, display_order = ?, title = ?, summary = ?, parent_id = ?, document_id = ?, modified_at = ?
    //             WHERE id = ?
    //         ")
    //         .bind::<String>(req.kind().into())
    //         .bind(req.display_order())
    //         .bind::<Option<String>>(req.title().map(|t| t.into()))
    //         .bind::<Option<String>>(req.summary().map(|s| s.into()))
    //         .bind::<Option<String>>(req.parent_id().map(|id| id.into()))
    //         .bind::<Option<String>>(req.document_id().map(|id| id.into()))
    //         .bind::<String>(req.modified_at().into())
    //         .bind::<String>(req.id().into())
    //         .execute(&*pool)
    //         .await
    //         .map_err(|e| UpdateComponentError::UnexpectedError(e.to_string()))?;
    //     Ok(())
    // }

    // async fn delete_component(
    //     &self,
    //     req: &DeleteComponentRequest,
    // ) -> Result<(), DeleteComponentError> {
    //     let pool = self.pool.clone();
    //     let affected_rows = query(
    //         "
    //             UPDATE component
    //             SET deleted_at = ?
    //             WHERE id = ?
    //         ",
    //     )
    //     .bind::<String>(req.deleted_at().into())
    //     .bind::<String>(req.id().into())
    //     .execute(&*pool)
    //     .await?
    //     .rows_affected();

    //     if affected_rows == 0 {
    //         Err(DeleteComponentError::UnexpectedError(
    //             "Unexpected error".into(),
    //         ))
    //     } else {
    //         Ok(())
    //     }
    // }
}

mod tests {
    use super::*;
    use anyhow::{Context, Result};

    use sqlx::query;
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
    async fn test_document_repository() -> Result<(), Box<dyn std::error::Error>> {
        let adapter = setup().await?;
        seed(adapter.clone()).await?;

        // Create a document
        let req = CreateDocumentRequest::new(project_id().into());
        let res = adapter
            .create_document(&req)
            .await
            .expect("Failed to create document.");

        let id = res.clone();
        // Get the document
        let req = GetDocumentRequest::new(res.into());
        let document = adapter
            .get_document(&req)
            .await
            .expect("Failed to get document.");
        assert_eq!(document.project_id(), project_id());

        // Update the document
        let req =
            UpdateDocumentRequest::new(id.clone().into(), String::from("Hello World!").into());
        let res = adapter
            .update_document(&req)
            .await
            .expect("Failed to update document.");
        assert_eq!(res, ());

        // Get document again to check if it was updated
        let req = GetDocumentRequest::new(id.clone().into());
        let document = adapter
            .get_document(&req)
            .await
            .expect("Failed to get document.");
        assert_eq!(document.content(), "Hello World!".to_string().into());

        // Soft-delete the document
        let req = DeleteDocumentRequest::new(id.clone().into());
        let res = adapter
            .delete_document(&req)
            .await
            .expect("Failed to delete document.");
        assert_eq!(res, ());

        // Check if the document was soft-deleted
        let req = GetDocumentRequest::new(id.into());
        let document = adapter.get_document(&req).await;
        assert!(document.unwrap().deleted_at().is_some());

        Ok(())
    }

    // #[tokio::test]
    // async fn test_component_repository() -> Result<(), Box<dyn std::error::Error>> {
    //     let adapter = setup().await?;
    //     seed(adapter.clone()).await?;
    //     // Create a component
    //     let req = CreateComponentRequest::new(
    //         project_id(),
    //         ComponentKind::Note,
    //         Some(String::from("Title").into()),
    //         Some(String::from("Summary").into()),
    //         None,
    //         None,
    //         0,
    //     );
    //     let res = adapter
    //         .create_component(&req)
    //         .await
    //         .expect("Failed to create component.");

    //     let id = res.id();

    //     // Get the component
    //     let req = GetComponentRequest::new(id.clone());
    //     let component = adapter
    //         .get_component(&req)
    //         .await
    //         .expect("Failed to get component.");
    //     assert_eq!(component.project_id(), project_id());

    //     // Update the component
    //     let req = UpdateComponentRequest::new(
    //         id.clone(),
    //         Some("New Title".into()),
    //         Some("New Summary".into()),
    //         2,
    //         None,
    //         None,
    //         "2022-01-01T00:00:00".into(),
    //     );
    //     adapter
    //         .update_component(&req)
    //         .await
    //         .expect("Failed to update component.");

    //     // Get component again to check if it was updated
    //     let req = GetComponentRequest::new(id.clone());
    //     let component = adapter
    //         .get_component(&req)
    //         .await
    //         .expect("Failed to get component.");
    //     assert_eq!(component.title().as_deref(), Some("New Title"));

    //     // Soft-delete the component
    //     let req = DeleteComponentRequest::new(id.clone(), "2022-01-01T00:00:00".into());
    //     adapter
    //         .delete_component(&req)
    //         .await
    //         .expect("Failed to delete component.");

    //     // Check if the component was soft-deleted
    //     let req = GetComponentRequest::new(id.into());
    //     let component = adapter.get_component(&req).await;
    //     assert!(component.unwrap().deleted_at().is_some());

    //     Ok(())
    // }
}
