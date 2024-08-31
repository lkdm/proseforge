pub mod api;
pub mod features;
pub mod node;

use features::project::{
    ports::{ComponentRepository, DocumentRepository, ProjectRepository},
    services::ProjectService,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone)]
/// Represents the central application state, aggregating various repositories.
///
/// Also provides high-level handlers for dealing with the repositories.
pub struct Node<
    PR: ProjectRepository,
    PS: ProjectService,
    // CFG: ConfigRepository,
> {
    pub project_repo: Arc<PR>,
    pub project_service: Arc<PS>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NodeError {
    #[error("Error with repository.")]
    RepositoryError,
}

impl<PR: ProjectRepository, PS: ProjectService> Node<PR, PS> {
    pub fn new(project_repo: PR, project_service: PS) -> Result<Self, NodeError> {
        let node = Node {
            project_repo: Arc::new(project_repo),
            project_service: Arc::new(project_service),
        };
        return Ok(node);
    }
}
