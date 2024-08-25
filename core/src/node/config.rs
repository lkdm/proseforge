use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Theme {
    System = 0,
    Light = 1,
    Dark = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UserPreferences {
    pub theme: Theme,
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    project_directory: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NodeConfigVersion {
    V0 = 0,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NodeConfig {
    pub preferences: UserPreferences,
    version: NodeConfigVersion,
}

impl NodeConfig {
    pub fn default() -> Self {
        Self {
            preferences: UserPreferences {
                theme: Theme::Dark,
                #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
                project_directory: PathBuf::new(),
            },
            version: NodeConfigVersion::V0,
        }
    }
}
