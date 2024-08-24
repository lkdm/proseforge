pub mod config;
pub mod error;
pub mod md;

use config::Config;
use error::NodeError;
use md::{ContentRepository, TextFile};
use std::sync::Arc;

pub struct Node {
    pub editor: Arc<TextFile>,
    pub config: Arc<Config>,
}

impl Node {
    pub fn new() -> Result<Node, NodeError> {
        let node = Node {
            editor: Arc::new(TextFile::default()),
            config: Arc::new(Config::default()),
        };
        Ok(node)
    }
}
