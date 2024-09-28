use std::{future::Future, path::PathBuf};
use thiserror::Error;

/// File System adapter
///
/// Provides access to the file system for the Proseforge application.
#[derive(Debug, Clone)]
pub struct FileSystem;

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
    fn read_file(&self, path: PathBuf) -> Result<Vec<u8>, FileSystemError> {
        std::fs::read(path).map_err(FileSystemError::IoError)
    }

    fn write_file(&self, path: PathBuf, content: &[u8]) -> Result<(), FileSystemError> {
        std::fs::write(path, content).map_err(FileSystemError::IoError)
    }

    fn create_file(&self, path: PathBuf) -> Result<(), FileSystemError> {
        std::fs::File::create(path).map_err(FileSystemError::IoError)?;
        Ok(())
    }

    fn to_utf8(&self, content: Vec<u8>) -> Result<String, FileSystemError> {
        String::from_utf8(content).map_err(FileSystemError::Utf8ConversionError)
    }
}

impl FileSystem {
    async fn new_prosefile(&self, path: PathBuf) -> Result<(), FileSystemError> {
        path.with_extension("prose");
        self.create_file(path)?;
        Ok(())
    }
}
