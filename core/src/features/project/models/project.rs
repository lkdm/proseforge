use derive_more::derive::From;
use proseforge_common::Id;
use std::error::Error;
use thiserror::Error;

use super::Title;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ProjectKind {
    /// A environment that is used for practicing the craft of writing.
    Practice,

    /// A single short story.
    ShortStory,

    /// A collection of short stories set in the same universe, or with a common theme.
    Anthology,

    /// A single novel.
    Novel,

    /// A novel with multiple parts.
    NovelWithParts,

    /// A collection of novels set in the same universe, or with a common theme.
    Series,

    /// A project used for journaling.
    Journal,
}

/// A project is a collection of components and content.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Project {
    id: Id,
    title: Title,
    kind: ProjectKind,
    components: Vec<Id>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateProjectRequest {
    id: Id,
    content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateProjectRequest {
    content: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct DeleteProjectRequest {
    id: Id,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetProjectRequest {
    id: Id,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ListProjectsRequest {} // TODO: Filters, pagination, params

#[derive(Debug, Error)]
pub enum UpdateProjectError {
    #[error("Project not found")]
    NotFound,
    #[error("Invalid project data: {0}")]
    ValidationError(String),
    #[error("Conflict: {0}")]
    ConflictError(String),
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum CreateProjectError {
    #[error("Invalid project data: {0}")]
    ValidationError(String),
    #[error("Duplicate project")]
    DuplicateError,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum DeleteProjectError {
    #[error("Project not found")]
    NotFound,
    #[error("Cannot delete: project is referenced")]
    ReferenceError,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum GetProjectError {
    #[error("Project not found")]
    NotFound,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum ListProjectsError {
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}
