// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod commands;
mod utils;
mod config;
mod notifications;
mod tray;
mod storage;

use tauri::async_runtime::Mutex;
use tauri::Manager;
use commands::{get_status, reset_day};
use config::load_or_create_timer;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let timer = load_or_create_timer(app.handle());
            app.manage(Mutex::new(timer));
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
