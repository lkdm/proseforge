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
    repo: R,
}

pub trait ProjectService {
    fn create_document(&self, req: &NewDocumentRequest) -> Result<(), ServiceError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewDocumentRequest {
    parent_id: String,
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
    fn create_document(&self, req: &NewDocumentRequest) -> Result<(), ServiceError> {
        // TODO: This needs to take a parent component_id to know where to put the document.
        // TODO: Create a component
        // TODO: Create a document
        Ok(())
    }
}
