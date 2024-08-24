use md_core::config::{Config, Theme};
use md_core::error::CoreError;
use md_core::md::DataStorage;
use md_core::md::MarkdownFile;
use md_core::md::*;
use md_core::Node;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::menu::{
    AboutMetadata, AboutMetadataBuilder, CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder,
    PredefinedMenuItem, SubmenuBuilder,
};
use tauri::{async_runtime::block_on, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use tauri::{AppHandle, Emitter, Manager};
use tokio::task::block_in_place;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn load(state: tauri::State<'_, Mutex<Node>>) -> Result<String, CoreError> {
    let state = state.lock().unwrap();
    let editor = state.editor.clone();
    Ok(editor.content())
}

#[tauri::command]
async fn save(content: &str, state: tauri::State<'_, Mutex<Node>>) -> Result<(), CoreError> {
    let mut state = state.lock().unwrap();
    // Use Arc::get_mut to get a mutable reference to MarkdownFile
    // Note: This only works if there's exactly one Arc pointer to the data
    let editor = match Arc::get_mut(&mut state.editor) {
        Some(editor) => editor,
        None => return Err(CoreError::multiple_arc_references()),
    };
    editor.set_content(content);
    editor.write()?;
    Ok(())
}

#[tauri::command]
async fn open_file_dialogue(
    app: AppHandle,
    state: tauri::State<'_, Mutex<Node>>,
) -> Result<(), CoreError> {
    app.emit("file-opening", 1).unwrap();
    let path = open_file_dialog()?;
    let mut md: MarkdownFile = path.into();
    let mut state = state.lock().unwrap();
    md.read()?;
    state.editor = Arc::new(md);
    app.emit("file-opened", 1).unwrap();
    Ok(())
}

#[tauri::command]
async fn get_config(state: tauri::State<'_, Mutex<Node>>) -> Result<Config, CoreError> {
    let state = state.lock().map_err(|_| CoreError::blocking_error())?;
    let config = state.config.clone();
    Ok(config.as_ref().clone())
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
                        .name("Markdown Editor".into())
                        .version("0.1.0".into())
                        .authors(vec!["Luke Martin".into()].into())
                        .website("https://lukm.dev/".into())
                        .comments("A simple markdown editor".into())
                        .build(),
                ))
                .separator()
                .build()?;

            let file_submenu = SubmenuBuilder::new(handle, "File")
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
                if event.id() == "OPEN" {
                    block_on(open_file_dialogue(handle.clone(), handle.state())).unwrap();
                }
                if event.id() == "SAVE" {
                    handle.emit("file-save", 1).unwrap();
                }
            });

            // Create the main window
            let win_builder = WebviewWindowBuilder::new(handle, "main", WebviewUrl::default())
                .title("Markdown Editor")
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
        .invoke_handler(tauri::generate_handler![
            greet,
            load,
            save,
            open_file_dialogue,
            get_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
