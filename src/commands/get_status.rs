use crate::core::timer::WorkTimer;
use crate::core::calculator::WorkCalculator;
use crate::core::state::WorkState;
use chrono::Local;
use tauri::async_runtime::Mutex;

#[tauri::command]
pub async fn get_status(timer: tauri::State<'_, Mutex<WorkTimer>>) -> Result<String, String> {
    let timer = timer.lock().await;

    // estado atual (Working, Paused, SoftBreak, Finished)
    let state = timer.get_state();

    let now = Local::now().time();
    let (worked_minutes, remaining_minutes) = WorkCalculator::
        calculate_worked_and_remaining(
            timer.start_time,
            now,
            &timer.schedule,
            timer.daily_target_minutes,
        );

    let status_str = match state {
        WorkState::Working => "Trabalhando".to_string(),
        WorkState::Finished => "Concluído".to_string(),
        WorkState::Paused { reason, .. } => format!("Pausado: {}", reason),
        WorkState::SoftBreak { label, .. } => format!("Pausa leve: {}", label),
    };

    Ok(format!(
        "Trabalhados: {} minutos | Restante: {} minutos | {}",
        worked_minutes, remaining_minutes, status_str
    ))
}