use std::{fs::File, path::PathBuf};
use tauri::{AppHandle, Wry};
use tauri_plugin_dialog::{DialogExt, FilePath};
use thiserror::Error;

#[derive(Debug, Error)]
enum ProsefileError {
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("URL Path is not implemented")]
    UrlPath,
    #[error("No path provided")]
    NoPath,
}

/// Prompts the user to open a prosefile at a location
pub fn open_prosefile<F>(handle: &AppHandle<Wry>, callback: F)
where
    F: FnOnce(Option<FilePath>) + Send + 'static,
{
    handle
        .dialog()
        .file()
        .set_title("Open Prosefile")
        .add_filter("Prosefiles", &["prose"])
        .pick_file(move |file_path| {
            callback(file_path);
        });
}

/// Prompts the user to create a new prosefile
///
pub fn new_prosefile<F>(handle: &AppHandle<Wry>, callback: F)
where
    F: FnOnce(Option<PathBuf>) + Send + 'static,
{
    // TODO: I think the dialog message needs to be moved to this base level.
    handle
        .dialog()
        .file()
        .set_title("New Prosefile")
        .set_can_create_directories(true)
        .add_filter("Prosefiles", &["prose"])
        .save_file(move |file_path| {
            return match file_path {
                Some(FilePath::Path(path)) => {
                    let _ = File::create_new(&path).map_err(ProsefileError::IoError);
                    callback(Some(path))
                }
                _ => (),
            };
        });
}
