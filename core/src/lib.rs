mod data;
mod domain;
mod outbound;
use crate::domain::editor::models::Document;
use domain::editor::ports::DocumentRepository;
use outbound::{data_store::DataStore, file_system::FileSystem};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
/// The application state available to all request handlers.
struct Node<DR: DocumentRepository> {
    document_repo: Arc<DR>,
}

impl Node<FileSystem> {
    pub fn new() -> Result<Node<FileSystem>, NodeError> {
        let file_system = FileSystem::new();
        let node = Node {
            document_repo: Arc::new(file_system),
        };

        Ok(node)
    }
}

pub enum NodeError {}
