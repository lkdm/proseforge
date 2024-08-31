use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use proseforge_common::Id;
use serde::{Deserialize, Serialize};
use thiserror::Error;

///
/// Service contains functions that more directly relate to the business logic of the application.
/// A service function may call multiple repository functions to accomplish its task.
///
/// It may also publish events or perform other side effects.
///
use super::{
    models::document::{CreateDocumentError, CreateDocumentRequest},
    ports::ProjectRepository,
};

#[derive(Debug, Clone)]
pub struct Service<R>
where
    R: ProjectRepository,
{
    repo: Arc<R>,
    // mem: Arc<Mutex<M>>,
}

impl<R> Service<R>
where
    R: ProjectRepository,
{
    pub fn new(repo: R) -> Self {
        Service {
            repo: Arc::new(repo),
        }
    }
}

pub trait ProjectService {
    fn create_document(
        &self,
        req: &CreateDocumentRequestDto,
    ) -> impl Future<Output = Result<Id, ServiceError>> + Send;
    fn update_document_content(
        &self,
        req: &str,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send;
    fn save_document_changes(&self) -> impl Future<Output = Result<(), ServiceError>> + Send;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequestDto {
    parent_id: Option<String>,
    project_id: String,
    kind: String,
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Document creation failed: {0}")]
    CreateDocumentError(#[from] CreateDocumentError),

    #[error("An unexpected error occurred: {0}")]
    UnexpectedError(String),
}

impl<R> ProjectService for Service<R>
where
    R: ProjectRepository,
{
    /// Create a new document in the project.
    /// This creates a new component and document in the project, and returns the component id.
    async fn create_document(&self, req: &CreateDocumentRequestDto) -> Result<Id, ServiceError> {
        // TODO: This needs to take a parent component_id to know where to put the document.
        let project_id = req.project_id.clone();
        let request = CreateDocumentRequest::new(project_id.into());
        let result = self.repo.create_document(&request).await;
        match result {
            Ok(r) => Ok(r.into()),
            Err(e) => Err(ServiceError::CreateDocumentError(e)),
        }
    }
    async fn update_document_content(&self, req: &str) -> Result<(), ServiceError> {
        print!("Updating document content: {}", req);
        Ok(())
    }
    async fn save_document_changes(&self) -> Result<(), ServiceError> {
        print!("Saving document changes");
        Ok(())
    }
}
