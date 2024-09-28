use std::path::PathBuf;

use serde_json::json;
use tauri::{
    menu::{AboutMetadataBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Manager, Wry,
};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::{with_store, StoreCollection};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(move |app| {
            let handle = app.handle();

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

            handle.on_menu_event(move |handle, event| {
                if event.id() == "NEW" {
                    // TODO: if unsaved changes– ask user– are you sure?
                    handle
                        .dialog()
                        .file()
                        .set_title("New Prosefile")
                        .set_can_create_directories(true)
                        .add_filter("Prosefiles", &["prose"])
                        .save_file(|file_path| {
                            // Clear application memory
                            // Create db file
                        })
                }
                if event.id() == "OPEN" {
                    handle
                        .dialog()
                        .file()
                        .set_title("Open Prosefile")
                        .add_filter("Prosefiles", &["prose"])
                        .pick_file(|file_path| {
                            // return a file_path `Option`, or `None` if the user closes the dialog
                            match file_path {
                                Some(path) => print!("{:?}", path),
                                _ => (),
                            }
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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
