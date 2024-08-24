use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::System,
        }
    }
}
