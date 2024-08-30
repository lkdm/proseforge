use derive_more::derive::{Deref, From};
use proseforge_common::Id;
use std::{cmp::Ordering, error::Error};
use thiserror::Error;

use crate::node::Timestamp;

use super::Title;

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
