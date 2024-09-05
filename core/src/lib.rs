pub mod editor;
pub mod project;
pub mod types;
use editor::services::document::DocumentService;
use project::services::project::DesktopService as ProjectService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone)]
/// Represents the central application state, aggregating various repositories.
///
/// Also provides high-level handlers for dealing with the repositories.
pub struct Node<
    DS: DocumentService,
    PS: ProjectService,
    // CFG: ConfigRepository,
> {
    pub document_service: Arc<DS>,
    pub project_service: Arc<PS>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NodeError {
    #[error("Error with repository.")]
    RepositoryError,
}

impl<DS: DocumentService, PS: ProjectService> Node<DS, PS> {
    pub fn new(document_service: DS, project_service: PS) -> Result<Self, NodeError> {
        let node = Node {
            project_service: Arc::new(project_service),
            document_service: Arc::new(document_service),
        };
        return Ok(node);
    }
}
