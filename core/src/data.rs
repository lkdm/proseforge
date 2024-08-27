use chrono::{DateTime, Utc};
use derive_more::derive::{AsRef, Deref, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref, Display)]
pub struct Timestamp(DateTime<Utc>);

impl Default for Timestamp {
    fn default() -> Self {
        Self(Utc::now())
    }
}
