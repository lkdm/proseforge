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
    fn kind(&self) -> String;
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

            fn kind(&self) -> String {
                let name = stringify!($name).to_lowercase();
                name[..name.len() - 2].into()
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let name_str = stringify!($name);
                // remove "id" from the end of the name
                let trimmed_name = &name_str[..name_str.len() - 2];
                write!(f, "{}_{}", trimmed_name.to_lowercase(), self.ulid())
            }
        }
        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                // Split the string to separate the prefix and the ULID
                let parts: Vec<&str> = s.split('_').collect();
                if parts.len() != 2 {
                    panic!("Invalid format: {}", s);
                }

                // Ensure the prefix matches the expected identifier name
                let prefix = stringify!($name).to_lowercase();
                let prefix = &prefix[..prefix.len() - 2];

                if parts[0] != prefix {
                    panic!("Expected prefix '{}', found '{}'", prefix, parts[0]);
                }

                // Parse the ULID from the second part
                let ulid_str = parts[1];
                let ulid: Ulid = (ulid_str)
                    .parse()
                    .expect(&format!("Invalid ULID: {}", ulid_str));

                // Create a new instance of the ID struct
                $name(Identifier::from(ulid))
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

mod test {
    use super::*;
    #[test]
    fn test_resource_identifier() {
        let id = DocumentId::new();
        println!("{}", id);
        assert_eq!(id.ulid().to_string().len(), 26);

        // Ensure name is no id in the name
        let id_str = id.to_string();
        assert!(
            !id_str.contains("id"),
            "Identifier string should not contain 'id'"
        );
        assert!(
            id_str.starts_with("document_"),
            "Identifier string should start with 'document_'"
        );

        // Ensure the ID can be parsed back into a DocumentId
        let parsed_id: DocumentId = id_str.as_str().into();
        assert_eq!(id, parsed_id);

        // Ensure the kind is correct
        assert_eq!(id.kind(), "document");
    }
}
