mod data;
mod domain;
mod outbound;
use crate::domain::editor::models::Document;
use domain::editor::ports::DocumentRepository;
use outbound::file_system::FileSystem;
use std::sync::{Arc, Mutex};

pub struct Node {
    document: Arc<Document>,
}

#[derive(Debug, Clone)]
/// The application state available to all request handlers.
struct AppState<DR: DocumentRepository> {
    document_repo: Arc<DR>,
}

impl Node {
    pub fn new() -> Result<Node, NodeError> {
        let node = Node {
            document: Arc::new(Document::default()),
        };

        Ok(node)
    }
}
