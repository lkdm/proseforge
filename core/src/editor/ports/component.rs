use std::future::Future;

use crate::editor::models::component::{
    Component, CreateComponentError, CreateComponentRequest, DeleteComponentError,
    DeleteComponentRequest, GetComponentError, GetComponentRequest, ListComponentError,
    ListComponentRequest, UpdateComponentError, UpdateComponentRequest,
};

/// ComponentRepository
pub trait ComponentRepository: Clone + Send + Sync + 'static {
    /// Creates new component in the repository.
    fn create_component(
        &self,
        req: &CreateComponentRequest,
    ) -> impl Future<Output = Result<Component, CreateComponentError>> + Send;

    /// Retrieves component from the repository.
    fn get_component(
        &self,
        req: &GetComponentRequest,
    ) -> impl Future<Output = Result<Component, GetComponentError>> + Send;

    /// Updates existing component in the repository.
    fn update_component(
        &self,
        req: &UpdateComponentRequest,
    ) -> impl Future<Output = Result<(), UpdateComponentError>> + Send;

    /// Deletes component from the repository.
    fn delete_component(
        &self,
        req: &DeleteComponentRequest,
    ) -> impl Future<Output = Result<(), DeleteComponentError>> + Send;

    /// Lists components from the repository.
    fn list_components(
        &self,
        req: &ListComponentRequest,
    ) -> impl Future<Output = Result<Vec<Component>, ListComponentError>> + Send;
}
