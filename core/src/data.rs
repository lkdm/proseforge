use crate::error::NodeError;
use serde::{Deserialize, Serialize};
use std::fs::{self, File as StdFile, Metadata};
use std::sync::{Arc, Mutex};
use std::{
    io::{self, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

const DOCUMENTATION_STR: &'static str = r#"# Markdown Editor

This is a simple markdown editor that allows you to create, edit, and save markdown files.
"#;

#[derive(Debug, Clone)]
pub(crate) struct ReadResult {
    /// Contains the data read from storage
    data: Vec<u8>,
    /// Contains metadata about the data read from storage
    metadata: Option<Metadata>,
}

#[derive(Clone)]
pub enum StorageMedium {
    File(std::path::PathBuf),
}

/// Persist defines standard methods for persisting data
pub(crate) trait Persist: Send {
    /// Standard method for read operations
    fn read(&self) -> Result<ReadResult, std::io::Error>;
    /// Standard method for write operations
    fn write(&mut self, content: &[u8]) -> Result<(), std::io::Error>;
    /// Standard method for delete operations
    fn delete(&mut self) -> Result<(), std::io::Error>;
}

impl Into<StorageMedium> for PathBuf {
    fn into(self) -> StorageMedium {
        StorageMedium::File(self)
    }
}

impl Persist for StorageMedium {
    fn read(&self) -> Result<ReadResult, std::io::Error> {
        match self {
            StorageMedium::File(path) => {
                let file = StdFile::open(path)?;
                let mut buf_reader = BufReader::new(file);
                let mut content = Vec::new();
                buf_reader.read_to_end(&mut content)?;
                let metadata = fs::metadata(path)?;
                Ok(ReadResult {
                    data: content,
                    metadata: Some(metadata),
                })
            }
        }
    }

    fn write(&mut self, content: &[u8]) -> Result<(), std::io::Error> {
        match self {
            StorageMedium::File(path) => {
                let file = StdFile::create(path)?;
                let mut buf_writer = std::io::BufWriter::new(file);
                buf_writer.write_all(content)?;
                buf_writer.flush()?;
                Ok(())
            }
        }
    }

    fn delete(&mut self) -> Result<(), std::io::Error> {
        match self {
            StorageMedium::File(path) => {
                fs::remove_file(path)?;
                Ok(())
            }
        }
    }
}

/// UserContent defines standard methods for managing user content
pub trait UserContent {
    /// Create a blank instance of content
    fn new() -> Self;
    /// Load content from storage
    fn load(&mut self) -> Result<(), NodeError>;
    /// Save content to storage
    fn save(&mut self) -> Result<(), NodeError>;
    /// Update content in memory
    fn update_content(&mut self, content: String);
    /// Retrieve content from memory
    fn get_content(&self) -> &str;
    /// Set the storage location for the content
    fn set_save_location(&mut self, storage: StorageMedium);
    /// Retrieve the storage location for the content
    fn get_save_location(&self) -> Option<StorageMedium>;
    /// Check if the content has been modified since last read or write
    fn has_unsaved_changes(&self) -> bool;
}

/// Document represents a markdown document
#[derive(Clone)]
pub struct Document {
    /// Contains the content of the document
    content: String,
    /// The storage location for the document, this can be a file or other storage type
    storage: StorageMedium,
    /// If the content has been modified since read or write
    modified: bool,
}

impl UserContent for Document {
    fn new() -> Self {
        Document {
            content: String::from(""),
            storage: StorageMedium::File(PathBuf::new()),
            modified: false,
        }
    }

    fn load(&mut self) -> Result<(), NodeError> {
        let result = self.storage.read()?;
        self.content =
            String::from_utf8(result.data).map_err(|e| NodeError::FromUtf8Error { source: e })?;
        Ok(())
    }

    fn save(&mut self) -> Result<(), NodeError> {
        self.storage.write(self.content.as_bytes())?;
        self.modified = false;
        Ok(())
    }

    fn update_content(&mut self, content: String) {
        self.content = content;
        self.modified = true;
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    fn set_save_location(&mut self, storage: StorageMedium) {
        self.storage = storage;
    }

    fn get_save_location(&self) -> Option<StorageMedium> {
        Some(self.storage.clone())
    }

    fn has_unsaved_changes(&self) -> bool {
        self.modified
    }
}

pub struct DocumentBuilder(Document);

impl DocumentBuilder {
    pub fn new() -> DocumentBuilder {
        DocumentBuilder(Document::new())
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.0.set_save_location(StorageMedium::File(path));
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.0.update_content(content);
        self
    }

    pub fn load(mut self) -> Result<Self, NodeError> {
        self.0.load()?;
        Ok(self)
    }

    /// Lock the editor and replace its content with the new document
    pub fn commit(self, editor: &Arc<Mutex<Document>>) -> Result<Document, NodeError> {
        // Lock the editor and replace its content with the new document
        let mut editor_lock = editor.lock().map_err(|_| NodeError::LockError)?;
        let document_copy = self.0.clone();
        *editor_lock = self.0; // Replace the existing document with the new one
        Ok(document_copy)
    }
}
