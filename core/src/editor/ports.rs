use std::future::Future;

use super::models::{
    CreateDocumentError, CreateDocumentRequest, Document, GetDocumentError, GetDocumentRequest,
    UpdateDocumentError, UpdateDocumentRequest,
};

/// The document repository persists the document to a data store.
pub trait DocumentRepository: Clone + Send + Sync + 'static {
    fn create_document(
        &self,
        req: &CreateDocumentRequest,
    ) -> impl Future<Output = Result<Document, CreateDocumentError>> + Send;
    fn get_document(
        &self,
        req: &GetDocumentRequest,
    ) -> impl Future<Output = Result<Document, GetDocumentError>> + Send;
    fn update_document(
        &self,
        req: &UpdateDocumentRequest,
    ) -> impl Future<Output = Result<(), UpdateDocumentError>> + Send;
}

pub trait InMemoryDocumentRepository: Clone + Send + Sync + 'static {
    fn update_content(
        &self,
        req: &UpdateDocumentRequest,
    ) -> impl Future<Output = Result<(), UpdateDocumentError>> + Send;

    fn get_content(
        &self,
        req: &GetDocumentRequest,
    ) -> impl Future<Output = Result<Document, GetDocumentError>> + Send;
}
