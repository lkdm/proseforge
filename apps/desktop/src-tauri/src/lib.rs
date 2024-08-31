use proseforge_core::features::project::models::document::CreateDocumentRequest;
use proseforge_core::features::project::services::CreateDocumentRequestDto;
use proseforge_core::features::project::services::GetDocumentRequestDto;
use proseforge_core::features::project::services::GetDocumentResponseDto;
use proseforge_core::features::project::services::ProjectService;
use proseforge_core::features::project::services::ServiceError;
use proseforge_core::features::project::services::StatefulService;
use proseforge_core::features::project::services::UpdateDocumentRequestDto;
use proseforge_core::{Node, NodeError};
use proseforge_sqlite::SqliteAdapter;
use std::sync::{Arc, Mutex};
use tauri::menu::{AboutMetadataBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::State;
use tauri::{async_runtime::block_on, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use tauri::{AppHandle, Emitter, Manager};
use tokio::task::block_in_place;

type AppState = Node<StatefulService<SqliteAdapter>>;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
async fn handle_new_document(
    data: CreateDocumentRequestDto,
    app: AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), NodeError> {
    let project_service = {
        let state = state.lock().unwrap();
        state.project_service.clone()
    };
    project_service.document_create(&data);
    Ok(())
}

#[tauri::command]
async fn handle_open_document(
    data: GetDocumentRequestDto,
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<GetDocumentResponseDto, ServiceError> {
    let project_service = state.project_service.clone();
    project_service.document_get(&data).await
}

#[tauri::command]
async fn handle_save_document(
    data: UpdateDocumentRequestDto,
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<(), ServiceError> {
    let project_service = state.project_service.clone();
    project_service.document_update(&data).await
}

// #[tauri::command]
// async fn handle_update_content(
//     content: String,
//     state: tauri::State<'_, Mutex<AppState>>,
// ) -> Result<(), NodeError> {
//     // Lock the state safely
//     let ds;
//     {
//         let state = state.lock().unwrap();
//         ds = state.document_ds.clone();
//     }

//     let req = UpdateDocumentRequest::new(content.into());
//     ds.update_content(&req)
//         .await
//         .map_err(|_| NodeError::RepositoryError)?;
//     Ok(())
// }

// #[tauri::command]
// async fn handle_open_dialog(
//     app: AppHandle,
//     state: tauri::State<'_, Mutex<AppState>>,
// ) -> Result<(), NodeError> {
//     let state = state.lock().unwrap();
//     if state.editor.lock().unwrap().has_unsaved_changes() {
//         if !request_ignore_unsaved_changes_dialog() {
//             return Ok(());
//         }
//     }
//     let path = match request_open_path_dialog(None) {
//         Some(path) => path,
//         None => return Ok(()),
//     };
//     let document = DocumentBuilder::new()
//         .with_path(path.clone())
//         .load()?
//         .commit(&state.editor)?;
//     let content = document.get_content();

//     let evt = CoreEvent::document_load(content.into());
//     app.emit("file-opened", evt).unwrap();

//     Ok(())
// }

// #[tauri::command]
// async fn handle_save(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), NodeError> {
//     let state = state.lock().unwrap();
//     let result = {
//         // Lock the editor and attempt to save
//         match state.handle_save() {
//             Ok(()) => Ok(()),
//             Err(NodeError::NoSavePath) => {
//                 // Open a save location dialog
//                 let path = match request_save_path_dialog(None) {
//                     Some(path) => path,
//                     None => return Ok(()),
//                 };

//                 // Re-acquire the lock on editor to update the save location and retry saving
//                 let mut editor = state.editor.lock().unwrap();
//                 editor.set_save_location(path.into());

//                 // Retry saving
//                 editor.save()
//             }
//             Err(e) => Err(e),
//         }
//     };
//     result
// }

// #[tauri::command]
// async fn get_config(state: tauri::State<'_, Mutex<TauriNode>>) -> Result<Config, NodeError> {
//     let state = state.lock().map_err(|_| NodeError::BlockingError)?;
//     let config = state.config.clone();
//     Ok(config.as_ref().clone())
// }

// #[tauri::command]
// async fn handle_new_file(
//     app: AppHandle,
//     state: tauri::State<'_, Mutex<TauriNode>>,
// ) -> Result<(), NodeError> {
//     // Check if the current document has unsaved changes
//     // TODO: build dialogue for unsaved changes

//     let state = state.lock().unwrap();
//     match state.handle_new_document(false) {
//         Ok(_) => {}
//         Err(NodeError::FileNotSaved) => {
//             let force = request_ignore_unsaved_changes_dialog();
//             if force {
//                 state.handle_new_document(true)?;
//             } else {
//                 return Ok(()); // User cancelled the dialog
//             }
//         }
//         Err(e) => {
//             eprintln!("Error creating new document: {:?}", e);
//             return Err(e);
//         }
//     }
//     let evt = CoreEvent::document_load(String::new());
//     app.emit("file-opened", evt).unwrap();
//     Ok(())
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    // Share the tokio runtime with tauri.
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(move |app| {
            let handle = app.handle();

            // We need a the app handle to determine the data directory now.
            // This means all the setup code has to be within `setup`, however it doesn't support async so we `block_on`.
            block_in_place(|| {
                block_on(async move {
                    let sqlite_adapter = SqliteAdapter::new("sqlite::memory:").await.unwrap();
                    let service = StatefulService::new(sqlite_adapter);
                    let node = Node::new(service.clone());
                    handle.manage(node.clone());
                })
            });

            // let config = Config::default();

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
                // if event.id() == "NEW" {
                //     block_on(handle_new_document(handle.clone(), handle.state())).unwrap();
                // }
                // if event.id() == "OPEN" {
                //     block_on(handle_open_document(handle.clone(), handle.state())).unwrap();
                // }
                if event.id() == "SAVE" {
                    todo!("Got to get content from memory here.")
                    // block_on(handle_save_document(handle.state())).unwrap();
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
                // unsafe {
                //     let bg_colour = match config.theme {
                //         Theme::Light => NSColor::colorWithRed_green_blue_alpha_(
                //             nil, 0.9294, 0.9294, 0.9098, 1.0,
                //         ),
                //         Theme::Dark => NSColor::colorWithRed_green_blue_alpha_(
                //             nil, 0.1529, 0.1451, 0.1529, 1.0,
                //         ),
                //         _ => NSColor::colorWithRed_green_blue_alpha_(nil, 1.0, 1.0, 1.0, 1.0),
                //     };
                //     ns_window.setBackgroundColor_(bg_colour);
                // }
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            handle_new_document,
            handle_open_document,
            handle_save_document,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
