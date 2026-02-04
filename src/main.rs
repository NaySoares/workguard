// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;

use core::timer::WorkTimer;
use core::schedule::{Schedule, TimeBlock, BlockType};
use core::calculator::WorkCalculator;
use core::state::WorkState;
use chrono::{NaiveTime, Local};
use std::sync::Mutex;

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
        .invoke_handler(tauri::generate_handler![get_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
