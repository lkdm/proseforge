use chrono::{DateTime, Utc};
use derive_more::derive::{AsRef, Constructor, Deref, Display, FromStr};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref, Display)]
pub struct Timestamp(DateTime<Utc>);

impl Default for Timestamp {
    fn default() -> Self {
        Self(Utc::now())
    }
}

// #[derive(
//     Debug, Clone, Display, PartialOrd, Ord, PartialEq, Eq, AsRef, Deref, Constructor, FromStr,
// )]
// pub struct Content(String);

// impl Default for Content {
//     fn default() -> Self {
//         Self(String::new())
//     }
// }

// impl Content {
//     pub fn is_empty(&self) -> bool {
//         self.0.is_empty()
//     }
//     pub fn word_count(&self) -> usize {
//         self.0.split_whitespace().count()
//     }
//     pub fn char_count(&self) -> usize {
//         self.0.chars().count()
//     }
// }

// #[derive(Debug, Clone, Display, PartialOrd, Ord, PartialEq, Eq, AsRef, Deref)]
// pub struct Title(String);

// impl Default for Title {
//     fn default() -> Self {
//         Self(String::from("Untitled"))
//     }
// }

// #[derive(Debug, Error)]
// pub enum TitleError {
//     #[error("Title cannot be empty")]
//     Empty,
//     #[error("Title is too short (minimum {min} characters)")]
//     TooShort { min: usize },
//     #[error("Title is too long (maximum {max} characters)")]
//     TooLong { max: usize },
//     #[error("Title cannot contain newline characters")]
//     ContainsNewline,
//     #[error("Title contains invalid characters")]
//     InvalidCharacters,
// }

// impl Title {
//     pub fn new(title: &str) -> Result<Self, TitleError> {
//         const MIN_LENGTH: usize = 1;
//         const MAX_LENGTH: usize = 255;

//         let trimmed = title.trim();
//         if trimmed.is_empty() {
//             return Err(TitleError::Empty);
//         }
//         if trimmed.len() < MIN_LENGTH {
//             return Err(TitleError::TooShort { min: MIN_LENGTH });
//         }
//         if trimmed.len() > MAX_LENGTH {
//             return Err(TitleError::TooLong { max: MAX_LENGTH });
//         }
//         if trimmed.contains('\n') {
//             return Err(TitleError::ContainsNewline);
//         }
//         // Remove duplicate spaces
//         let normalised = trimmed.split_whitespace().collect::<Vec<&str>>().join(" ");

//         // Check for invalid characters (example: only allow alphanumeric and some punctuation)
//         if !normalised
//             .chars()
//             .all(|c| c.is_alphanumeric() || " ,.!?-_".contains(c))
//         {
//             return Err(TitleError::InvalidCharacters);
//         }

//         Ok(Self(normalised))
//     }
// }

// #[derive(Debug, Error)]
// pub enum DocumentError {
//     #[error("Invalid Title: {0}")]
//     TitleError(#[from] TitleError),
//     #[error("Invalid Content")]
//     ContentError,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Document {
//     title: Title,
//     content: Content,
//     saved_at: Option<Timestamp>,
//     modified_at: Option<Timestamp>,
// }

// impl Document {
//     pub fn new(title: Title, content: Content) -> Self {
//         Self {
//             title,
//             content,
//             saved_at: None,
//             modified_at: None,
//         }
//     }
//     fn set_modified(&mut self) {
//         self.modified_at = Some(Timestamp::default());
//     }
//     pub fn set_saved(&mut self) {
//         self.saved_at = Some(Timestamp::default());
//     }
//     pub fn title(&self) -> &Title {
//         &self.title
//     }
//     pub fn content(&self) -> &Content {
//         &self.content
//     }
//     pub fn has_unsaved_changes(&self) -> bool {
//         Some(self.modified_at) > Some(self.saved_at)
//     }
//     pub fn set_title(&mut self, title: &str) -> Result<(), DocumentError> {
//         match Title::new(title) {
//             Ok(new_title) => {
//                 self.title = new_title;
//                 self.set_modified();
//                 Ok(())
//             }
//             Err(err) => Err(DocumentError::TitleError(err)),
//         }
//     }
//     pub fn set_content(&mut self, content: &str) -> Result<(), DocumentError> {
//         match content.parse::<Content>() {
//             Ok(new_content) => {
//                 self.content = new_content;
//                 self.set_modified();
//                 Ok(())
//             }
//             Err(err) => Err(DocumentError::ContentError),
//         }
//     }
// }
// impl Default for Document {
//     fn default() -> Self {
//         Self {
//             title: Title::default(),
//             content: Content::default(),
//             saved_at: None,
//             modified_at: None,
//         }
//     }
// }

// #[derive(Debug, Error)]
// pub enum NodeError {
//     #[error("Document error: {0}")]
//     DocumentError(#[from] DocumentError),
// }
