use rusty_ulid::generate_ulid_bytes;
use rusty_ulid::Ulid;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use thiserror::Error;
// TODO: Business logic for node goes here, inc. config.
use chrono::{NaiveDateTime, Utc};
use derive_more::derive::{AsRef, Deref, Display};

// Timestamp for a resource
//
// Because SQLite uses NaiveDateTime, we use that here.
// But it's assumed that this is always in UTC.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, AsRef, Deref, Display)]
pub struct Timestamp(NaiveDateTime);

#[derive(Error, Debug)]
pub enum TimestampError {
    #[error("Timestamp was not in the correct format")]
    IncorrectFormat(#[source] chrono::ParseError),
    #[error("Timestamp database interop error")]
    DatabaseInterop(#[source] sqlx::Error),
    #[error("Timestamp was not the correct length")]
    IncorrectLength,
}

impl Timestamp {
    pub fn from_naive(naive: NaiveDateTime) -> Self {
        Self(naive)
    }
}

impl From<NaiveDateTime> for Timestamp {
    fn from(dt: NaiveDateTime) -> Self {
        Self(dt)
    }
}
impl Timestamp {
    pub fn now() -> Self {
        let utc_now = Utc::now();
        Self(utc_now.naive_utc())
    }
    // Method to convert from Option<NaiveDateTime> to Option<Timestamp>
    pub fn from_option(opt: Option<NaiveDateTime>) -> Option<Self> {
        opt.map(|naive| Self(naive))
    }
}

fn parse_timestamp(s: &str) -> Result<NaiveDateTime, TimestampError> {
    // Determine the format based on the length of the string
    match s.len() {
        // Date only
        10 => NaiveDateTime::parse_from_str(s, "%Y-%m-%d").map_err(TimestampError::IncorrectFormat),
        // Datetime with seconds
        19 => NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
            .map_err(TimestampError::IncorrectFormat),
        // Datetime with fractional seconds
        26 => NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S.%f")
            .map_err(TimestampError::IncorrectFormat),
        _ => Err(TimestampError::IncorrectLength),
    }
}

impl From<String> for Timestamp {
    fn from(s: String) -> Self {
        // Attempt to parse the string into a NaiveDateTime
        let naive =
            parse_timestamp(&s).unwrap_or_else(|_| panic!("Failed to parse timestamp: {}", s));
        Self(naive)
    }
}
impl Into<String> for Timestamp {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

#[derive(Error, Debug)]
pub enum IdError {
    #[error("Invalid ULID")]
    InvalidUlid,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id([u8; 16]);

impl Id {
    pub fn new() -> Self {
        Id(generate_ulid_bytes())
    }
    pub fn from_string(s: &str) -> Result<Self, IdError> {
        let ulid = Ulid::from_str(s).map_err(|_| IdError::InvalidUlid)?;
        Ok(ulid.into())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", Ulid::from(self.0))
    }
}
impl From<Ulid> for Id {
    fn from(ulid: Ulid) -> Self {
        Id(ulid.into())
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        let ulid = Ulid::from_str(&s).unwrap();
        ulid.into()
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        let ulid = Ulid::from_str(s).unwrap();
        ulid.into()
    }
}

impl Into<String> for Id {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl Default for Id {
    fn default() -> Self {
        Id::new()
    }
}

mod tests {
    use super::*;
    use rusty_ulid::generate_ulid_string;
    use std::panic;

    #[test]
    fn test_id_new() {
        let id = Id::new();
        assert_eq!(id.0.len(), 16);
    }

    #[test]
    fn test_id_default() {
        let id = Id::default();
        assert_eq!(id.0.len(), 16);
    }

    #[test]
    fn test_id_display() {
        let id = Id::new();
        let id_str = format!("{}", id);
        assert_eq!(id_str.len(), 26);
    }

    #[test]
    fn test_from_ulid() {
        // Generate a new Ulid
        let ulid = Ulid::generate();
        let id: Id = ulid.into();
        let id_str: String = id.into();
        assert_eq!(id_str, ulid.to_string());
    }

    #[test]
    fn test_from_string() {
        // Generate a new Ulid and convert it to a string
        let ulid_str = generate_ulid_string();
        let id: Id = ulid_str.clone().into();
        assert_eq!(format!("{}", id), ulid_str);
    }

    #[test]
    fn test_from_string_invalid() {
        // Test invalid Ulid string
        let ulid_str = "01D3NQZ1KZQZQZQZQZQZQZQZQZQ";
        // Use panic::catch_unwind to capture any panic
        let result = panic::catch_unwind(|| {
            let _id: Id = ulid_str.to_string().into();
        });

        // Ensure that the code panicked
        assert!(
            result.is_err(),
            "The code should panic for an invalid Ulid string"
        );
    }
}
