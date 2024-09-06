use serde::Serialize;
use std::{
    future::{ready, Future},
    path::PathBuf,
};
use thiserror::Error;

use crate::{
    project::{
        models::project::{CreateProjectError, CreateProjectRequest, GetProjectError, Project},
        ports::project::{FileSystemProjectRepository, ProjectRepository},
    },
    types::Id,
};
///
/// Service contains functions that more directly relate to the business logic of the application.
/// A service function may call multiple repository functions to accomplish its task.
///
/// It may also publish events or perform other side effects.
///
#[derive(Debug, Clone)]
pub struct DesktopService<FS, DB>
where
    FS: FileSystemProjectRepository,
    DB: ProjectRepository,
{
    repo: DB,
    fs_repo: FS,
}

pub struct WebService<DB>
where
    DB: ProjectRepository,
{
    repo: DB,
}

#[derive(Debug, Serialize, Error)]
#[error("Application error")]
pub struct ServiceError(String);

pub struct ProjectUpdateDto {
    pub name: String,
    pub kind: String,
}

/// ProjectService needs to work on both the Web and Desktop platforms.
///
/// On Desktop, a Project is both the Application file format (a SQLite DB file), and a record in its `project` table.
///
/// This means that the ProjectService needs to be able to create, read, update, and delete both the file and the record.
///
pub trait ProjectService {
    type CreateInput: Send + 'static;
    type GetInput;
    type ListInput;
    type ListOutput;

    /// Project create
    ///
    /// On web, creates a project record in the database. On desktop, creates a project file, then creates a record in its db.
    fn project_create(
        &self,
        req: &Self::CreateInput,
    ) -> impl Future<Output = Result<Id, ServiceError>> + Send;

    /// Project get
    ///
    /// On web, gets a project record by ID. On desktop, gets a project file, then finds the record by ID in its db.
    fn project_get(
        &self,
        req: &Self::GetInput,
    ) -> impl Future<Output = Result<Project, ServiceError>> + Send;

    /// Project update
    ///
    /// Updates the project record in the database.
    fn project_update(
        &self,
        req: Id,
        update_data: ProjectUpdateDto,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send;

    /// Project delete
    ///
    /// Not to be implemented on Desktop
    fn project_delete(&self, req: Id) -> impl Future<Output = Result<(), ServiceError>> + Send {
        ready(Err(ServiceError("Not implemented".to_string())))
    }

    /// Project list
    ///
    /// On web, lists all project records. On desktop, opens a RFD modal to select a project file.
    fn project_list(
        &self,
        req: &Self::ListInput,
    ) -> impl Future<Output = Result<&Self::ListOutput, ServiceError>> + Send;
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectCreateDto {
    pub title: String,
    pub kind: String,
}

impl From<ProjectCreateDto> for CreateProjectRequest {
    fn from(dto: ProjectCreateDto) -> Self {
        CreateProjectRequest::builder()
            .title(dto.title.into())
            .kind(dto.kind.into())
            .build()
    }
}

impl From<CreateProjectError> for ServiceError {
    fn from(error: CreateProjectError) -> Self {
        ServiceError(error.to_string())
    }
}

impl<F, R> ProjectService for DesktopService<F, R>
where
    F: FileSystemProjectRepository,
    R: ProjectRepository,
{
    type CreateInput = ProjectCreateDto;
    type GetInput = PathBuf;
    /// Pass in the default project directory
    type ListInput = PathBuf;
    /// Returns the path for the Project file.
    type ListOutput = PathBuf;

    fn project_create(
        &self,
        req: &Self::CreateInput,
    ) -> impl Future<Output = Result<Id, ServiceError>> + Send {
        // Clone the repositories and the request
        let fs_repo = self.fs_repo.clone();
        let repo = self.repo.clone();
        let create_request: CreateProjectRequest = req.clone().into();

        // Return a future that is Send
        async move {
            // Using fs repo, create .prose file at path.
            fs_repo.new_prosefile().await?;

            // Using db repo, create record with path.
            let project = repo.create_project(&create_request).await?;
            Ok(project.id())
        }
    }

    fn project_get(
        &self,
        req: &Self::GetInput,
    ) -> impl Future<Output = Result<Project, ServiceError>> + Send {
        async move { todo!() }
    }

    fn project_update(
        &self,
        req: Id,
        update_data: ProjectUpdateDto,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send {
        async move { todo!() }
    }

    fn project_list(
        &self,
        req: &Self::ListInput,
    ) -> impl Future<Output = Result<&Self::ListOutput, ServiceError>> + Send {
        async move { todo!() }
    }
}
