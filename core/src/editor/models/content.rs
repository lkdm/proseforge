use derive_more::derive::From;
use std::error::Error;
use thiserror::Error;

use crate::data::{Id, Timestamp};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ContentId(Id);

/// Content is text that can be edited in the editor.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Content {
    id: ContentId,
    data: String,

    created_at: Timestamp,
    saved_at: Option<Timestamp>,
    modified_at: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateContentRequest {
    id: ContentId,
    data: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateContentRequest {
    data: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct DeleteContentRequest {
    id: ContentId,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetContentRequest {
    id: ContentId,
}

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
