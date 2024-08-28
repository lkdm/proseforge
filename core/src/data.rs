use chrono::{DateTime, Utc};
use derive_more::derive::{AsRef, Deref, Display};
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
