use chrono::Utc;
use serde::Serialize;

// Listen to events use async/await.
#[derive(Debug, Clone, Serialize)]
pub enum CoreEvent {
    DocumentLoad { content: String, timestamp: u32 },
}

fn timestamp() -> u32 {
    Utc::now().timestamp() as u32
}

impl CoreEvent {
    pub fn document_load(content: String) -> Self {
        CoreEvent::DocumentLoad {
            content,
            timestamp: timestamp(),
        }
    }
}
