// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;

use core::timer::WorkTimer;
use core::schedule::{Schedule, TimeBlock, BlockType};
use core::calculator::WorkCalculator;
use core::state::WorkState;
use chrono::{NaiveTime, Local};
use tauri::async_runtime::Mutex;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    image::Image, Manager,
};

/// Formata o status atual de trabalho em uma string legível
fn format_status(timer: &WorkTimer) -> String {
    let now = Local::now().time();

    let (worked_minutes, remaining_minutes) =
        WorkCalculator::calculate_worked_and_remaining(
            timer.start_time,
            now,
            &timer.schedule,
            timer.daily_target_minutes
        );

    let worked_h = worked_minutes / 60;
    let worked_m = worked_minutes % 60;
    let remaining_h = remaining_minutes / 60;
    let remaining_m = remaining_minutes % 60;

    format!(
        "{}h {}m trabalhadas | {}h {}m restantes",
        worked_h, worked_m, remaining_h, remaining_m
    )
}

#[tauri::command]
async fn get_status(timer: tauri::State<'_, Mutex<WorkTimer>>) -> Result<String, String> {
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

#[tauri::command]
async fn reset_day(timer: tauri::State<'_, Mutex<WorkTimer>>) -> Result<(), String> {
    let mut timer = timer.lock().await;
    timer.reset_day();
    Ok(())
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


            // Calcula status inicial
            let initial_text = tauri::async_runtime::block_on(async {
                let state: tauri::State<Mutex<WorkTimer>> = app.state();
                let timer = state.lock().await;
                format_status(&timer)
            });

            // MENU ITEMS
            let status_item = MenuItem::with_id(
                app,
                "status",
                &initial_text,
                false,            // desabilitado (é só display)
                None::<&str>,     // sem atalho
            )?;
            let status_item_for_menu = status_item.clone();
            let status_item_for_loop = status_item.clone();

            let quit_item = MenuItem::with_id(
                app,
                "quit",
                "Sair",
                true,
                None::<&str>,
            )?;

            let reset_item = MenuItem::with_id(
                app,
                "reset",
                "Reiniciar Dia",
                true,
                None::<&str>,
            )?;

            // MENU
            let menu = Menu::new(app)?;
            menu.append(&status_item)?;
            menu.append(&reset_item)?;
            menu.append(&quit_item)?;

            // TRAY:
            let (icon_rgba, icon_width, icon_height) = {
                let icon_bytes = include_bytes!("../icons/timer.png");
                let img = image::load_from_memory(icon_bytes)
                    .expect("Failed to load icon")
                    .into_rgba8();
                let (w, h) = img.dimensions();
                (img.into_raw(), w, h)
            };

            let icon = Image::new_owned(icon_rgba, icon_width, icon_height);
            TrayIconBuilder::new()
                .menu(&menu)
                .icon(icon)
                .on_menu_event(move |app, event| {
                    let app_handle = app.clone();
                    let status_item = status_item_for_menu.clone();
                    tauri::async_runtime::spawn(async move {
                        match event.id.as_ref() {
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
                })
                .build(app)?;

            use tauri::async_runtime::spawn;
            use std::time::Duration;

            let app_handle = app.handle().clone();

            // Spawn loop para atualização periódica
            spawn(async move {
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
                    let _ = status_item_for_loop.set_text(&text);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_status, reset_day])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
