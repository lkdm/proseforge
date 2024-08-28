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
        // TODO: Create tables if not exists, etc.
        Ok(Self { connection })
    }
}
