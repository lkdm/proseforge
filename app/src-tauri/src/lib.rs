use std::path::PathBuf;

use file::{new_prosefile, open_prosefile};
use serde_json::json;
use tauri::{Manager, Wry};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::{with_store, StoreCollection};
pub mod file;
pub mod menu;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(move |app| {
            let handle = app.handle();
            // let menu = setup_menu(handle);

            // app.set_menu(menu)?;

            handle.on_menu_event(move |handle, event| {
                if event.id() == "NEW" {
                    // TODO: if unsaved changes– ask user– are you sure?
                    new_prosefile(&handle, |file_path| {
                        dbg!(file_path);
                    })
                }
                if event.id() == "OPEN" {
                    open_prosefile(handle, |file_path| {
                        dbg!(file_path);
                    })
                }
                if event.id() == "SAVE" {
                    // Saves the current focused document
                    unimplemented!("Save transactions");
                }
            });

            let stores = handle
                .try_state::<StoreCollection<Wry>>()
                .ok_or("Store not found")?;
            let path = PathBuf::from("store.bin");

            // Dependency injection to manage this.
            // 1. Define the interface for Config in the Config crate, and make it available in the core.
            #[warn(unused_must_use)]
            with_store(app.handle().clone(), stores, path, |store| {
                // Note that values must be serde_json::Value instances,
                // otherwise, they will not be compatible with the JavaScript bindings.
                store.insert("some-key".to_string(), json!({ "value": 5 }))?;

                // Get a value from the store.
                let value = store
                    .get("some-key")
                    .expect("Failed to get value from store");
                println!("{}", value); // {"value":5}

                // You can manually save the store after making changes.
                // Otherwise, it will save upon graceful exit as described above.
                store.save()?;

                Ok(())
            });

            Ok(())
        })
        .menu(menu::setup_menu)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
