///
/// Ports are the interfaces that the application uses to interact with the outside world.
use crate::{
    editor::document::models::{
        CreateDocumentError, CreateDocumentRequest, DeleteDocumentError, DeleteDocumentRequest,
        GetDocumentError, GetDocumentRequest, UpdateDocumentError, UpdateDocumentRequest,
    },
    types::Id,
};
use std::future::Future;

use super::Document;

/// ProjectRepository
pub trait DocumentRepository: Clone + Send + Sync + 'static {
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
