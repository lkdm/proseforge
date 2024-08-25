pub mod config;
pub mod data;
pub mod error;
pub mod event;
use tokio::sync::broadcast::{channel, Sender};

use config::Config;
use data::{Document, UserContent};
use error::NodeError;
use std::sync::{Arc, Mutex};

pub struct Node {
    pub editor: Arc<Mutex<Document>>,
    pub config: Arc<Config>,
}

impl Node {
    pub fn new() -> Result<Node, NodeError> {
        let node = Node {
            editor: Arc::new(Mutex::new(Document::new())),
            config: Arc::new(Config::default()),
        };

        Ok(node)
    }

    pub fn handle_update_content(&self, content: String) -> Result<(), NodeError> {
        let mut editor = self.editor.lock().unwrap();
        editor.update_content(content);
        Ok(())
    }

    pub fn handle_new_document(&self, force: bool) -> Result<(), NodeError> {
        let editor = self.editor.lock().unwrap();
        if !force && editor.has_unsaved_changes() {
            return Err(NodeError::FileNotSaved);
        }
        let new = Document::new();
        let mut editor = editor;
        *editor = new;
        Ok(())
    }

    pub fn handle_save(&self) -> Result<(), NodeError> {
        let mut editor = self.editor.lock().unwrap();
        editor.save().map_err(|_| NodeError::NoSavePath)
    }

    // pub fn handle_load(&self) -> Result<String, NodeError> {
    //     let mut editor = self.editor.lock().unwrap();
    //     editor.load()?;
    //     Ok(editor.get_content().into())
    // }
}
