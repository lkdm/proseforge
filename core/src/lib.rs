pub mod config;
pub mod data;
pub mod error;
pub mod event;
use tokio::sync::broadcast::{channel, Sender};

use config::Config;
use data::{ContentRepository, TextFile};
use error::NodeError;
use std::sync::{Arc, Mutex};

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

    // Updates the content in memory.
    pub fn handle_update_content(&self, content: String) -> Result<(), NodeError> {
        let mut editor = self.editor.lock().unwrap();
        editor.update_content(content);
        Ok(())
    }

    // Create new document.
    pub fn handle_new_document(&self) -> Result<(), NodeError> {
        let mut editor = self.editor.lock().unwrap();
        let new = TextFile::new(None);
        *editor = new;
        Ok(())
    }

    // Save the document.
    pub fn handle_save(&self) -> Result<(), NodeError> {
        let editor = self.editor.lock().unwrap();
        editor.save()
    }
}
