use crate::domain::editor::{
    models::{
        Document, GetDocumentError, GetDocumentRequest, UpdateDocumentError, UpdateDocumentRequest,
    },
    ports::InMemoryDocumentRepository,
};
use std::{future::Future, sync::Arc, sync::Mutex};
// use tokio::sync::Mutex;

// DataStore just holds a reference
#[derive(Debug, Clone)]
pub struct DataStore<T: Clone> {
    data: Arc<Mutex<T>>,
}

impl<T: Clone> DataStore<T> {
    /// Create a new DataStore with initial data
    pub fn new(data: T) -> Self {
        DataStore {
            data: Arc::new(Mutex::new(data)),
        }
    }

    /// Write new data to the DataStore asynchronously
    fn write(&self, data: T) {
        let mut locked_data = self.data.lock().unwrap(); // Await the lock acquisition
        *locked_data = data;
    }

    /// Read data from the DataStore asynchronously
    fn read(&self) -> T {
        let locked_data = self.data.lock().unwrap(); // Await the lock acquisition
        locked_data.clone()
    }
}

impl InMemoryDocumentRepository for DataStore<Document> {
    async fn update_content(
        &self,
        _req: &UpdateDocumentRequest,
    ) -> Result<(), UpdateDocumentError> {
        let mut locked_data = self.read(); // Await the read
        locked_data.set_content(_req.content().clone().into());
        self.write(locked_data); // Await the write
        Ok(())
    }

    async fn get_content(&self, _req: &GetDocumentRequest) -> Result<Document, GetDocumentError> {
        let locked_data = self.read(); // Await the read
        Ok(locked_data)
    }
}
