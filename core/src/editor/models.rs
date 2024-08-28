pub mod component;
pub mod content;
pub mod project;

pub struct Title(String);

// pub struct DocumentBuilder {
//     id: Option<DocumentId>,
//     content: Option<Content>,
//     saved_at: Option<Timestamp>,
//     modified_at: Option<Timestamp>,
// }

// impl DocumentBuilder {
//     pub fn new() -> Self {
//         Self {
//             id: None,
//             content: None,
//             saved_at: None,
//             modified_at: None,
//         }
//     }
//     pub fn with_content(mut self, content: Content) -> Self {
//         self.content = Some(content);
//         self
//     }
//     pub fn saved_now(mut self) -> Self {
//         self.saved_at = Some(Timestamp::default());
//         self
//     }
//     pub fn modified_now(mut self) -> Self {
//         self.modified_at = Some(Timestamp::default());
//         self
//     }
//     pub fn with_id(mut self, id: DocumentId) -> Self {
//         self.id = Some(id);
//         self
//     }
//     pub fn generate_id(mut self) -> Self {
//         self.id = Some(DocumentId::default());
//         self
//     }
//     pub fn build(self) -> Document {
//         Document {
//             id: self.id.unwrap(),
//             content: self.content.unwrap(),
//             saved_at: self.saved_at,
//             modified_at: self.modified_at,
//         }
//     }
// }

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
// pub struct CreateDocumentRequest {
//     content: Content,
// }

// impl CreateDocumentRequest {
//     pub fn new(content: Content) -> Self {
//         Self { content }
//     }

//     pub fn content(&self) -> &Content {
//         &self.content
//     }
// }

// #[derive(Debug, Error)]
// pub enum CreateDocumentError {
//     #[error("Unknown error")]
//     UnknownError,
// }

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
// pub struct UpdateDocumentRequest {
//     id: DocumentId,
//     content: Content,
// }

// impl UpdateDocumentRequest {
//     pub fn new(id: DocumentId, content: Content) -> Self {
//         Self { id, content }
//     }

//     pub fn id(&self) -> DocumentId {
//         self.id
//     }

//     pub fn content(&self) -> &Content {
//         &self.content
//     }
// }

// #[derive(Debug, Error)]
// pub enum UpdateDocumentError {
//     #[error("Unknown error")]
//     UnknownError,
// }

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
// pub struct GetDocumentRequest {
//     id: DocumentId,
// }

// #[derive(Debug, Error)]
// pub enum GetDocumentError {
//     #[error("Unknown error")]
//     UnknownError,
// }
