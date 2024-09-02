pub mod editor;
pub mod types;
use editor::document::services::DocumentService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone)]
/// Represents the central application state, aggregating various repositories.
///
/// Also provides high-level handlers for dealing with the repositories.
pub struct Node<
    DS: DocumentService,
    // CFG: ConfigRepository,
> {
    pub document_service: Arc<DS>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NodeError {
    #[error("Error with repository.")]
    RepositoryError,
}

impl<DS: DocumentService> Node<DS> {
    pub fn new(project_service: DS) -> Result<Self, NodeError> {
        let node = Node {
            document_service: Arc::new(project_service),
        };
        return Ok(node);
    }
}
