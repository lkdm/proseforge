use std::{future::Future, sync::Arc};

use proseforge_common::Id;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::features::project::models::document::GetDocumentRequest;

///
/// Service contains functions that more directly relate to the business logic of the application.
/// A service function may call multiple repository functions to accomplish its task.
///
/// It may also publish events or perform other side effects.
///
use super::{models::document::CreateDocumentRequest, ports::ProjectRepository};

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
    fn document_create(
        &self,
        req: &CreateDocumentRequestDto,
    ) -> impl Future<Output = Result<Id, ServiceError>> + Send;
    fn document_content_update(
        &self,
        req: &str,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send;
    fn document_content_save(&self) -> impl Future<Output = Result<(), ServiceError>> + Send;
    fn document_get(
        &self,
        req: &GetDocumentRequestDto,
    ) -> impl Future<Output = Result<GetDocumentResponseDto, ServiceError>> + Send;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequestDto {
    parent_id: Option<String>,
    project_id: String,
    kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRequestDto {
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentResponseDto {
    id: String,
    content: String,
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ServiceError {
    #[error("Document creation failed")]
    CreateDocumentError,

    #[error("Document retrieval failed")]
    GetDocumentError,

    #[error("An unexpected error occurred")]
    UnexpectedError(String),
}

impl<R> ProjectService for Service<R>
where
    R: ProjectRepository,
{
    /// Create a new document in the project.
    /// This creates a new component and document in the project, and returns the component id.
    async fn document_create(&self, req: &CreateDocumentRequestDto) -> Result<Id, ServiceError> {
        // TODO: This needs to take a parent component_id to know where to put the document.
        let project_id = req.project_id.clone();
        let request = CreateDocumentRequest::new(project_id.into());
        let result = self.repo.create_document(&request).await;
        match result {
            Ok(r) => Ok(r.into()),
            Err(e) => Err(ServiceError::CreateDocumentError),
        }
    }
    async fn document_content_update(&self, req: &str) -> Result<(), ServiceError> {
        print!("Updating document content: {}", req);
        Ok(())
    }
    async fn document_content_save(&self) -> Result<(), ServiceError> {
        print!("Saving document changes");
        Ok(())
    }
    async fn document_get(
        &self,
        req: &GetDocumentRequestDto,
    ) -> Result<GetDocumentResponseDto, ServiceError> {
        print!("Getting document: {}", req.id);

        let request = GetDocumentRequest::new(req.id.clone().into());
        let result = self.repo.get_document(&request).await;
        match result {
            Ok(r) => Ok(GetDocumentResponseDto {
                id: r.id().into(),
                content: r.content().into(),
            }),
            Err(e) => Err(ServiceError::GetDocumentError),
        }
    }
}
