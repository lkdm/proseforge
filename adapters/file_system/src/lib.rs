pub mod dialogs;

use std::path::{Path, PathBuf};
use thiserror::Error;

use pf_core::editor::{
    models::{
        CreateDocumentError, CreateDocumentRequest, Document, GetDocumentError, GetDocumentRequest,
        UpdateDocumentError, UpdateDocumentRequest,
    },
    ports::DocumentRepository,
};

#[derive(Debug, Clone)]
pub struct FileSystem {
    path: Option<PathBuf>,
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
        Self { path: None }
    }

    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: Some(path.as_ref().to_path_buf()),
        }
    }

    fn get_path(&self) -> Result<PathBuf, FileSystemError> {
        self.path.clone().ok_or(FileSystemError::NoFilePath)
    }

    fn read_file(&self) -> Result<Vec<u8>, FileSystemError> {
        let path = self.get_path()?;
        if !path.is_file() {
            return Err(FileSystemError::NotAFile(path.clone()));
        }
        std::fs::read(path).map_err(FileSystemError::IoError)
    }

    fn write_file(&self, content: &[u8]) -> Result<(), FileSystemError> {
        let path = self.get_path()?;
        std::fs::write(path, content).map_err(FileSystemError::IoError)
    }

    fn create_file(&self) -> Result<(), FileSystemError> {
        let path = self.get_path()?;
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
        self.create_file();
        self.write_file(_req.content().as_bytes());

        let document = Document::builder()
            .with_content(_req.content().clone())
            .saved_now()
            .build();

        Ok(document)
    }

    async fn get_document(&self, _req: &GetDocumentRequest) -> Result<Document, GetDocumentError> {
        let content = self.read_file().unwrap();
        let content = self.to_utf8(content).unwrap();
        let document = Document::builder()
            .with_content(content.into())
            .saved_now()
            .build();

        Ok(document)
    }

    async fn update_document(
        &self,
        _req: &UpdateDocumentRequest,
    ) -> Result<(), UpdateDocumentError> {
        self.write_file(_req.content().as_bytes());
        Ok(())
    }
}
