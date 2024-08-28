mod data;
pub mod editor;
pub mod node;
use editor::ports::{
    component::ComponentRepository, content::ContentRepository, project::ProjectRepository,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone)]
/// The application state available to all request handlers.
pub struct Node<PRR: ProjectRepository, CMP: ComponentRepository, COR: ContentRepository> {
    pub document_repo: Arc<PRR>,
    pub component_repo: Arc<CMP>,
    pub content_repo: Arc<COR>,
    // Todo: Implement config repo
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NodeError {
    #[error("Error with repository.")]
    RepositoryError,
}

impl<DR: DocumentRepository> Node<DR> {
    pub fn new(document_repo: DR) -> Result<Self, NodeError> {
        let node = Node {
            document_repo: Arc::new(document_repo),
        };
        // TODO: Return Arc of Node
        return Ok(node);
    }
}
