use tauri::Emitter;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};

pub fn create_menu(app: &tauri::App) -> Result<(), tauri::Error> {
    let handle = app.handle();
    let about_item = MenuItem::with_id(
        handle,
        "about_datasquirrel",
        "About DataSquirrel",
        true,
        None::<&str>,
    )?;

    let mac_app_submenu = Submenu::with_items(
        handle,
        "DataSquirrel",
        true,
        &[
            &about_item,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::services(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::hide(handle, None)?,
            &PredefinedMenuItem::hide_others(handle, None)?,
            &PredefinedMenuItem::show_all(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::quit(handle, None)?,
        ],
    )?;

    let edit_submenu = Submenu::with_items(
        handle,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(handle, None)?,
            &PredefinedMenuItem::redo(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::cut(handle, None)?,
            &PredefinedMenuItem::copy(handle, None)?,
            &PredefinedMenuItem::paste(handle, None)?,
            &PredefinedMenuItem::select_all(handle, None)?,
        ],
    )?;

    let menu = Menu::with_items(handle, &[&mac_app_submenu, &edit_submenu])?;
    app.set_menu(menu)?;

    app.on_menu_event(move |app_handle, event| {
        if event.id().as_ref() == "about_datasquirrel" {
            app_handle.emit("open-about-modal", ()).unwrap_or(());
        }
    });

    Ok(())
}
