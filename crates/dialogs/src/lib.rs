use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult};
use std::path::PathBuf;

pub fn request_save_path_dialog(dir: Option<PathBuf>) -> Option<PathBuf> {
    let dir = match dir {
        Some(dir) => dir,
        None => std::env::current_dir().unwrap(),
    };
    FileDialog::new()
        .set_directory(dir)
        .save_file()
        .map(|file_handle| file_handle.to_path_buf()) // Convert to PathBuf if Some
}

pub fn request_open_path_dialog(dir: Option<PathBuf>) -> Option<PathBuf> {
    let dir = match dir {
        Some(dir) => dir,
        None => std::env::current_dir().unwrap(),
    };
    FileDialog::new()
        .set_directory(dir)
        .pick_file()
        .map(|file_handle| file_handle.to_path_buf()) // Convert to PathBuf if Some
}

/// Opens a dialog warning the user of unsaved changes
pub fn request_ignore_unsaved_changes_dialog() -> bool {
    let dialog = MessageDialog::new()
        .set_title("Unsaved Changes")
        .set_description("You have unsaved changes. Do you want to continue?")
        .set_buttons(MessageButtons::YesNo);

    dialog.show() == MessageDialogResult::Yes // Returns true if user selects Yes
}
