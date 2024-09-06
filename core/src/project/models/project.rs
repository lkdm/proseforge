use crate::{editor::Title, types::Id};
use bon::{bon, builder};
use derive_more::derive::From;
use std::error::Error;
use thiserror::Error;

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

impl From<String> for ProjectKind {
    fn from(s: String) -> Self {
        s.parse().unwrap()
    }
}

/// A project is a collection of components and content.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
#[builder]
pub struct Project {
    id: Id,
    title: Title,
    kind: ProjectKind,
}

impl Project {
    pub fn id(&self) -> Id {
        self.id.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
#[builder]
pub struct UpdateProjectRequest {
    id: Id,
    title: Title,
    kind: ProjectKind,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
#[builder]
pub struct CreateProjectRequest {
    title: Title,
    kind: ProjectKind,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
#[builder]
pub struct DeleteProjectRequest {
    id: Id,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
#[builder]
pub struct GetProjectRequest {
    id: Id,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
#[builder]
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
    #[error("Could not create prosefile")]
    ProsefileError,
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
    #[error("No path provided")]
    NoPath,
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
