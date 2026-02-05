use tauri::{menu::{Menu, MenuItem}, AppHandle};

pub struct TrayMenu {
    pub menu: Menu<tauri::Wry>,
    pub status_item: MenuItem<tauri::Wry>,
}

/// Cria o menu do tray com os itens: status, reset e quit
pub fn build_menu(app: &AppHandle, initial_status: &str) -> Result<TrayMenu, Box<dyn std::error::Error>> {
    let status_item = MenuItem::with_id(
        app,
        "status",
        initial_status,
        false,            // desabilitado (é só display)
        None::<&str>,     // sem atalho
    )?;

    let reset_item = MenuItem::with_id(
        app,
        "reset",
        "Reiniciar Dia",
        true,
        None::<&str>,
    )?;

    let quit_item = MenuItem::with_id(
        app,
        "quit",
        "Sair",
        true,
        None::<&str>,
    )?;

    let menu = Menu::new(app)?;
    menu.append(&status_item)?;
    menu.append(&reset_item)?;
    menu.append(&quit_item)?;

    Ok(TrayMenu { menu, status_item })
}
