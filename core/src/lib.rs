pub mod config;
pub mod error;
pub mod md;

use config::Config;
use error::CoreError;
use md::DataStorage;
use md::MarkdownFile;
use std::sync::Arc;

pub struct Node {
    pub editor: Arc<MarkdownFile>,
    pub config: Arc<Config>,
}

impl Node {
    pub fn new() -> Result<Node, CoreError> {
        let node = Node {
            editor: Arc::new(MarkdownFile::default()),
            config: Arc::new(Config::default()),
        };
        Ok(node)
    }
}

#[derive(Debug)]
pub enum NodeError {}
