pub mod config;
pub mod data;
pub mod error;

use config::Config;
use data::{ContentRepository, TextFile};
use error::NodeError;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

pub struct Node {
    pub editor: Arc<Mutex<TextFile>>,
    pub config: Arc<Config>,
}

impl Node {
    pub fn new() -> Result<Node, NodeError> {
        let node = Node {
            editor: Arc::new(Mutex::new(TextFile::default())),
            config: Arc::new(Config::default()),
        };

        Ok(node)
    }
}
