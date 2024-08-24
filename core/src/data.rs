use crate::error::NodeError;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

const DOCUMENTATION_STR: &'static str = r#"# Markdown Editor

This is a simple markdown editor that allows you to create, edit, and save markdown files.
"#;

#[derive(Debug, Clone, PartialEq)]
pub enum SaveLocation {
    PathBuf(PathBuf),
    String(String),
}

impl PartialEq<PathBuf> for SaveLocation {
    fn eq(&self, other: &PathBuf) -> bool {
        match self {
            SaveLocation::PathBuf(p) => p == other,
            SaveLocation::String(s) => PathBuf::from(s) == *other,
        }
    }
}

impl From<String> for SaveLocation {
    fn from(s: String) -> SaveLocation {
        SaveLocation::String(s)
    }
}

impl From<PathBuf> for SaveLocation {
    fn from(p: PathBuf) -> SaveLocation {
        SaveLocation::PathBuf(p)
    }
}

// ContentRepository trait
// An interface designed to allow for the loading and saving of content across different storage types.
// Examples: File system, API, or database
pub trait ContentRepository {
    // Loads data into memory
    fn load(&mut self) -> Result<(), NodeError>;

    // Saves the current content to storage
    fn save(&mut self) -> Result<(), NodeError>;

    fn new(location: Option<SaveLocation>) -> Self
    where
        Self: Sized;

    fn update_content(&mut self, content: String);

    // Retrieve the current content
    fn get_content(&self) -> String;

    // Sets the save location or identifier for storage
    fn set_save_location<T>(&mut self, path: T)
    where
        T: Into<SaveLocation>;

    // Retrieves the save location or identifier for storage
    fn get_save_location(&self) -> Option<SaveLocation>;

    // Check for unsaved changes
    fn has_unsaved_changes(&self) -> bool;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TextFile {
    path: Option<PathBuf>,
    content: String,
    modified: bool,
}

impl Default for TextFile {
    fn default() -> Self {
        TextFile {
            content: String::from(DOCUMENTATION_STR),
            path: None,
            modified: false,
        }
    }
}

impl ContentRepository for TextFile {
    fn load(&mut self) -> Result<(), NodeError> {
        let file = File::open(self.path.as_ref().unwrap())?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content)?;
        self.content = content;
        Ok(())
    }
    fn save(&mut self) -> Result<(), NodeError> {
        let file = match &self.path {
            Some(path) => File::create(path)?,
            None => return Err(NodeError::NoSavePath),
        };
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(self.content.as_bytes())?;
        buf_writer.flush()?;
        self.modified = false;
        Ok(())
    }
    fn new(location: Option<SaveLocation>) -> Self {
        let path = match location {
            Some(SaveLocation::PathBuf(p)) => Some(p),
            Some(SaveLocation::String(s)) => Some(PathBuf::from(s)),
            None => None,
        };
        TextFile {
            path,
            content: String::from(""),
            modified: false,
        }
    }
    fn update_content(&mut self, content: String) {
        self.content = content;
        self.modified = true;
    }
    fn get_content(&self) -> String {
        self.content.clone()
    }
    fn set_save_location<T>(&mut self, path: T)
    where
        T: Into<SaveLocation>,
    {
        self.path = match path.into() {
            SaveLocation::PathBuf(p) => Some(p),
            SaveLocation::String(s) => Some(PathBuf::from(s)),
        };
    }

    fn get_save_location(&self) -> Option<SaveLocation> {
        self.path.clone().map(SaveLocation::PathBuf)
    }

    fn has_unsaved_changes(&self) -> bool {
        self.modified
    }
}

pub fn open_file_dialog() -> Result<PathBuf, NodeError> {
    let dir = PathBuf::from("/");
    let file_dialog_res = FileDialog::new().set_directory(dir).pick_file();
    if let Some(file_handle) = file_dialog_res {
        Ok(file_handle.to_path_buf())
    } else {
        Err(NodeError::NoOpenPath)
    }
}
pub fn open_file_save_dialog() -> Result<PathBuf, NodeError> {
    let dir = PathBuf::from("/");
    let file_dialog_res = FileDialog::new().set_directory(dir).save_file();
    if let Some(file_handle) = file_dialog_res {
        Ok(file_handle.to_path_buf())
    } else {
        Err(NodeError::NoSavePath)
    }
}

pub fn open_save_warning_dialog() -> bool {
    let dialog = MessageDialog::new()
        .set_title("Warning")
        .set_description("You have unsaved changes. Do you want to save?")
        .set_buttons(MessageButtons::YesNo);
    dialog.show() == MessageDialogResult::Yes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn text_textfile() {
        let mut md = TextFile::new();
        assert_eq!(md.get_content(), "");

        md.update_content(String::from("Hello, world!"));
        assert_eq!(md.get_content(), "Hello, world!");

        let path = env::temp_dir().join("test.md");
        md.set_save_location(path);
        assert_eq!(
            md.get_save_location(),
            Some(SaveLocation::PathBuf(env::temp_dir().join("test.md")))
        );
        assert!(md.save().is_ok());

        md.update_content(String::from("Goodbye, world!"));
        assert_eq!(md.get_content(), "Goodbye, world!");
        assert!(md.load().is_ok());
        assert_eq!(md.get_content(), "Hello, world!");
    }
}
