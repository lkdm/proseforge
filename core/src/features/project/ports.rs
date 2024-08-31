///
/// Ports are the interfaces that the application uses to interact with the outside world.
use proseforge_common::Id;

use crate::features::project::models::component::{
    CreateComponentError, CreateComponentRequest, DeleteComponentError, DeleteComponentRequest,
    GetComponentError, GetComponentRequest, ListComponentError, ListComponentRequest,
    ProjectComponent, UpdateComponentError, UpdateComponentRequest,
};
use crate::features::project::models::document::{
    CreateDocumentError, CreateDocumentRequest, DeleteDocumentError, DeleteDocumentRequest,
    Document, GetDocumentError, GetDocumentRequest, UpdateDocumentError, UpdateDocumentRequest,
};
use crate::features::project::models::project::{
    CreateProjectError, CreateProjectRequest, DeleteProjectError, DeleteProjectRequest,
    GetProjectError, GetProjectRequest, ListProjectsError, ListProjectsRequest, Project,
    UpdateProjectError, UpdateProjectRequest,
};
use std::future::Future;

/// ProjectRepository
pub trait ProjectRepository: Clone + Send + Sync + 'static {
    /// Creates a new project in the repository.
    // fn create_project(
    //     &self,
    //     req: &CreateProjectRequest,
    // ) -> impl Future<Output = Result<Project, CreateProjectError>> + Send;

    // /// Retrieves a project from the repository.
    // fn get_project(
    //     &self,
    //     req: &GetProjectRequest,
    // ) -> impl Future<Output = Result<Project, GetProjectError>> + Send;

    // /// Updates an existing project in the repository.
    // fn update_project(
    //     &self,
    //     req: &UpdateProjectRequest,
    // ) -> impl Future<Output = Result<(), UpdateProjectError>> + Send;

    // /// Deletes a project from the repository.
    // fn delete_project(
    //     &self,
    //     req: &DeleteProjectRequest,
    // ) -> impl Future<Output = Result<(), DeleteProjectError>> + Send;

    // /// Lists projects from the repository.
    // fn list_projects(
    //     &self,
    //     req: &ListProjectsRequest,
    // ) -> impl Future<Output = Result<Vec<Project>, ListProjectsError>> + Send;

    // /// Creates new component in the repository.
    // fn create_component(
    //     &self,
    //     req: &CreateComponentRequest,
    // ) -> impl Future<Output = Result<ProjectComponent, CreateComponentError>> + Send;

    // /// Retrieves component from the repository.
    // fn get_component(
    //     &self,
    //     req: &GetComponentRequest,
    // ) -> impl Future<Output = Result<ProjectComponent, GetComponentError>> + Send;

    // /// Updates existing component in the repository.
    // fn update_component(
    //     &self,
    //     req: &UpdateComponentRequest,
    // ) -> impl Future<Output = Result<(), UpdateComponentError>> + Send;

    // /// Deletes component from the repository.
    // fn delete_component(
    //     &self,
    //     req: &DeleteComponentRequest,
    // ) -> impl Future<Output = Result<(), DeleteComponentError>> + Send;

    // /// Lists components from the repository.
    // fn list_components(
    //     &self,
    //     req: &ListComponentRequest,
    // ) -> impl Future<Output = Result<Vec<ProjectComponent>, ListComponentError>> + Send;

    /// Creates new content in the repository.
    fn create_document(
        &self,
        req: &CreateDocumentRequest,
    ) -> impl Future<Output = Result<Id, CreateDocumentError>> + Send;

    /// Retrieves document from the repository.
    fn get_document(
        &self,
        req: &GetDocumentRequest,
    ) -> impl Future<Output = Result<Document, GetDocumentError>> + Send;

    /// Updates existing document in the repository.
    fn update_document(
        &self,
        req: &UpdateDocumentRequest,
    ) -> impl Future<Output = Result<(), UpdateDocumentError>> + Send;

    /// Deletes document from the repository.
    fn delete_document(
        &self,
        req: &DeleteDocumentRequest,
    ) -> impl Future<Output = Result<(), DeleteDocumentError>> + Send;
}
