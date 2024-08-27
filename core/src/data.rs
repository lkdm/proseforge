use rusty_ulid::Ulid;
use chrono::{DateTime, Utc};
use derive_more::derive::{AsRef, Deref, Display};
use serde::Deserialize;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref, Display, Deserialize,
)]
pub struct Timestamp(DateTime<Utc>);

impl Default for Timestamp {
    fn default() -> Self {
        Self(Utc::now())
    }
};


/// Ulid is a 128-bit Universally Unique Lexicographically Sortable Identifier
/// https://github.com/ulid/spec
/// We use it as a unique identifier for resources in the application.
/// Use resource_identifier!() to create a new resource identifier, passing in the name of the identifier.
pub type Identifier = [u8; 16];
pub trait ResourceIdentifier {
    fn new() -> Self;
    fn ulid(&self) -> Ulid;
    fn datetime(&self) -> DateTime<Utc>;
}

#[macro_export]
macro_rules! create_resource_identifier {
    ($name:ident) => {
        use crate::data::Identifier;
        use crate::data::ResourceIdentifier;
        use rusty_ulid::generate_ulid_bytes;
        use rusty_ulid::Ulid;
        use chrono::{DateTime, Utc};
        use std::fmt;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize)]
        pub struct $name(Identifier);
        impl ResourceIdentifier for $name {
            fn new () -> Self {
                Self(generate_ulid_bytes())
            }

            fn ulid(&self) -> Ulid {
                Ulid::from(self.0)
            }

            fn datetime(&self) -> DateTime<Utc> {
                self.ulid().datetime()
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}_{}", stringify!($name).to_lowercase(), self.ulid())
            }
        }
    };
};
