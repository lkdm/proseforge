///
/// Ports are the interfaces that the application uses to interact with the outside world.
use std::future::Future;

use super::{
    models::{
        CreateProjectError, CreateProjectRequest, DeleteProjectError, DeleteProjectRequest,
        GetProjectError, GetProjectRequest, ListProjectsError, ListProjectsRequest,
        UpdateProjectError, UpdateProjectRequest,
    },
    Project,
};

/// ProjectRepository
pub trait ProjectRepository: Clone + Send + Sync + 'static {
    /// Creates a new project in the repository.
    fn create_project(
        &self,
        req: &CreateProjectRequest,
    ) -> impl Future<Output = Result<Project, CreateProjectError>> + Send;

    /// Retrieves a project from the repository.
    fn get_project(
        &self,
        req: &GetProjectRequest,
    ) -> impl Future<Output = Result<Project, GetProjectError>> + Send;

    /// Updates an existing project in the repository.
    fn update_project(
        &self,
        req: &UpdateProjectRequest,
    ) -> impl Future<Output = Result<(), UpdateProjectError>> + Send;

    /// Deletes a project from the repository.
    fn delete_project(
        &self,
        req: &DeleteProjectRequest,
    ) -> impl Future<Output = Result<(), DeleteProjectError>> + Send;

    /// Lists projects from the repository.
    fn list_projects(
        &self,
        req: &ListProjectsRequest,
    ) -> impl Future<Output = Result<Vec<Project>, ListProjectsError>> + Send;
}
