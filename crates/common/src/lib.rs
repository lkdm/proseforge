use rusty_ulid::generate_ulid_bytes;
use rusty_ulid::Ulid;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use thiserror::Error;

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
