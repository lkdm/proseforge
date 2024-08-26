use std::path::PathBuf;

use crate::domain::editor::{
    models::{
        CreateDocumentError, CreateDocumentRequest, Document, GetDocumentError, GetDocumentRequest,
        UpdateDocumentError, UpdateDocumentRequest,
    },
    ports::DocumentRepository,
};

#[derive(Clone)]
pub struct FileSystem {
    path: PathBuf,
}

impl FileSystem {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    fn read_file(&self) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(self.get_path())
    }

    fn write_file(&self, content: &[u8]) -> Result<(), std::io::Error> {
        std::fs::write(self.get_path(), content)
    }

    fn create_file(&self) -> Result<(), std::io::Error> {
        std::fs::File::create(self.get_path())?;
        Ok(())
    }

    fn to_utf8(&self, content: Vec<u8>) -> String {
        String::from_utf8(content).unwrap()
    }
}

impl DocumentRepository for FileSystem {
    async fn create_document(
        &self,
        _req: &CreateDocumentRequest,
    ) -> Result<Document, CreateDocumentError> {
        self.create_file();
        self.write_file(_req.content().as_bytes());

        let document = Document::builder()
            .with_content(_req.content().clone())
            .saved_now()
            .build();

        Ok(document)
    }

    async fn get_document(&self, _req: &GetDocumentRequest) -> Result<Document, GetDocumentError> {
        let content = self.read_file().unwrap();
        let content = self.to_utf8(content);
        let document = Document::builder()
            .with_content(content.into())
            .saved_now()
            .build();

        Ok(document)
    }

    async fn update_document(
        &self,
        _req: &UpdateDocumentRequest,
    ) -> Result<(), UpdateDocumentError> {
        self.write_file(_req.content().as_bytes());
        Ok(())
    }
}
