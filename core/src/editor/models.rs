use crate::node::Timestamp;
use derive_more::derive::{Deref, From};
use proseforge_common::Id;
use std::{
    error::Error,
    hash::{Hash, Hasher},
};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContentError {
    #[error("Content exceeds the maximum length of {max_length} characters")]
    ExceedsMaxLength { max_length: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Content(String);

impl Content {
    pub fn new(content: String) -> Result<Self, ContentError> {
        let max_length = 66_666;
        let trimmed_content = content.trim().to_string();

        if trimmed_content.len() > max_length {
            return Err(ContentError::ExceedsMaxLength { max_length });
        }

        Ok(Content(trimmed_content))
    }
}

impl From<String> for Content {
    fn from(s: String) -> Self {
        Content::new(s).unwrap()
    }
}
impl Into<String> for Content {
    fn into(self) -> String {
        self.0
    }
}

/// # Document
///
/// A `Document` is the primary unit of content in Proseforge.
/// It represents anything that is written in the main editor.
/// Because documents can be quite long, they are managed separately from `ProjectComponent`.
///
#[derive(Clone, Debug, PartialEq, Eq, From)]
pub struct Document {
    id: Id,
    project_id: Id,

    content: Content,

    created_at: Timestamp,
    modified_at: Option<Timestamp>,
    deleted_at: Option<Timestamp>,
}

impl Hash for Document {
    /// Hashes the `Document` based on its `content`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.content.hash(state);
    }
}

impl Document {
    fn new(project_id: Id, content: String) -> Self {
        Document {
            id: Id::new(),
            project_id,
            content,
            created_at: Timestamp::now(),
            modified_at: None,
            deleted_at: None,
        }
    }
    pub fn builder(project_id: Id) -> DocumentBuilder {
        DocumentBuilder::new(project_id)
    }
}

/// Builder for Document
///
/// Allows for a document to be built up in a fluent style.
#[derive(Clone, Debug)]
struct DocumentBuilder {
    project_id: Id,
    id: Option<Id>,
    content: Option<String>,
    created_at: Option<Timestamp>,
    modified_at: Option<Timestamp>,
    deleted_at: Option<Timestamp>,
}

impl DocumentBuilder {
    pub fn new(project_id: Id) -> Self {
        DocumentBuilder {
            project_id,
            id: None,
            content: None,
            created_at: None,
            modified_at: None,
            deleted_at: None,
        }
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_project_id(mut self, project_id: Id) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn with_created_at(mut self, created_at: Timestamp) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn with_modified_at(mut self, modified_at: Timestamp) -> Self {
        self.modified_at = Some(modified_at);
        self
    }

    pub fn with_deleted_at(mut self, deleted_at: Timestamp) -> Self {
        self.deleted_at = Some(deleted_at);
        self
    }

    pub fn build(self) -> Document {
        Document {
            id: self.id.unwrap_or(Id::new()),
            project_id: self.project_id.expect("project_id is mandatory"),
            content: self.content.unwrap_or(String::new()).into(),
            created_at: self.created_at.unwrap_or(Timestamp::now()),
            modified_at: self.modified_at,
            deleted_at: self.deleted_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateDocumentRequest {
    id: Id,
    content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateDocumentRequest {
    pub content: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct DeleteDocumentRequest {
    pub id: Id,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetDocumentRequest {
    pub id: Id,
}

#[derive(Debug, Error)]
pub enum UpdateDocumentError {
    #[error("Document not found")]
    NotFound,
    #[error("Invalid document data: {0}")]
    ValidationError(String),
    #[error("Conflict: {0}")]
    ConflictError(String),
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum CreateDocumentError {
    #[error("Invalid content data: {0}")]
    ValidationError(String),
    #[error("Duplicate content")]
    DuplicateError,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
    // #[error("Database error: {0}")]
    // DatabaseError(#[from] RusqliteError),
}

#[derive(Debug, Error)]
pub enum DeleteDocumentError {
    #[error("Document not found")]
    NotFound,
    #[error("Cannot delete: document is referenced")]
    ReferenceError,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum GetDocumentError {
    #[error("Document not found")]
    NotFound,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum ListDocumentError {
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}
