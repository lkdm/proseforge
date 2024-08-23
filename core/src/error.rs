use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Display, Formatter},
    io,
    string::FromUtf8Error,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreError {
    title: String,
    message: String,
    detail: Option<String>,
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.detail {
            Some(detail) => write!(
                f,
                "An error has occured in Core:/n/n{}/n/n{}/n/n{}",
                self.title, self.message, detail
            ),
            None => write!(
                f,
                "An error has occured in Core:/n/n{}/n/n{}",
                self.title, self.message
            ),
        }
    }
}

impl Default for CoreError {
    fn default() -> Self {
        let title: String = String::from("Unknown Error");
        let message: String = String::from("An error has occured.");
        let detail: Option<String> = Some(String::from("Error code: 0x00000000"));
        Self {
            title,
            message,
            detail,
        }
    }
}

impl From<io::Error> for CoreError {
    fn from(io_error: io::Error) -> Self {
        let title = String::from("I/O Error");
        let message = io_error.kind().to_string();
        let detail = None;
        Self {
            title,
            message,
            detail,
        }
    }
}

impl From<FromUtf8Error> for CoreError {
    fn from(utf8_error: FromUtf8Error) -> Self {
        let title = String::from("File Read Error");
        let message = String::from("Could not convert the selected file to a string.");
        let detail = Some(utf8_error.to_string());
        Self {
            title,
            message,
            detail,
        }
    }
}

impl CoreError {
    pub fn no_save_path() -> Self {
        let title = String::from("Save Error");
        let message = String::from("The file path is missing.");
        let detail = Some(String::from("Please provide a file path."));
        Self {
            title,
            message,
            detail,
        }
    }
    pub fn no_open_path() -> Self {
        let title = String::from("Open Error");
        let message = String::from("The file path is missing.");
        let detail = Some(String::from("Please provide a file path."));
        Self {
            title,
            message,
            detail,
        }
    }
    pub fn multiple_arc_references() -> Self {
        let title = String::from("Arc Error");
        let message = String::from("Multiple references to the same Arc.");
        let detail = Some(String::from("Please clone the Arc before using it."));
        Self {
            title,
            message,
            detail,
        }
    }
}
