use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult};
use std::{future::Future, path::PathBuf};

use proseforge_core::project::{
    models::project::{CreateProjectError, GetProjectError},
    ports::project::FileSystemProjectRepository,
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct FileSystem {
    user_dir: PathBuf,
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

impl FileSystemProjectRepository for FileSystem {
    fn new_prosefile(&self) -> Result<(), CreateProjectError> {
        let user_dir = self.user_dir.clone();
        let path = FileDialog::new()
            .set_directory(user_dir)
            .save_file()
            .map(|file_handle| file_handle.to_path_buf());
        if let Some(path) = path {
            path.with_extension("prose");
            self.create_file(path);
        }
        Ok(())
    }

    fn load_prosefile(&self) -> Result<(), GetProjectError> {
        let user_dir = self.user_dir.clone();
        let user_dir = match user_dir {
            Some(user_dir) => user_dir,
            None => std::env::current_dir().unwrap(),
        };
        FileDialog::new()
            .set_directory(user_dir)
            .pick_file()
            .map(|file_handle| file_handle.to_path_buf()) // Convert to PathBuf if Some
    }
}
