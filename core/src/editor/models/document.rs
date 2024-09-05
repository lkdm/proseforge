use anyhow;
use derive_more::derive::From;
use std::{
    error::Error,
    hash::{Hash, Hasher},
};
use thiserror::Error;

use crate::types::{Id, Timestamp};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContentError {
    #[error("Content exceeds the maximum length of {max_length} characters")]
    ExceedsMaxLength { max_length: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Content(String);

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

impl Default for Content {
    fn default() -> Self {
        Content::new(String::new()).unwrap()
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
    modified_at: Timestamp,
    deleted_at: Option<Timestamp>,
}

// impl Hash for Document {
//     /// Hashes the `Document` based on its `content`.
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.content.hash(state);
//     }
// }

impl Document {
    fn new(
        id: Id,
        project_id: Id,
        content: Content,
        created_at: Timestamp,
        modified_at: Timestamp,
        deleted_at: Option<Timestamp>,
    ) -> Self {
        Document {
            id,
            project_id,
            content,
            created_at,
            modified_at,
            deleted_at,
        }
    }
    pub fn builder<T>(project_id: T) -> DocumentBuilder
    where
        T: Into<Id>,
    {
        DocumentBuilder::new(project_id.into())
    }
    pub fn id(&self) -> Id {
        self.id.clone()
    }
    pub fn project_id(&self) -> Id {
        self.project_id.clone()
    }
    pub fn content(&self) -> Content {
        self.content.clone()
    }
    pub fn created_at(self) -> Timestamp {
        self.created_at.clone()
    }
    pub fn modified_at(&self) -> Timestamp {
        self.modified_at.clone()
    }
    pub fn deleted_at(&self) -> Option<Timestamp> {
        self.deleted_at.clone()
    }
}

/// Builder for Document
///
/// Allows for a document to be built up in a fluent style.
#[derive(Clone, Debug)]
pub struct DocumentBuilder {
    project_id: Id,
    id: Option<Id>,
    content: Option<Content>,
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

    pub fn with_defaults(mut self) -> Self {
        self.id = Some(Id::new());
        self.content = Some(Content::default());
        self.created_at = Some(Timestamp::now());
        self.modified_at = Some(Timestamp::now());
        self
    }

    pub fn with_id<T>(mut self, id: T) -> Self
    where
        T: Into<Id>,
    {
        self.id = Some(id.into());
        self
    }

    pub fn with_content<T>(mut self, content: T) -> Self
    where
        T: Into<Content>,
    {
        self.content = Some(content.into());
        self
    }

    pub fn with_created_at<T>(mut self, created_at: T) -> Self
    where
        T: Into<Timestamp>,
    {
        self.created_at = Some(created_at.into());
        self
    }

    pub fn with_modified_at<T>(mut self, modified_at: Option<T>) -> Self
    where
        T: Into<Timestamp>,
    {
        self.modified_at = modified_at.map(|t| t.into());
        self
    }

    pub fn with_deleted_at<T>(mut self, deleted_at: Option<T>) -> Self
    where
        T: Into<Timestamp>,
    {
        self.deleted_at = deleted_at.map(|t| t.into());
        self
    }

    pub fn build(self) -> Document {
        Document::new(
            self.id.unwrap_or(Id::new()),
            self.project_id,
            self.content.unwrap_or(Content::default()),
            self.created_at.unwrap_or(Timestamp::now()),
            self.modified_at.unwrap_or(Timestamp::now()),
            self.deleted_at,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateDocumentRequest {
    id: Id,
    content: Content,
}

impl UpdateDocumentRequest {
    pub fn new(id: Id, content: Content) -> Self {
        UpdateDocumentRequest { id, content }
    }
    pub fn id(&self) -> Id {
        self.id.clone()
    }
    pub fn content(&self) -> Content {
        self.content.clone()
    }
    pub fn modified_at(&self) -> Timestamp {
        Timestamp::now()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateDocumentRequest {
    project_id: Id,
}

impl CreateDocumentRequest {
    pub fn new(project_id: Id) -> Self {
        CreateDocumentRequest { project_id }
    }
    pub fn project_id(&self) -> Id {
        self.project_id.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct DeleteDocumentRequest {
    id: Id,
}

impl DeleteDocumentRequest {
    pub fn new(id: Id) -> Self {
        DeleteDocumentRequest { id }
    }
    pub fn id(&self) -> Id {
        self.id.clone()
    }
    pub fn deleted_at(&self) -> Timestamp {
        Timestamp::now()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetDocumentRequest {
    id: Id,
}

impl GetDocumentRequest {
    pub fn new(id: Id) -> Self {
        GetDocumentRequest { id }
    }
    pub fn id(&self) -> Id {
        self.id.clone()
    }
}

#[derive(Debug, Error)]
pub enum UpdateDocumentError {
    #[error("Validation error: {source}")]
    ValidationError {
        #[from]
        source: ContentError,
    },
    #[error("Repository error: {source}")]
    RepositoryError {
        #[from]
        source: sqlx::Error,
    },
    #[error("Operation failed: {0}")]
    UnexpectedError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum CreateDocumentError {
    #[error("Operation failed: {0}")]
    UnexpectedError(String),
}

#[derive(Debug, Error)]
pub enum DeleteDocumentError {
    #[error("Repository error: {source}")]
    RepositoryError {
        #[from]
        source: sqlx::Error,
    },
    #[error("Operation failed: {0}")]
    UnexpectedError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum GetDocumentError {
    #[error("Repository error: {source}")]
    DatabaseError {
        #[from]
        source: sqlx::Error,
    },
    #[error("Operation failed: {source}")]
    OperationFailed {
        #[from]
        source: anyhow::Error,
    },
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ListDocumentError {
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}
