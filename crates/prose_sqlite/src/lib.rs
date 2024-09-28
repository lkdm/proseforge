use bon::{bon, builder};
use rusqlite::Connection;
use rusqlite::Error as RusqliteError;
use std::path::PathBuf;
use thiserror::Error;
// TODO Add versioning/cache invalidation
// https://github.com/RandomEngy/tauri-sqlite/blob/main/src-tauri/src/database.rs

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("No path for database provided")]
    InvalidPathError,
    #[error("Database error: {0}")]
    DatabaseError(#[from] RusqliteError),
}

/// SqliteAdapter
///
/// An adapter for the application that implements SQLite.
pub struct SqliteAdapter(Connection);

/// Create an adapter in memory for testing
fn in_memory_adapter() -> SqliteAdapter {
    match Connection::open_in_memory() {
        Ok(cnx) => SqliteAdapter(cnx),
        _ => panic!("Could not open Sqlite connection in memory."),
    }
}

impl SqliteAdapter {
    /// Creates a new SqliteAdapter
    pub fn new(path: PathBuf) -> Result<Self, AdapterError> {
        let cnx = Connection::open(path)?;
        Ok(SqliteAdapter(cnx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> SqliteAdapter {
        in_memory_adapter()
    }

    #[test]
    fn sqliteadapter_init_test() {
        let adapter = setup();
    }
}
