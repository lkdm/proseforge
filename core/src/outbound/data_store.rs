use std::sync::{Arc, Mutex};

use crate::domain::editor::{models::Document, ports::DocumentRepository};

// DataStore just holds a reference
#[derive(Debug, Clone)]
pub struct DataStore<T: Clone> {
    data: Arc<Mutex<T>>,
}

impl<T: Clone> DataStore<T> {
    /// Create a new DataStore with initial data
    fn new(data: T) -> Self {
        DataStore {
            data: Arc::new(Mutex::new(data)),
        }
    }

    /// Write new data to the DataStore
    fn write(&self, data: T) {
        let mut locked_data = self.data.lock().unwrap();
        *locked_data = data;
    }

    /// Read data from the DataStore
    fn read(&self) -> T {
        let locked_data = self.data.lock().unwrap();
        locked_data.clone()
    }
}

impl DocumentRepository for DataStore<Document> {
    async fn create_document(
        &self,
        req: &crate::domain::editor::models::CreateDocumentRequest,
    ) -> Result<Document, crate::domain::editor::models::CreateDocumentError> {
        // Example implementation (replace with actual logic)
        let new_document = Document::builder()
            .with_content(req.content().clone())
            .saved_now()
            .build();

        self.write(new_document.clone()); // Store the new document
        Ok(new_document)
    }

    async fn get_document(
        &self,
        req: &crate::domain::editor::models::GetDocumentRequest,
    ) -> Result<Document, crate::domain::editor::models::GetDocumentError> {
        // Example implementation (replace with actual logic)
        let document = self.read(); // Read the document
        Ok(document) // Return the document
    }

    async fn update_document(
        &self,
        req: &crate::domain::editor::models::UpdateDocumentRequest,
    ) -> Result<(), crate::domain::editor::models::UpdateDocumentError> {
        // Example implementation (replace with actual logic)
        let mut document = self.read(); // Read the current document
        document.set_content(req.content().clone()); // Update the content
        self.write(document); // Write the updated document
        Ok(())
    }
}
