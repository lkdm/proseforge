use std::sync::{Arc, Mutex};

use pf_core::{
    editor::{
        models::{ContentId, CreateContentError, CreateContentRequest},
        ports::ContentRepository,
    },
    node::Id,
};
use rusqlite::{params, Connection, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SqliteAdapterError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
}

#[derive(Clone, Debug)]
struct SqliteAdapter {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteAdapter {
    pub fn new(database_url: &str) -> Result<Self> {
        let connection = Connection::open(database_url)?;
        connection.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS project (
                id ULID PRIMARY KEY DEFAULT ulid(),
                title TEXT NOT NULL,
                component_ids TEXT NOT NULL,

                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
                modified_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                deleted_at TIMESTAMP,
            );
            CREATE TABLE IF NOT EXISTS content (
                id ULID PRIMARY KEY DEFAULT ulid(),
                data TEXT NOT NULL,

                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
                modified_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                deleted_at TIMESTAMP,
            );
            CREATE TABLE IF NOT EXISTS component (
                id ULID PRIMARY KEY DEFAULT ulid(),
                kind TEXT NOT NULL,
                component_ids TEXT NOT NULL,
                content_id ULID,
                summary_id ULID,
                document_id ULID,

                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
                modified_at TIMESTAMP,
                deleted_at TIMESTAMP,
            );
        ",
        );
        let connection = Arc::new(Mutex::new(connection));
        Ok(Self { connection })
    }
}

// PRR: ProjectRepository,
// CMP: ComponentRepository,
// COR: ContentRepository,

impl ContentRepository for SqliteAdapter {
    async fn create_content(
        &self,
        _req: &CreateContentRequest,
    ) -> Result<ContentId, CreateContentError> {
        let conn = self.connection.lock().unwrap();
        let id: Id = conn.query_row(
            "
            INSERT INTO content (data, created_at, modified_at)
            VALUES (?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            RETURNING id
            ",
            params![_req.data],
            |row| row.get(0), // Extract the ID from the row
        )?;
        Ok(ContentId::new(id))
    }
}

// CREATE TABLE IF NOT EXISTS content (
//     id ULID PRIMARY KEY DEFAULT ulid(),
//     data TEXT NOT NULL,

//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
//     modified_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
//     deleted_at TIMESTAMP,
// );
