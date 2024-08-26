mod data;
pub mod domain;
mod inbound;
mod outbound;
use crate::domain::editor::models::Document;
use domain::editor::ports::{DocumentRepository, InMemoryDocumentRepository};
use inbound::ContentRepository;
use outbound::{data_store::DataStore, file_system::FileSystem};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Clone)]
/// The application state available to all request handlers.
pub struct Node<DR: DocumentRepository, IMDR: InMemoryDocumentRepository> {
    pub document_repo: Arc<DR>,
    pub document_ds: Arc<IMDR>,
    // config: Arc<CR>
}

type DocumentDataStore = DataStore<Document>;

pub type TauriNode = Node<FileSystem, DocumentDataStore>;

impl TauriNode {
    pub fn new() -> Result<Node<FileSystem, DocumentDataStore>, NodeError> {
        let file_system = FileSystem::new();
        let node = Node {
            document_repo: Arc::new(file_system),
            document_ds: Arc::new(DataStore::new(Document::builder().build())),
        };

        Ok(node)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NodeError {
    #[error("Error with repository.")]
    RepositoryError,
}
