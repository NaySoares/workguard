use crate::core::state::WorkTimerState;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

const STATE_FILE: &str = "workguard_state.json";

pub fn state_file_path(app: &AppHandle) -> Result<PathBuf, tauri::Error> {
    let dir = app.path().app_data_dir()?;
    fs::create_dir_all(&dir).ok();
    Ok(dir.join(STATE_FILE))
}

pub fn save_timer_state(
    path: &Path,
    state: &WorkTimerState,
) -> Result<(), Box<dyn std::error::Error>> {

    let json = serde_json::to_string_pretty(state)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_timer_state(path: &Path) -> Result<WorkTimerState, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let state: WorkTimerState = serde_json::from_str(&data)?;
    Ok(state)
}