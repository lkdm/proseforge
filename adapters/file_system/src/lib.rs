pub mod dialogs;
mod store;
use crate::store::InMemoryPathStore;
use dialogs::request_save_path_dialog;
use proseforge_core::editor::{
    models::{
        CreateDocumentError, CreateDocumentRequest, Document, DocumentId, GetDocumentError,
        GetDocumentRequest, UpdateDocumentError, UpdateDocumentRequest,
    },
    ports::DocumentRepository,
};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct FileSystem {
    documents: InMemoryPathStore<DocumentId>,
}

#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("No file path provided")]
    NoFilePath,
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to convert file content to UTF-8: {0}")]
    Utf8ConversionError(#[from] std::string::FromUtf8Error),
    #[error("Path {0} is not a file")]
    NotAFile(PathBuf),
    #[error("Unknown error")]
    UnknownError,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            documents: InMemoryPathStore::default(),
        }
    }

    fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, FileSystemError> {
        std::fs::read(path).map_err(FileSystemError::IoError)
    }

    fn write_file<P: AsRef<Path>>(&self, path: P, content: &[u8]) -> Result<(), FileSystemError> {
        std::fs::write(path, content).map_err(FileSystemError::IoError)
    }

    fn create_file<P: AsRef<Path>>(&self, path: P) -> Result<(), FileSystemError> {
        std::fs::File::create(path).map_err(FileSystemError::IoError)?;
        Ok(())
    }

    fn to_utf8(&self, content: Vec<u8>) -> Result<String, FileSystemError> {
        String::from_utf8(content).map_err(FileSystemError::Utf8ConversionError)
    }
}

impl DocumentRepository for FileSystem {
    async fn create_document(
        &self,
        _req: &CreateDocumentRequest,
    ) -> Result<Document, CreateDocumentError> {
        let document = Document::builder()
            .with_content(_req.content().clone())
            .generate_id()
            .saved_now()
            .build();
        // TODO Cancel if no path is provided
        let path = request_save_path_dialog(None).expect("Failed to get save path");
        let id = document.id();
        // Borrow a mutable reference to self
        self.documents.insert(id, path.clone());
        self.create_file(path.clone());
        self.write_file(path, _req.content().as_bytes());
        Ok(document)
    }

    async fn get_document(&self, _req: &GetDocumentRequest) -> Result<Document, GetDocumentError> {
        unimplemented!()
        // let content = self.read_file().unwrap();
        // let content = self.to_utf8(content).unwrap();
        // let document = Document::builder()
        //     .with_content(content.into())
        //     .saved_now()
        //     .build();

        // Ok(document)
    }

    async fn update_document(
        &self,
        _req: &UpdateDocumentRequest,
    ) -> Result<(), UpdateDocumentError> {
        let id = _req.id();
        let path = self.documents.get(&id).unwrap();
        self.write_file(path, _req.content().as_bytes());
        Ok(())
    }
}
