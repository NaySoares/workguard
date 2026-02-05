mod menu;
mod handlers;
mod tray_icon;

use tauri::{App, Manager, tray::TrayIconBuilder};
use tauri::async_runtime::Mutex;
use crate::core::timer::WorkTimer;
use crate::utils::format_status;

/// Configura o tray icon completo: menu, handlers e loop de atualização
pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Calcula status inicial
    let initial_text = tauri::async_runtime::block_on(async {
        let state: tauri::State<Mutex<WorkTimer>> = app.state();
        let timer = state.lock().await;
        format_status(&timer)
    });

    // Cria o menu
    let tray_menu = menu::build_menu(app.handle(), &initial_text)?;
    let status_item_for_menu = tray_menu.status_item.clone();
    let status_item_for_loop = tray_menu.status_item.clone();

    // Carrega o ícone
    let icon = tray_icon::load_icon();

    // Constrói o tray
    TrayIconBuilder::new()
        .menu(&tray_menu.menu)
        .icon(icon)
        .on_menu_event(move |app, event| {
            handlers::handle_menu_event(app, event.id.as_ref(), status_item_for_menu.clone());
        })
        .build(app)?;

    // Inicia o loop de atualização periódica
    handlers::start_status_updater(app.handle().clone(), status_item_for_loop);

    Ok(())
}
