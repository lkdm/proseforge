use derive_more::derive::{Deref, From};
use proseforge_common::Id;
use std::{
    cmp::Ordering,
    error::Error,
    fmt::{self, Display, Formatter},
};
use thiserror::Error;

use crate::node::Timestamp;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid title.")]
pub struct TitleError(String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Title(String);

impl Title {
    pub fn new(title: String) -> Result<Self, TitleError> {
        // Strip leading and trailing whitespace
        let trimmed_title = title.trim();

        // Replace multiple consecutive spaces with a single space
        let normalized_title = trimmed_title
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        // Check if the title is empty after normalization
        if normalized_title.is_empty() {
            return Err(TitleError("Title cannot be empty".to_string()));
        }

        // Check if the title length is within the limit
        if normalized_title.len() > 254 {
            return Err(TitleError(
                "Title exceeds maximum length of 254 characters".to_string(),
            ));
        }

        // Validate allowed characters (alphanumeric, spaces, underscores, and dashes)
        if !normalized_title
            .chars()
            .all(|c| c.is_alphanumeric() || c.is_whitespace() || c == '_' || c == '-')
        {
            return Err(TitleError("Title contains invalid characters".to_string()));
        }

        Ok(Title(normalized_title))
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_title_new() {
        let title = Title::new("Hello World".to_string());
        assert_eq!(title.unwrap().0, "Hello World");
    }

    #[test]
    fn test_title_new_empty() {
        let title = Title::new("".to_string());
        assert!(title.is_err());
    }

    #[test]
    fn test_title_new_whitespace() {
        let title = Title::new("  ".to_string());
        assert!(title.is_err());
    }

    #[test]
    fn test_title_new_whitespace_trimmed() {
        let title = Title::new("  Hello World  ".to_string());
        assert_eq!(title.unwrap().0, "Hello World");
    }

    #[test]
    fn test_title_new_whitespace_normalized() {
        let title = Title::new("Hello    World!".to_string());
        assert_eq!(title.unwrap().0, "Hello World");
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
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

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid summary.")]
pub struct SummaryError(String);

#[derive(Deref, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Summary(String);

impl Summary {
    fn new(summary: String) -> Result<Self, SummaryError> {
        // Trim whitespace
        let normalized_summary = summary.trim().to_string();

        // Validate length
        if normalized_summary.len() > 254 {
            return Err(SummaryError(
                "Summary exceeds maximum length of 254 characters".to_string(),
            ));
        }

        Ok(Summary(normalized_summary))
    }
}

impl From<String> for Summary {
    fn from(summary: String) -> Self {
        Summary::new(summary).unwrap()
    }
}
impl Into<String> for Summary {
    fn into(self) -> String {
        self.0
    }
}

/// # Component
///
/// A component is a part of a project. It can be a draft, part, chapter, scene, character, location, note, or outline.
/// Components can have children, and can be nested.
/// They have an optional summary and document.
#[derive(Clone, Debug, Eq, Hash, From)]
pub struct ProjectComponent {
    id: Id,
    kind: ComponentKind,
    display_order: u32,

    title: Title,
    summary: Option<Summary>,

    project_id: Id,
    parent_id: Option<Id>,
    document_id: Option<Id>,

    created_at: Timestamp,
    updated_at: Option<Timestamp>,
    deleted_at: Option<Timestamp>,
}

impl PartialOrd for ProjectComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.display_order.cmp(&other.display_order))
    }
}
impl Ord for ProjectComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.display_order.cmp(&other.display_order)
    }
}
impl PartialEq for ProjectComponent {
    /// Compare by id
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateComponentRequest {
    project_id: Id,
    id: Id,
    kind: ComponentKind,
    title: Title,
    summary: Option<Summary>,
    parent_id: Option<Id>,
    document_id: Option<Id>,
    display_order: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateComponentRequest {
    project_id: Id,
    kind: ComponentKind,
    title: Title,
    summary: Option<Summary>,
    parent_id: Option<Id>,
    document_id: Option<Id>,
    display_order: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct DeleteComponentRequest {
    id: Id,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetComponentRequest {
    id: Id,
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
