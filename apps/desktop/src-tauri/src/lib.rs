use md_core::config::{Config, Theme};
use md_core::data::*;
use md_core::error::NodeError;
use md_core::event::CoreEvent;
use md_core::Node;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::menu::{
    AboutMetadata, AboutMetadataBuilder, CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder,
    PredefinedMenuItem, SubmenuBuilder,
};
use tauri::{async_runtime::block_on, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::DialogExt;
use tokio::task::block_in_place;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
async fn handle_update_content(
    content: String,
    state: tauri::State<'_, Mutex<Node>>,
) -> Result<(), NodeError> {
    state.lock().unwrap().handle_update_content(content)?;
    Ok(())
}

#[tauri::command]
async fn handle_open_dialog(
    app: AppHandle,
    state: tauri::State<'_, Mutex<Node>>,
) -> Result<(), NodeError> {
    let path = open_file_dialog()?;
    let state = state.lock().unwrap();
    let mut txt = state.editor.lock().unwrap();
    txt.set_save_location(path);
    txt.load()?;
    let evt = CoreEvent::document_load(txt.get_content());
    app.emit("file-opened", evt).unwrap();
    Ok(())
}

#[tauri::command]
async fn handle_save(state: tauri::State<'_, Mutex<Node>>) -> Result<(), NodeError> {
    let state = state.lock().unwrap();
    let result = {
        // Lock the editor and attempt to save
        match state.handle_save() {
            Ok(()) => Ok(()),
            Err(NodeError::NoSavePath) => {
                // Open a save location dialog
                let path = match open_file_save_dialog() {
                    Ok(path) => path,
                    Err(_) => return Ok(()), // User cancelled the dialog
                };

                // Re-acquire the lock on editor to update the save location and retry saving
                let mut editor = state.editor.lock().unwrap();
                editor.set_save_location(path);

                // Retry saving
                editor.save()
            }
            Err(e) => Err(e),
        }
    };
    result
}

#[tauri::command]
async fn get_config(state: tauri::State<'_, Mutex<Node>>) -> Result<Config, NodeError> {
    let state = state.lock().map_err(|_| NodeError::BlockingError)?;
    let config = state.config.clone();
    Ok(config.as_ref().clone())
}

#[tauri::command]
async fn handle_new_file(
    app: AppHandle,
    state: tauri::State<'_, Mutex<Node>>,
) -> Result<(), NodeError> {
    // Check if the current document has unsaved changes
    // TODO: build dialogue for unsaved changes

    let state = state.lock().unwrap();
    match state.handle_new_document(false) {
        Ok(_) => {}
        Err(NodeError::FileNotSaved) => {
            let force = open_save_warning_dialog();
            if force {
                state.handle_new_document(true)?;
            } else {
                return Ok(()); // User cancelled the dialog
            }
        }
        Err(e) => {
            eprintln!("Error creating new document: {:?}", e);
            return Err(e);
        }
    }
    let evt = CoreEvent::document_load(String::new());
    app.emit("file-opened", evt).unwrap();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(move |app| {
            let handle = app.handle();

            // Create a new Node instance
            let node = match Node::new() {
                Ok(node) => node,
                Err(e) => {
                    eprintln!("Error creating Node instance: {:?}", e);
                    std::process::exit(1);
                }
            };
            let config = node.config.clone();
            handle.manage(Mutex::new(node));

            // Tauri-specific
            let app_submenu = SubmenuBuilder::new(handle, "Application")
                .about(Some(
                    AboutMetadataBuilder::new()
                        .name("Proseforge".into())
                        .version("0.1.0".into())
                        .authors(vec!["Luke Martin".into()].into())
                        .website("https://lukm.dev/".into())
                        .comments("A simple markdown editor".into())
                        .build(),
                ))
                .separator()
                .close_window()
                .quit()
                .build()?;

            let file_submenu = SubmenuBuilder::new(handle, "File")
                .item(
                    &MenuItemBuilder::with_id("NEW", "New")
                        .accelerator("CmdOrControl+N")
                        .build(handle)?,
                )
                .item(
                    &MenuItemBuilder::with_id("OPEN", "Open")
                        .accelerator("CmdOrControl+O")
                        .build(handle)?,
                )
                .item(
                    &MenuItemBuilder::with_id("SAVE", "Save")
                        .accelerator("CmdOrControl+S")
                        .build(handle)?,
                )
                .build()?;
            let edit_submenu = SubmenuBuilder::new(handle, "Edit")
                .cut()
                .copy()
                .paste()
                .separator()
                .undo()
                .redo()
                .separator()
                .select_all()
                .build()?;
            let menu = MenuBuilder::new(handle)
                .item(&app_submenu)
                .item(&file_submenu)
                .item(&edit_submenu)
                .build()?;
            app.set_menu(menu)?;

            // Events

            handle.on_menu_event(move |handle, event| {
                if event.id() == "NEW" {
                    block_on(handle_new_file(handle.clone(), handle.state())).unwrap();
                }
                if event.id() == "OPEN" {
                    block_on(handle_open_dialog(handle.clone(), handle.state())).unwrap();
                }
                if event.id() == "SAVE" {
                    block_on(handle_save(handle.state())).unwrap();
                }
            });

            // Create the main window
            let win_builder = WebviewWindowBuilder::new(handle, "main", WebviewUrl::default())
                .title("Proseforge")
                .inner_size(800.0, 600.0);

            // set transparent title bar only when building for macOS
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let window = win_builder.build().unwrap();

            // set background color only when building for macOS
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSColor, NSWindow};
                use cocoa::base::{id, nil};

                let ns_window = window.ns_window().unwrap() as id;
                unsafe {
                    let bg_colour = match config.theme {
                        Theme::Light => NSColor::colorWithRed_green_blue_alpha_(
                            nil, 0.9294, 0.9294, 0.9098, 1.0,
                        ),
                        Theme::Dark => NSColor::colorWithRed_green_blue_alpha_(
                            nil, 0.1529, 0.1451, 0.1529, 1.0,
                        ),
                        _ => NSColor::colorWithRed_green_blue_alpha_(nil, 1.0, 1.0, 1.0, 1.0),
                    };
                    ns_window.setBackgroundColor_(bg_colour);
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            handle_open_dialog,
            get_config,
            handle_update_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
