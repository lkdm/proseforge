pub mod features;
pub mod node;

use features::project::ports::{ComponentRepository, DocumentRepository, ProjectRepository};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone)]
/// Represents the central application state, aggregating various repositories.
///
/// Also provides high-level handlers for dealing with the repositories.
pub struct Node<
    PR: ProjectRepository,
    CR: ComponentRepository,
    DR: DocumentRepository,
    // CFG: ConfigRepository,
> {
    pub project_repo: Arc<PR>,
    pub component_repo: Arc<CR>,
    pub document_repo: Arc<DR>,
    // pub config_repo: Arc<CFG>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NodeError {
    #[error("Error with repository.")]
    RepositoryError,
}

impl<
        PR: ProjectRepository,
        CR: ComponentRepository,
        DR: DocumentRepository,
        // CFG: NodeConfigRepository,
    > Node<PR, CR, DR>
{
    pub fn new(
        project_repo: PR,
        component_repo: CR,
        document_repo: DR,
        // config_repo: CFG,
    ) -> Result<Self, NodeError> {
        let node = Node {
            project_repo: Arc::new(project_repo),
            component_repo: Arc::new(component_repo),
            document_repo: Arc::new(document_repo),
            // config_repo: Arc::new(config_repo),
        };
        return Ok(node);
    }
}
