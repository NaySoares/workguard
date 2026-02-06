// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod commands;
mod utils;
mod config;
mod notifications;
mod tray;

use tauri::async_runtime::Mutex;
use commands::{get_status, reset_day};
use config::default_timer;

fn main() {
    let timer = default_timer();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .manage(Mutex::new(timer))
        .setup(|app| {
            tray::setup(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            reset_day
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
