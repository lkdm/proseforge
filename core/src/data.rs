use chrono::{DateTime, Utc};
use derive_more::derive::{AsRef, Deref, Display};
use rusty_ulid::Ulid;
use serde::Deserialize;
use std::fmt;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref, Display, Deserialize,
)]
pub struct Timestamp(DateTime<Utc>);

impl Default for Timestamp {
    fn default() -> Self {
        Self(Utc::now())
    }
}

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
        #[derive(
            Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize, Copy,
        )]
        pub struct $name(Identifier);
        impl ResourceIdentifier for $name {
            fn new() -> Self {
                use rusty_ulid::generate_ulid_bytes;
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
                // remove "id" from the end of the name
                let name_str = stringify!($name);
                let trimmed_name = if name_str.ends_with("Id") {
                    &name_str[..name_str.len() - 2] // Remove "Id"
                } else {
                    name_str
                };
                write!(f, "{}_{}", stringify!($name).to_lowercase(), self.ulid())
            }
        }
    };
}
// Create resource identifiers for the following resources:
create_resource_identifier!(DocumentId);
create_resource_identifier!(ProjectId);
create_resource_identifier!(ChapterId);
create_resource_identifier!(PartId);
create_resource_identifier!(DraftId);
create_resource_identifier!(NoteId);
create_resource_identifier!(SceneId);
