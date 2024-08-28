use derive_more::derive::From;
use rusqlite::Error as RusqliteError;
use std::error::Error;
use thiserror::Error;

use crate::node::{Id, Timestamp};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ContentId(Id);

impl ContentId {
    pub fn new(id: Id) -> Self {
        ContentId(id)
    }
}

/// Content is text that can be edited in the editor.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Content {
    id: ContentId,
    data: String,

    created_at: Timestamp,
    modified_at: Option<Timestamp>,
    deleted_at: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateContentRequest {
    id: ContentId,
    data: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateContentRequest {
    pub data: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct DeleteContentRequest {
    id: ContentId,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetContentRequest {
    id: ContentId,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ListContentRequest {} // TODO: Filters, pagination, params

#[derive(Debug, Error)]
pub enum UpdateContentError {
    #[error("Content not found")]
    NotFound,
    #[error("Invalid content data: {0}")]
    ValidationError(String),
    #[error("Conflict: {0}")]
    ConflictError(String),
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum CreateContentError {
    #[error("Invalid content data: {0}")]
    ValidationError(String),
    #[error("Duplicate content")]
    DuplicateError,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
    #[error("Database error: {0}")]
    DatabaseError(#[from] RusqliteError),
}

#[derive(Debug, Error)]
pub enum DeleteContentError {
    #[error("Content not found")]
    NotFound,
    #[error("Cannot delete: content is referenced")]
    ReferenceError,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum GetContentError {
    #[error("Content not found")]
    NotFound,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum ListContentError {
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}
