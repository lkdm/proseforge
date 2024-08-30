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
            content: Content::from(content),
            created_at: Timestamp::now(),
            modified_at: None,
            deleted_at: None,
        }
    }
    pub fn builder<T>(project_id: T) -> DocumentBuilder
    where
        T: Into<Id>,
    {
        DocumentBuilder::new(project_id.into())
    }
}

/// Builder for Document
///
/// Allows for a document to be built up in a fluent style.
#[derive(Clone, Debug)]
struct DocumentBuilder {
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
            self.id,
            self.project_id,
            self.content,
            self.created_at,
            self.modified_at,
            self.deleted_at,
        )
        // Document {
        //     id: self.id.unwrap_or(Id::new()),
        //     project_id: self.project_id,
        //     content: self.content.unwrap_or(Content::default()).into(),
        //     created_at: self.created_at.unwrap_or(Timestamp::now()),
        //     modified_at: self.modified_at,
        //     deleted_at: self.deleted_at,
        // }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateDocumentRequest {
    id: Id,
    content: Content,
    deleted_at: Option<Timestamp>,
}

impl UpdateDocumentRequest {
    pub fn new(id: Id, content: String) -> Result<Self, UpdateDocumentError> {
        let req = UpdateDocumentRequest {
            id,
            content: Content::new(content)?,
        };
        Ok(req)
    }
    pub fn id(&self) -> Id {
        self.id.clone()
    }
    pub fn content(&self) -> Content {
        self.content.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateDocumentRequest {
    content: Option<Content>,
}

impl CreateDocumentRequest {
    pub fn new(content: String) -> Result<Self, CreateDocumentError> {
        let req = CreateDocumentRequest {
            content: Some(Content::new(content)?),
        };
        Ok(req)
    }
    pub fn content(&self) -> Content {
        self.content.clone().unwrap()
    }
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
    #[error("Repository error: {source}")]
    RepositoryError {
        #[from]
        source: sqlx::Error,
    },
    #[error("Operation failed: {0}")]
    UnexpectedError(#[source] Box<dyn Error + Send + Sync>),
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
    RepositoryError {
        #[from]
        source: sqlx::Error,
    },
    #[error("Operation failed: {0}")]
    UnexpectedError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum ListDocumentError {
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}
