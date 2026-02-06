use tauri::{AppHandle, Manager, menu::MenuItem};
use tauri::async_runtime::Mutex;
use crate::core::timer::WorkTimer;
use crate::utils::{format_work_state, format_status};
use crate::core::state::WorkState;
use crate::notifications::{notify_hard_pause, notify_soft_pause, notify_finished, notify_back_to_work};
use crate::storage::{save_timer_state, state_file_path};

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
            "quit" => {
                // Salva o estado antes de sair
                save_current_state(&app_handle).await;
                std::process::exit(0);
            }
            "reset" => {
                let state: tauri::State<Mutex<WorkTimer>> = app_handle.state();
                let mut timer = state.lock().await;
                timer.reset_day();

                let state_text = format_work_state(&timer);
                if let Ok(path) = state_file_path(&app_handle) {
                    let _ = save_timer_state(&path, &timer.to_state());
                }

                drop(timer);
                let _ = status_item.set_text(&state_text);
            }
            _ => {}
        }
    });
}

/// Inicia o loop de atualização periódica do status
pub fn start_status_updater(app_handle: AppHandle, status_item: MenuItem<tauri::Wry>) {
    use std::time::Duration;
    use std::sync::Arc;
    use tokio::sync::Mutex as TokioMutex;

    // Armazena o último estado para detectar mudanças
    let last_state: Arc<TokioMutex<Option<WorkState>>> = Arc::new(TokioMutex::new(None));

    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(30)).await;

            // Acesso ao estado global timer para calcular o status atualizado
            let state: tauri::State<Mutex<WorkTimer>> = app_handle.state();
            let timer = state.lock().await;

            let current_state = timer.get_state();
            let status_text = format!(
              "{} | {}",
              format_status(&timer),          // resumo (trab/min rest/min)
              format_work_state(&timer)       // estado (soft/hard/finalizado)
            );

            // Salva o estado atual no arquivo JSON
            if let Ok(path) = state_file_path(&app_handle) {
                if let Err(e) = save_timer_state(&path, &timer.to_state()) {
                    eprintln!("[WorkGuard] Erro ao salvar estado: {:?}", e);
                }
            }

            drop(timer);

            // Verifica se o estado mudou e dispara notificação
            let mut last = last_state.lock().await;
            let state_changed = last.as_ref() != Some(&current_state);
            if state_changed {
                notify_by_state(&app_handle, &current_state);
                *last = Some(current_state);
            }

            // Atualiza o item de menu "status"
            let _ = status_item.set_text(&status_text);
        }
    });
}

/// Salva o estado atual do timer
async fn save_current_state(app: &AppHandle) {
    let state: tauri::State<Mutex<WorkTimer>> = app.state();
    let timer = state.lock().await;

    if let Ok(path) = state_file_path(app) {
        if let Err(e) = save_timer_state(&path, &timer.to_state()) {
            eprintln!("[WorkGuard] Erro ao salvar estado ao sair: {:?}", e);
        } else {
            println!("[WorkGuard] Estado salvo com sucesso");
        }
    }
}

fn notify_by_state(app: &AppHandle, state: &WorkState) {
    match state {
        WorkState::Paused { reason, until } => {
            let time_str = until
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or_else(|| "?".to_string());
            notify_hard_pause(app, reason, &time_str);
        }
        WorkState::SoftBreak { label, until } => {
            let time_str = until
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or_else(|| "?".to_string());
            notify_soft_pause(app, label, &time_str);
        }
        WorkState::Finished => {
            notify_finished(app);
        }
        WorkState::Working => {
            notify_back_to_work(app);
        }
    }
}