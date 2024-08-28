use std::future::Future;

use super::models::{
    Content, CreateContentError, CreateContentRequest, DeleteContentError, DeleteContentRequest,
    GetContentError, GetContentRequest, ListContentError, ListContentRequest, UpdateContentError,
    UpdateContentRequest,
};

/// ContentRepository
pub trait ContentRepository: Clone + Send + Sync + 'static {
    /// Creates new content in the repository.
    fn create_content(
        &self,
        req: &CreateContentRequest,
    ) -> impl Future<Output = Result<Content, CreateContentError>> + Send;

    /// Retrieves content from the repository.
    fn get_content(
        &self,
        req: &GetContentRequest,
    ) -> impl Future<Output = Result<Content, GetContentError>> + Send;

    /// Updates existing content in the repository.
    fn update_content(
        &self,
        req: &UpdateContentRequest,
    ) -> impl Future<Output = Result<(), UpdateContentError>> + Send;

    /// Deletes content from the repository.
    fn delete_content(
        &self,
        req: &DeleteContentRequest,
    ) -> impl Future<Output = Result<(), DeleteContentError>> + Send;

    /// Lists content from the repository.
    fn list_content(
        &self,
        req: &ListContentRequest,
    ) -> impl Future<Output = Result<Vec<Content>, ListContentError>> + Send;
}
