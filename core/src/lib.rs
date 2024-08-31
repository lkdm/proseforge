pub mod features;
pub mod node;

use features::project::services::ProjectService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone)]
/// Represents the central application state, aggregating various repositories.
///
/// Also provides high-level handlers for dealing with the repositories.
pub struct Node<
    PS: ProjectService,
    // CFG: ConfigRepository,
> {
    pub project_service: Arc<PS>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NodeError {
    #[error("Error with repository.")]
    RepositoryError,
}

impl<PS: ProjectService> Node<PS> {
    pub fn new(project_service: PS) -> Result<Self, NodeError> {
        let node = Node {
            project_service: Arc::new(project_service),
        };
        return Ok(node);
    }
}
