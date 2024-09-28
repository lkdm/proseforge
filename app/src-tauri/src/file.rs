use std::path::PathBuf;

use tauri::{AppHandle, Wry};
use tauri_plugin_dialog::{DialogExt, FilePath};

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
    F: FnOnce(Option<FilePath>) + Send + 'static,
{
    handle
        .dialog()
        .file()
        .set_title("New Prosefile")
        .set_can_create_directories(true)
        .add_filter("Prosefiles", &["prose"])
        .save_file(move |file_path| {
            callback(file_path);
        });
}
