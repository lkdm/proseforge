pub mod data;
pub mod error;
pub mod event;
pub mod node;
use tokio::sync::broadcast::{channel, Sender};

use data::{DataStore, Document, FileStore};
use error::NodeError;
use node::config::*;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct Node {
    pub data_store: Arc<DataStore>,
    pub document: Arc<Document>,
}

impl Node {
    pub fn new() -> Result<Node, NodeError> {
        let path = PathBuf::default(); // TODO: Get the path from the config
        let node = Node {
            data_store: Arc::new(FileStore::new()),
            document: Arc::new(Document::new()),
        };

        Ok(node)
    }

    // pub fn handle_update_content(&self, content: String) -> Result<(), NodeError> {
    //     let mut editor = self.editor.lock().unwrap();
    //     editor.update_content(content);
    //     Ok(())
    // }

    // pub fn handle_new_document(&self, force: bool) -> Result<(), NodeError> {
    //     let editor = self.editor.lock().unwrap();
    //     if !force && editor.has_unsaved_changes() {
    //         return Err(NodeError::FileNotSaved);
    //     }
    //     let new = Document::new();
    //     let mut editor = editor;
    //     *editor = new;
    //     Ok(())
    // }

    // pub fn handle_save(&self) -> Result<(), NodeError> {
    //     let mut editor = self.editor.lock().unwrap();
    //     editor.save().map_err(|_| NodeError::NoSavePath)
    // }

    // pub fn handle_load(&self) -> Result<String, NodeError> {
    //     let mut editor = self.editor.lock().unwrap();
    //     editor.load()?;
    //     Ok(editor.get_content().into())
    // }
}
