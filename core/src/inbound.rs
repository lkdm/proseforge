use std::future::Future;

use crate::domain::editor::models::Content;
use derive_more::derive::From;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct UpdateContentRequest {
    content: Content,
}

#[derive(Debug, Error)]
#[error("Could not update content.")]
pub struct UpdateContentError {}

impl UpdateContentRequest {
    pub fn new(content: Content) -> Self {
        Self { content }
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}

pub trait ContentRepository: Clone + Send + Sync + 'static {
    fn update_content(
        &self,
        req: &UpdateContentRequest,
    ) -> impl Future<Output = Result<(), UpdateContentError>> + Send;
}
