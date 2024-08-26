use std::future::Future;

use derive_more::derive::{AsRef, Constructor, Deref, Display, From, FromStr};
use thiserror::Error;

use crate::data::Timestamp;

#[derive(
    Debug, Clone, Display, PartialOrd, Ord, PartialEq, Eq, AsRef, Deref, Constructor, FromStr, Hash,
)]
pub struct Content(String);

impl Into<String> for Content {
    fn into(self) -> String {
        self.0
    }
}

impl Into<Content> for String {
    fn into(self) -> Content {
        Content(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    // title: Title,
    content: Content,
    saved_at: Option<Timestamp>,
    modified_at: Option<Timestamp>,
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::new()
    }
    fn set_modified(&mut self) {
        self.modified_at = Some(Timestamp::default());
    }
    pub fn set_content(&mut self, content: Content) {
        self.content = content.into();
        self.set_modified();
    }
}

pub struct DocumentBuilder {
    content: Option<Content>,
    saved_at: Option<Timestamp>,
    modified_at: Option<Timestamp>,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self {
            content: None,
            saved_at: None,
            modified_at: None,
        }
    }
    pub fn with_content(mut self, content: Content) -> Self {
        self.content = Some(content);
        self
    }
    pub fn saved_now(mut self) -> Self {
        self.saved_at = Some(Timestamp::default());
        self
    }
    pub fn modified_now(mut self) -> Self {
        self.modified_at = Some(Timestamp::default());
        self
    }
    pub fn build(self) -> Document {
        Document {
            content: self.content.unwrap(),
            saved_at: self.saved_at,
            modified_at: self.modified_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateDocumentRequest {
    content: Content,
}

impl CreateDocumentRequest {
    pub fn new(content: Content) -> Self {
        Self { content }
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}

#[derive(Debug, Error)]
pub enum CreateDocumentError {
    #[error("Unknown error")]
    UnknownError,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateDocumentRequest {
    content: Content,
}

impl UpdateDocumentRequest {
    pub fn new(content: Content) -> Self {
        Self { content }
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}

#[derive(Debug, Error)]
pub enum UpdateDocumentError {
    #[error("Unknown error")]
    UnknownError,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetDocumentRequest {}

#[derive(Debug, Error)]
pub enum GetDocumentError {
    #[error("Unknown error")]
    UnknownError,
}
