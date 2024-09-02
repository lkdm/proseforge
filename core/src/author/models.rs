use derive_more::derive::{Deref, From};
use proseforge_common::Id;
use std::error::Error;
use std::path::PathBuf;
use strum_macros::EnumString;
use thiserror::Error;

#[derive(Deref, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UserId(Id);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub enum NodeConfigVersion {
    V0,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Theme {
    System,
    Light,
    Dark,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct NodeConfig {
    user_id: UserId,
    theme: Theme,
    font_size: u8,
    version: NodeConfigVersion,
    data_directory: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateNodeConfig {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateNodeConfig {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct GetNodeConfig {}

#[derive(Debug, Error)]
pub enum UpdateNodeConfigError {
    #[error("NodeConfig not found")]
    NotFound,
    #[error("Invalid NodeConfig data: {0}")]
    ValidationError(String),
    #[error("Conflict: {0}")]
    ConflictError(String),
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum CreateNodeConfigError {
    #[error("Invalid NodeConfig data: {0}")]
    ValidationError(String),
    #[error("Duplicate NodeConfig")]
    DuplicateError,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum GetNodeConfigError {
    #[error("NodeConfig not found")]
    NotFound,
    #[error("Operation failed: {0}")]
    OperationError(#[source] Box<dyn Error + Send + Sync>),
}
