use tauri::{AppHandle, Manager, menu::MenuItem};
use tauri::async_runtime::Mutex;
use crate::core::timer::WorkTimer;
use crate::utils::format_status;

/// Handler para eventos do menu do tray
pub fn handle_menu_event(
    app: &AppHandle,
    event_id: &str,
    status_item: MenuItem<tauri::Wry>,
) {
    let app_handle = app.clone();
    let status_item = status_item.clone();
    let event_id = event_id.to_string(); // Clone para mover para o async

    tauri::async_runtime::spawn(async move {
        match event_id.as_str() {
            "quit" => std::process::exit(0),
            "reset" => {
                let state: tauri::State<Mutex<WorkTimer>> = app_handle.state();
                let mut timer = state.lock().await;
                timer.reset_day();
                // Calcula o texto enquanto ainda temos o lock
                let text = format_status(&timer);
                drop(timer); // Libera o lock explicitamente
                let _ = status_item.set_text(&text);
            }
            _ => {}
        }
    });
}

/// Inicia o loop de atualização periódica do status
pub fn start_status_updater(app_handle: AppHandle, status_item: MenuItem<tauri::Wry>) {
    use std::time::Duration;

    tauri::async_runtime::spawn(async move {
        loop {
            // Aguarda antes da próxima atualização
            tokio::time::sleep(Duration::from_secs(30)).await;

            // Acesso ao estado global timer para calcular o status atualizado
            let text = {
                let state: tauri::State<Mutex<WorkTimer>> = app_handle.state();
                let timer = state.lock().await;
                format_status(&timer)
            };

            // Atualiza o item de menu "status"
            let _ = status_item.set_text(&text);
        }
    });
}
