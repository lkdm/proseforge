use rusqlite::{Connection, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SqliteAdapterError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
}

struct SqliteAdapter {
    connection: Connection,
}

impl SqliteAdapter {
    pub fn new(database_url: &str) -> Result<Self> {
        let connection = Connection::open(database_url)?;
        connection.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS project (
                id ULID PRIMARY KEY,
                title TEXT NOT NULL,
                component_ids TEXT NOT NULL,

                created_at TIMESTAMP NOT NULL,
                saved_at TIMESTAMP,
                modified_at TIMESTAMP,
                deleted_at TIMESTAMP,
            );
            CREATE TABLE IF NOT EXISTS content (
                id ULID PRIMARY KEY,
                data TEXT NOT NULL,

                created_at TIMESTAMP NOT NULL,
                saved_at TIMESTAMP,
                modified_at TIMESTAMP,
                deleted_at TIMESTAMP,
            );
            CREATE TABLE IF NOT EXISTS component (
                id ULID PRIMARY KEY,
                kind TEXT NOT NULL,
                component_ids TEXT NOT NULL,
                content_id ULID,
                summary_id ULID,
                document_id ULID,

                created_at TIMESTAMP NOT NULL,
                saved_at TIMESTAMP,
                modified_at TIMESTAMP,
                deleted_at TIMESTAMP,
            );
        ",
        );
        Ok(Self { connection })
    }
}
pub struct Component {
    id: ComponentId,
    kind: ComponentKind,
    components: Vec<ComponentId>,
    parent: Option<ComponentId>,

    summary: Option<ContentId>,
    document: Option<ContentId>,
    // TODO: comments, maybe by line-location, etc
    // comments: Vec<ContentId>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct Project {
    id: ProjectId,
    title: Title,
    components: Vec<ComponentId>,
}
