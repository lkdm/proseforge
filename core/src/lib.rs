pub mod config;
mod data;
pub mod editor;
use editor::ports::DocumentRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone)]
/// The application state available to all request handlers.
pub struct Node<DR: DocumentRepository> {
    pub document_repo: Arc<DR>,
    // pub document_ds: Arc<IMDR>,
    // config: Arc<CR>
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NodeError {
    #[error("Error with repository.")]
    RepositoryError,
}
