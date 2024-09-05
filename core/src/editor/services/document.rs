use std::{future::Future, sync::Arc};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    editor::{
        models::document::{CreateDocumentRequest, GetDocumentRequest, UpdateDocumentRequest},
        ports::document::DocumentRepository,
    },
    types::Id,
};

///
/// Service contains functions that more directly relate to the business logic of the application.
/// A service function may call multiple repository functions to accomplish its task.
///
/// It may also publish events or perform other side effects.
///
#[derive(Debug, Clone)]
pub struct Service<R>
where
    R: DocumentRepository,
    // M: AuthorMetrics,
    // N: AuthorNotifier,
{
    repo: R,
    // metrics: M,
    // notifier: N,
}

impl<R> Service<R>
where
    R: DocumentRepository,
    // M: SomeMetricsTrait,
    // N: SomeNotifierTrait,
{
    pub fn new(repo: R) -> Self {
        Service { repo }
    }
}

pub trait DocumentService {
    fn document_create(
        &self,
        req: &CreateDocumentRequestDto,
    ) -> impl Future<Output = Result<Id, ServiceError>> + Send;
    fn document_update(
        &self,
        req: &UpdateDocumentRequestDto,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send;
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
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentRequestDto {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentResponseDto {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectRequestDto {
    pub title: String,
    pub kind: String,
}

#[derive(Debug, Serialize, Error)]
#[error("Application error")]
pub struct ServiceError(String);
impl From<anyhow::Error> for ServiceError {
    fn from(err: anyhow::Error) -> Self {
        ServiceError(err.to_string())
    }
}

impl<R> DocumentService for Service<R>
where
    R: DocumentRepository,
{
    /// Create a new document in the project.
    /// This creates a new component and document in the project, and returns the component id.
    async fn document_create(&self, req: &CreateDocumentRequestDto) -> Result<Id, ServiceError> {
        // TODO: This needs to take a parent component_id to know where to put the document.
        let project_id = req.project_id.clone();
        let request = CreateDocumentRequest::new(project_id.into());
        self.repo
            .create_document(&request)
            .await
            .map(Into::into)
            .map_err(|e| ServiceError(e.to_string()))
    }
    async fn document_update(&self, req: &UpdateDocumentRequestDto) -> Result<(), ServiceError> {
        let request = UpdateDocumentRequest::new(req.id.clone().into(), req.content.clone().into());
        self.repo
            .update_document(&request)
            .await
            .map_err(|e| ServiceError(e.to_string()))
    }
    async fn document_get(
        &self,
        req: &GetDocumentRequestDto,
    ) -> Result<GetDocumentResponseDto, ServiceError> {
        print!("Getting document: {}", req.id);

        let request = GetDocumentRequest::new(req.id.clone().into());
        self.repo
            .get_document(&request)
            .await
            .map(|r| GetDocumentResponseDto {
                id: r.id().into(),
                content: r.content().into(),
            })
            .map_err(|e| ServiceError(e.to_string()))
    }
}
