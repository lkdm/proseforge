use derive_more::derive::From;
use std::error::Error;
use thiserror::Error;

use crate::node::Id;

use super::content::ContentId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ComponentId(Id);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, strum_macros::EnumString)]
pub enum ComponentKind {
    Draft,
    Part,
    Chapter,
    Scene,
    Character,
    Location,
    Note,
    Outline,
}

/// # Component
///
/// A component is a part of a project. It can be a draft, part, chapter, scene, character, location, note, or outline.
/// Components can have children, and can be nested.
/// They have an optional summary and document.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Component {
    id: ComponentId,
    kind: ComponentKind,
    children: Vec<ComponentId>,
    parent: Option<ComponentId>,

    summary: Option<ContentId>,
    document: Option<ContentId>,
    // TODO: comments, maybe by line-location, etc
    // comments: Vec<ContentId>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateComponentRequest {
    id: ComponentId,
    data: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateComponentRequest {
    data: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct DeleteComponentRequest {
    id: ComponentId,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetComponentRequest {
    id: ComponentId,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ListComponentRequest {} // TODO: Filters, pagination, params

#[derive(Debug, Error)]
pub enum UpdateComponentError {
    #[error("Component not found")]
    NotFound,
    #[error("Invalid component data: {0}")]
    ValidationError(String),
    #[error("Conflict: {0}")]
    ConflictError(String),
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum CreateComponentError {
    #[error("Invalid component data: {0}")]
    ValidationError(String),
    #[error("Duplicate component")]
    DuplicateError,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum DeleteComponentError {
    #[error("Component not found")]
    NotFound,
    #[error("Cannot delete: component is referenced")]
    ReferenceError,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum GetComponentError {
    #[error("Component not found")]
    NotFound,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum ListComponentError {
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}
