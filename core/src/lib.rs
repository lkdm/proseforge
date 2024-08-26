mod data;
use crate::data::{Document, NodeError};
use std::sync::{Arc, Mutex};

pub struct Node {
    document: Arc<Document>,
}

impl Node {
    pub fn new() -> Result<Node, NodeError> {
        let node = Node {
            document: Arc::new(Document::default()),
        };

        Ok(node)
    }
}
