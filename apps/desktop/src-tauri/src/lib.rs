use md_core::config::{Config, Theme};
use md_core::error::CoreError;
use md_core::md::DataStorage;
use md_core::md::MarkdownFile;
use md_core::md::*;
use md_core::Node;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::{async_runtime::block_on, Manager, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use tokio::task::block_in_place;

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
async fn open_file_dialogue(state: tauri::State<'_, Mutex<Node>>) -> Result<(), CoreError> {
    let path = open_file_dialog()?;
    let mut md: MarkdownFile = path.into();
    let mut state = state.lock().unwrap();
    md.read()?;
    state.editor = Arc::new(md);
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
        .setup(|app| {
            // Create a new Node instance
            let node = match Node::new() {
                Ok(node) => node,
                Err(e) => {
                    eprintln!("Error creating Node instance: {:?}", e);
                    std::process::exit(1);
                }
            };
            let config = node.config.clone();
            app.manage(Mutex::new(node));

            // Tauri-specific

            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
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
