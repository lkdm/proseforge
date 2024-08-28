mod data;
pub mod editor;
pub mod node;
use editor::ports::{
    component::ComponentRepository, content::ContentRepository, project::ProjectRepository,
};
use node::ports::NodeConfigRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone)]
/// The application state available to all request handlers.
pub struct Node<
    PRR: ProjectRepository,
    CMP: ComponentRepository,
    COR: ContentRepository,
    CFG: NodeConfigRepository,
> {
    pub document_repo: Arc<PRR>,
    pub component_repo: Arc<CMP>,
    pub content_repo: Arc<COR>,
    pub config_repo: Arc<CFG>,
    // Todo: Implement config repo
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NodeError {
    #[error("Error with repository.")]
    RepositoryError,
}

impl<
        P: ProjectRepository,
        C: ComponentRepository,
        CO: ContentRepository,
        CFG: NodeConfigRepository,
    > Node<P, C, CO, CFG>
{
    pub fn new(
        project_repo: P,
        component_repo: C,
        content_repo: CO,
        config_repo: CFG,
    ) -> Result<Self, NodeError> {
        let node = Node {
            document_repo: Arc::new(project_repo),
            component_repo: Arc::new(component_repo),
            content_repo: Arc::new(content_repo),
            config_repo: Arc::new(config_repo),
        };
        return Ok(node);
    }
}
