use bon::{bon, builder};
use rusqlite::Connection;
use rusqlite::Error as RusqliteError;
use std::path::PathBuf;
use thiserror::Error;
mod fs;
// TODO Add versioning/cache invalidation
// https://github.com/RandomEngy/tauri-sqlite/blob/main/src-tauri/src/database.rs

#[derive(Debug, Error)]
enum AdapterError {
    #[error("No path for database provided")]
    InvalidPathError,
    #[error("Database error: {0}")]
    DatabaseError(#[from] RusqliteError),
    #[error("In-memory specified yet path provided")]
    InMemoryWithPathError,
}

/// SqliteAdapter
///
/// An adapter for the application that implements SQLite.
struct SqliteAdapter(Connection);

#[bon]
impl SqliteAdapter {
    #[builder]
    fn path(path: PathBuf) -> Result<Self, AdapterError> {
        let connection = Connection::open(path)?;
        Ok(SqliteAdapter(connection))
    }

    #[builder]
    fn in_memory() -> Result<Self, AdapterError> {
        let connection = Connection::open_in_memory()?;
        Ok(SqliteAdapter(connection))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqliteadapter_init_test() {
        let adapter = SqliteAdapter::in_memory().call();
    }
}
