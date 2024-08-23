use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use crate::error::CoreError;

const DOCUMENTATION_STR: &'static str = r#"# Markdown Editor

This is a simple markdown editor that allows you to create, edit, and save markdown files.
"#;

pub struct MarkdownRecord {
    content: String,
    id: Option<i32>,
}

#[derive(Clone)]
pub struct MarkdownFile {
    content: String,
    path: Option<PathBuf>,
}

pub trait DataStorage {
    // Writes the content to the file
    fn write(&self) -> Result<(), CoreError>;
    // Reads the content from the file
    fn read(&self) -> Result<String, CoreError>;

    fn content(&self) -> String;
}

impl DataStorage for MarkdownFile {
    fn write(&self) -> Result<(), CoreError> {
        let file = match &self.path {
            Some(path) => File::create(path)?,
            None => return Err(CoreError::no_save_path()),
        };
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(self.content.as_bytes())?;
        buf_writer.flush()?;
        Ok(())
    }
    fn read(&self) -> Result<String, CoreError> {
        let file = File::open(self.path.as_ref().unwrap())?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content)?;
        Ok(content)
    }
    fn content(&self) -> String {
        self.content.clone()
    }
}

impl MarkdownFile {
    pub fn new(content: Option<String>, path: Option<PathBuf>) -> Self {
        match content {
            Some(content) => Self { content, path },
            None => Self {
                content: String::new(),
                path,
            },
        }
    }
}

impl Default for MarkdownFile {
    fn default() -> Self {
        Self {
            content: String::from(DOCUMENTATION_STR),
            path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_markdown_file() {
        let path = env::temp_dir().join("test.md");
        let markdown = MarkdownFile::new(Some(String::from("Hello, world!")), Some(path.clone()));
        // Check if is an error
        assert!(markdown.write().is_ok());

        // Load from reference
        let loaded_markdown = markdown.read();
        assert!(loaded_markdown.is_ok());
    }

    #[test]
    fn test_markdown_file_no_path() {
        let markdown = MarkdownFile {
            content: String::from("Hello, world!"),
            path: None,
        };
        // Ensure there is an error
        assert!(markdown.write().is_err());
    }
}
