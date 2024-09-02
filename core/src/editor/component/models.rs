use derive_more::derive::{Deref, From};
use std::{cmp::Ordering, error::Error};
use thiserror::Error;

use crate::{
    editor::Title,
    types::{Id, Timestamp},
};

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

impl Default for ComponentKind {
    fn default() -> Self {
        ComponentKind::Note
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid summary.")]
pub struct SummaryError(String);

#[derive(Deref, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Summary(String);

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
    modified_at: Option<Timestamp>,
    deleted_at: Option<Timestamp>,
}

impl ProjectComponent {
    fn new(project_id: Id) -> Self {
        ProjectComponent {
            id: Id::new(),
            kind: ComponentKind::Note,
            display_order: 0,
            title: Title::default(),
            summary: None,
            project_id,
            parent_id: None,
            document_id: None,
            created_at: Timestamp::now(),
            modified_at: None,
            deleted_at: None,
        }
    }
    pub fn builder(project_id: Id) -> ProjectComponentBuilder {
        ProjectComponentBuilder::new(project_id)
    }
}

pub struct ProjectComponentBuilder {
    project_id: Id,
    kind: Option<ComponentKind>,
    display_order: Option<u32>,
    title: Option<Title>,
    summary: Option<Summary>,
    parent_id: Option<Id>,
    document_id: Option<Id>,
    created_at: Option<Timestamp>,
    modified_at: Option<Timestamp>,
    deleted_at: Option<Timestamp>,
}

impl ProjectComponentBuilder {
    fn new(project_id: Id) -> Self {
        ProjectComponentBuilder {
            project_id,
            kind: None,
            display_order: None,
            title: None,
            summary: None,
            parent_id: None,
            document_id: None,
            created_at: None,
            modified_at: None,
            deleted_at: None,
        }
    }
    pub fn with_kind(mut self, kind: ComponentKind) -> Self {
        self.kind = Some(kind);
        self
    }
    pub fn with_display_order(mut self, display_order: u32) -> Self {
        self.display_order = Some(display_order);
        self
    }
    pub fn with_title<T>(mut self, title: Option<T>) -> Self
    where
        T: Into<Title>,
    {
        self.title = title.map(|t| t.into());
        self
    }
    pub fn with_summary<T>(mut self, summary: Option<T>) -> Self
    where
        T: Into<Summary>,
    {
        self.summary = summary.map(|s| s.into());
        self
    }
    pub fn with_parent_id<T>(mut self, parent_id: Option<T>) -> Self
    where
        T: Into<Id>,
    {
        self.parent_id = parent_id.map(|id| id.into());
        self
    }
    pub fn with_document_id<T>(mut self, document_id: T) -> Self
    where
        T: Into<Id>,
    {
        self.document_id = Some(document_id.into());
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

    pub fn build(self) -> ProjectComponent {
        ProjectComponent {
            id: Id::new(),
            kind: self.kind.unwrap_or(ComponentKind::Note),
            display_order: self.display_order.unwrap_or(0),
            title: self.title.unwrap_or_default(),
            summary: self.summary,
            project_id: self.project_id,
            parent_id: self.parent_id,
            document_id: self.document_id,
            created_at: self.created_at.unwrap_or(Timestamp::now()),
            modified_at: self.modified_at,
            deleted_at: self.deleted_at,
        }
    }
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
    title: Option<Title>,
    summary: Option<Summary>,
    parent_id: Option<Id>,
    document_id: Option<Id>,
    display_order: u32,
}

impl CreateComponentRequest {
    pub fn new(
        project_id: Id,
        kind: ComponentKind,
        title: Option<Title>,
        summary: Option<Summary>,
        parent_id: Option<Id>,
        document_id: Option<Id>,
        display_order: u32,
    ) -> Self {
        CreateComponentRequest {
            project_id,
            kind,
            title,
            summary,
            parent_id,
            document_id,
            display_order,
        }
    }
    pub fn project_id(&self) -> Id {
        self.project_id.clone()
    }
    pub fn kind(&self) -> ComponentKind {
        self.kind.clone()
    }
    pub fn title(&self) -> Option<Title> {
        self.title.clone()
    }
    pub fn summary(&self) -> Option<Summary> {
        self.summary.clone()
    }
    pub fn parent_id(&self) -> Option<Id> {
        self.parent_id.clone()
    }
    pub fn document_id(&self) -> Option<Id> {
        self.document_id.clone()
    }
    pub fn display_order(&self) -> u32 {
        self.display_order
    }
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
