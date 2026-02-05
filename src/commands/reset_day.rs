use crate::core::timer::WorkTimer;
use tauri::async_runtime::Mutex;

#[tauri::command]
pub async fn reset_day(timer: tauri::State<'_, Mutex<WorkTimer>>) -> Result<(), String> {
    let mut timer = timer.lock().await;
    timer.reset_day();
    Ok(())
}