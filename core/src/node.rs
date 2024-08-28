pub mod models;
pub mod ports;

// TODO: Business logic for node goes here, inc. config.
use chrono::{DateTime, Utc};
use derive_more::derive::{AsRef, Deref, Display};
use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, Value, ValueRef};
use rusqlite::Result as RusqliteResult;
use rusty_ulid::{generate_ulid_bytes, Ulid};
use serde::Deserialize;
use std::fmt;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref, Display, Deserialize,
)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self(Utc::now())
    }
}

/// An ID for a resource.
/// Usage: `struct MyResourceId(Id)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id([u8; 16]);
impl Id {
    pub fn new() -> Self {
        Self(generate_ulid_bytes())
    }
    fn bytes(&self) -> [u8; 16] {
        self.0
    }
    fn ulid(&self) -> Ulid {
        Ulid::from(self.0)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ulid())
    }
}
impl FromSql for Id {
    fn column_result(value: ValueRef) -> Result<Self, FromSqlError> {
        match value {
            ValueRef::Blob(blob) if blob.len() == 16 => {
                let mut array = [0u8; 16];
                array.copy_from_slice(blob);
                Ok(Id(array))
            }
            _ => Err(FromSqlError::InvalidType), // Use FromSqlError for invalid type
        }
    }
}

impl From<Id> for Vec<u8> {
    fn from(id: Id) -> Vec<u8> {
        id.0.to_vec() // Convert the array to a Vec<u8>
    }
}

impl ToSql for Id {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput<'_>> {
        // Convert the [u8; 16] array to Vec<u8>
        let blob = self.0.to_vec();
        // Create a Value::Blob from the Vec<u8>
        // let value = Value::Blob(blob);
        // Wrap the Value::Blob in ToSqlOutput
        Ok(ToSqlOutput::from(blob))
    }
}
