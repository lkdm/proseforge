// use crate::{data::Persist, data::StorageMedium, error::NodeError};
// use directories::ProjectDirs;
// use serde::{Deserialize, Serialize};
// use serde_json::{json, Map, Value};
// use std::{fs::File, io, path::PathBuf};

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub enum Theme {
//     System = 0,
//     Light = 1,
//     Dark = 2,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub struct UserPreferences {
//     pub theme: Theme,
//     #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
//     project_directory: PathBuf,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub enum NodeConfigVersion {
//     V0 = 0,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub struct NodeConfig {
//     pub preferences: UserPreferences,
//     version: NodeConfigVersion,
// }

// impl NodeConfig {
//     pub fn default() -> Self {
//         Self {
//             preferences: UserPreferences {
//                 theme: Theme::Dark,
//                 #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
//                 project_directory: PathBuf::new(),
//             },
//             version: NodeConfigVersion::V0,
//         }
//     }
// }

// fn project_path_buf() -> Option<PathBuf> {
//     {
//         match ProjectDirs::from("com", "lkdm", "Prose Forge") {
//             Some(proj_dirs) => Some(proj_dirs.config_dir().to_path_buf()),
//             None => None,
//         }
//     }
// }

// struct ConfigManager {
//     config: NodeConfig,
//     storage: StorageMedium,
// }

// impl ConfigManager {
//     pub(crate) async fn new() -> Result<Self, NodeError> {
//         let storage = match project_path_buf() {
//             Some(path) => StorageMedium::File(path.join("config.json")),
//             None => return Err(NodeError::NoSavePath),
//         };
//         let config = match storage.read().await {
//             Ok(read_result) => {
//                 let data = read_result.data;
//                 let config: NodeConfig = match serde_json::from_slice(&data) {
//                     Ok(config) => config,
//                     Err(_) => NodeConfig::default(),
//                 };
//                 config
//             }
//             Err(_) => NodeConfig::default(),
//         };
//         Ok(Self { config, storage })
//     }
// }
