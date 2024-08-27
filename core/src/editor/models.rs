use std::path::PathBuf;

use crate::data::{DirectoryId, DocumentId, ProjectId};
use derive_more::derive::{AsRef, Constructor, Deref, Display, From, FromStr};
use serde::Deserialize;
use thiserror::Error;

use crate::data::Timestamp;

#[derive(
    Debug,
    Clone,
    Display,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    AsRef,
    Deref,
    Constructor,
    FromStr,
    Hash,
    Deserialize,
)]
pub struct Content(String);

impl From<&str> for Content {
    fn from(s: &str) -> Self {
        Content(s.to_string())
    }
}

impl From<String> for Content {
    fn from(s: String) -> Self {
        Content(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Document {
    // title: Title,
    id: DocumentId,
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
    pub fn id(&self) -> DocumentId {
        self.id
    }
}

pub struct DocumentBuilder {
    id: Option<DocumentId>,
    content: Option<Content>,
    saved_at: Option<Timestamp>,
    modified_at: Option<Timestamp>,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
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
    pub fn with_id(mut self, id: DocumentId) -> Self {
        self.id = Some(id);
        self
    }
    pub fn generate_id(mut self) -> Self {
        self.id = Some(DocumentId::default());
        self
    }
    pub fn build(self) -> Document {
        Document {
            id: self.id.unwrap(),
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
    id: DocumentId,
    content: Content,
}

impl UpdateDocumentRequest {
    pub fn new(id: DocumentId, content: Content) -> Self {
        Self { id, content }
    }

    pub fn id(&self) -> DocumentId {
        self.id
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
pub struct GetDocumentRequest {
    id: DocumentId,
}

#[derive(Debug, Error)]
pub enum GetDocumentError {
    #[error("Unknown error")]
    UnknownError,
}
