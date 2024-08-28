use derive_more::derive::From;
use std::collections::HashMap;
use std::error::Error;
use thiserror::Error;

use crate::node::Id;

use super::{
    component::{Component, ComponentId},
    content::{Content, ContentId},
    Title,
};
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct ProjectId(Id);

/// A project is a collection of components and content.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Project {
    id: ProjectId,
    title: Title,
    root_components: Vec<ComponentId>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateProjectRequest {
    id: ProjectId,
    data: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateProjectRequest {
    data: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct DeleteProjectRequest {
    id: ProjectId,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetProjectRequest {
    id: ProjectId,
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
