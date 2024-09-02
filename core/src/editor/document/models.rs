use anyhow;
use derive_more::derive::From;
use std::{
    error::Error,
    hash::{Hash, Hasher},
};
use thiserror::Error;

use crate::types::{Id, Timestamp};

use super::{Content, ContentError};

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
