///
/// Ports are the interfaces that the application uses to interact with the outside world.
use std::{future::Future, path::PathBuf};

use super::{
    models::{
        CreateProjectError, CreateProjectRequest, DeleteProjectError, DeleteProjectRequest,
        GetProjectError, GetProjectRequest, ListProjectsError, ListProjectsRequest,
        UpdateProjectError, UpdateProjectRequest,
    },
    Project,
};

/// ProjectRepository
///
/// CRUD operations on a project in the database.
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

/// ProseFileRepository
///
/// CRUD operations on a project file in the file system.
pub trait ProseFileRepository: Clone + Send + Sync + 'static {
    /// Creates a new project in the file system.
    fn create_prose_file(
        &self,
        req: &CreateProjectRequest,
    ) -> impl Future<Output = Result<PathBuf, CreateProjectError>> + Send;

    /// Reads an existing project from the file system.
    fn read_prose_file(
        &self,
        path: &PathBuf,
    ) -> impl Future<Output = Result<Project, GetProjectError>> + Send;

    /// Updates an existing project file in the file system.
    fn update_prose_file(
        &self,
        path: &PathBuf,
        req: &UpdateProjectRequest,
    ) -> impl Future<Output = Result<(), UpdateProjectError>> + Send;

    /// Deletes a project file from the file system.
    fn delete_prose_file(
        &self,
        path: &PathBuf,
    ) -> impl Future<Output = Result<(), DeleteProjectError>> + Send;

    /// Lists all projects in a given directory.
    fn list_prose_files(
        &self,
        directory: &PathBuf,
    ) -> impl Future<Output = Result<Vec<PathBuf>, ListProjectsError>> + Send;
}
