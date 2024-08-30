// TODO: Business logic for node goes here, inc. config.
use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::derive::{AsRef, Deref, Display};
use thiserror::Error;

// Timestamp for a resource
//
// Because SQLite uses NaiveDateTime, we use that here.
// But it's assumed that this is always in UTC.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, AsRef, Deref, Display)]
pub struct Timestamp(NaiveDateTime);

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

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}
