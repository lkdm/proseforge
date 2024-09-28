use tauri::{
    menu::{AboutMetadataBuilder, Menu, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    AppHandle, Wry,
};

pub fn setup_menu(handle: &AppHandle) -> tauri::Result<Menu<Wry>> {
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

    Ok(menu)
}
