use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Display, Formatter},
    io,
    string::FromUtf8Error,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("I/O Error: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },
    #[error("File Read Error: Could not convert the selected file to a string.")]
    FromUtf8Error {
        #[from]
        source: FromUtf8Error,
    },
    #[error("Save Error: The file path is missing. Please provide a file path.")]
    NoSavePath,
    #[error("Open Error: The file path is missing. Please provide a file path.")]
    NoOpenPath,
    #[error(
        "Arc Error: Multiple references to the same Arc. Please clone the Arc before using it."
    )]
    MultipleArcReferences,
    #[error("Blocking Error: The thread has been blocked. Please check the thread for errors.")]
    BlockingError,
    #[error("Unknown Error: {message}")]
    Unknown {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error>>,
    },
}

impl serde::Serialize for NodeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
