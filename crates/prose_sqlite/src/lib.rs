use bon::{bon, builder};
use migrations::get_migrations;
use rusqlite::Connection;
use rusqlite::Error as RusqliteError;
use std::path::PathBuf;
use thiserror::Error;
mod migrations;
// TODO Add versioning/cache invalidation
// https://github.com/RandomEngy/tauri-sqlite/blob/main/src-tauri/src/database.rs

// const CURRENT_DB_VERSION: u32 = 0;

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

#[bon]
impl SqliteAdapter {
    /// Creates a new SqliteAdapter
    #[builder]
    pub fn new(path: PathBuf) -> Result<Self, AdapterError> {
        let cnx = Connection::open(path)?;
        Ok(SqliteAdapter(cnx))
    }
    /// Initialises the database
    #[builder]
    pub fn init(&mut self) -> Result<(), rusqlite::Error> {
        let mut user_pragma = self.0.prepare("PRAGMA user_version")?;
        let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
        drop(user_pragma);
        self.run_migrations_if_needed(existing_user_version)?;
        Ok(())
    }

    /// Runs migrations if necessary
    fn run_migrations_if_needed(&mut self, user_ver: u32) -> Result<(), rusqlite::Error> {
        let migrations = get_migrations();
        let current_db_version: u32 = migrations.len() as u32;
        let remaining_migrations = &migrations[user_ver as usize..current_db_version as usize];

        self.0.pragma_update(None, "journal_mode", "WAL")?;
        let tx = self.0.transaction()?;
        tx.pragma_update(None, "user_version", current_db_version)?;
        for migration in remaining_migrations {
            tx.execute_batch(&migration.up)?;
        }
        tx.commit()?;

        Ok(())
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
