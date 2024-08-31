pub mod component;
pub mod document;
pub mod project;
use derive_more::derive::{AsRef, From};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid title.")]
pub struct TitleError(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, AsRef)]
pub struct Title(String);

impl Title {
    pub fn new(title: String) -> Result<Self, TitleError> {
        // Strip leading and trailing whitespace
        let trimmed_title = title.trim();

        // Replace multiple consecutive spaces with a single space
        let normalized_title = trimmed_title
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        // Check if the title is empty after normalization
        if normalized_title.is_empty() {
            return Err(TitleError("Title cannot be empty".to_string()));
        }

        // Check if the title length is within the limit
        if normalized_title.len() > 254 {
            return Err(TitleError(
                "Title exceeds maximum length of 254 characters".to_string(),
            ));
        }

        // Validate allowed characters (alphanumeric, spaces, underscores, and dashes)
        if !normalized_title
            .chars()
            .all(|c| c.is_alphanumeric() || c.is_whitespace() || c == '_' || c == '-')
        {
            return Err(TitleError("Title contains invalid characters".to_string()));
        }

        Ok(Title(normalized_title))
    }
}

impl Default for Title {
    fn default() -> Self {
        Title("Untitled".to_string())
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_title_new() {
        let title = Title::new("Hello World".to_string());
        assert_eq!(title.unwrap().0, "Hello World");
    }

    #[test]
    fn test_title_new_empty() {
        let title = Title::new("".to_string());
        assert!(title.is_err());
    }

    #[test]
    fn test_title_new_whitespace() {
        let title = Title::new("  ".to_string());
        assert!(title.is_err());
    }

    #[test]
    fn test_title_new_whitespace_trimmed() {
        let title = Title::new("  Hello World  ".to_string());
        assert_eq!(title.unwrap().0, "Hello World");
    }

    #[test]
    fn test_title_new_whitespace_normalized() {
        let title = Title::new("Hello    World!".to_string());
        assert_eq!(title.unwrap().0, "Hello World");
    }
}
