// TODO: Business logic for node goes here, inc. config.
use chrono::{NaiveDateTime, Utc};
use derive_more::derive::{AsRef, Deref, Display};
use thiserror::Error;

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
