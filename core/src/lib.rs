pub mod error;
pub mod md;

use error::CoreError;
use md::DataStorage;
use md::MarkdownFile;
use std::sync::Arc;

pub struct Node {
    pub editor: Arc<MarkdownFile>,
}

impl Node {
    pub fn new() -> Result<Node, CoreError> {
        let node = Node {
            editor: Arc::new(MarkdownFile::default()),
        };
        Ok(node)
    }
}

#[derive(Debug)]
pub enum NodeError {}
