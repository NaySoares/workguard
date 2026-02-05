// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;

use core::timer::WorkTimer;
use core::schedule::{Schedule, TimeBlock, BlockType};
use core::calculator::WorkCalculator;
use core::state::WorkState;
use chrono::{NaiveTime, Local};
use std::sync::Mutex;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder},
    Manager,
};

#[tauri::command]
fn get_status(timer: tauri::State<Mutex<WorkTimer>>) -> String{
    let timer = timer.lock().unwrap();

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

    format!(
        "Trabalhados: {} minutos | Restante: {} minutos | {}",
        worked_minutes, remaining_minutes, status_str
    )
}
fn main() {

    let schedule = Schedule {
        blocks: vec![
            TimeBlock {
                label: "Almoço".to_string(),
                start: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                end: NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
                block_type: BlockType::HardPause,
            },
        ],
    };

    let timer = WorkTimer::new(
        NaiveTime::from_hms_opt(8, 0, 0).unwrap(), // hora de início
        schedule,
        8 * 60, // meta em minutos
    );


    tauri::Builder::default()
        .manage(Mutex::new(timer))
        .setup(move |app| {

            // MENU ITEMS
            let status_item = MenuItem::with_id(
                app,
                "status",
                "Iniciando...",
                false,            // desabilitado (é só display)
                None::<&str>,     // sem atalho
            )?;

            let quit_item = MenuItem::with_id(
                app,
                "quit",
                "Sair",
                true,
                None::<&str>,
            )?;

            // MENU
            let menu = Menu::new(app)?;
            menu.append(&status_item)?;
            menu.append(&quit_item)?;

            // TRAY
            TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "quit" => std::process::exit(0),

                        "status" => {
                            let state: tauri::State<Mutex<WorkTimer>> = app.state();
                            let timer = state.lock().unwrap();
                            let status = timer.get_state();

                            let text = format!("{:?}", status);

                            if let Some(menu) = app.menu() {
                                if let Some(kind) = menu.get("status") {
                                    match kind {
                                        tauri::menu::MenuItemKind::MenuItem(item) => { item.set_text(text).ok(); },
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            use tauri::async_runtime::spawn;
            use std::time::Duration;

            let app_handle = app.handle().clone();

            spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(30)).await;

                    // Acesso ao estado global timer
                    let state: tauri::State<Mutex<WorkTimer>> = app_handle.state();
                    let timer = state.lock().unwrap();

                    let status = timer.get_state();
                    let text = format!("{:?}", status);

                    // Atualiza o item de menu "status"
                    if let Some(menu) = app_handle.menu() {
                        if let Some(kind) = menu.get("status") {
                            match kind {
                                tauri::menu::MenuItemKind::MenuItem(item) => { let _ = item.set_text(text); },
                                _ => {}
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
