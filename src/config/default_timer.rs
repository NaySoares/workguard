use chrono::{NaiveTime, Local};
use tauri::AppHandle;
use crate::core::timer::WorkTimer;
use crate::core::schedule::{Schedule, TimeBlock, BlockType};
use crate::storage::{load_timer_state, state_file_path};

// Carrega o timer salvo ou cria um novo com valores padrão
pub fn load_or_create_timer(app: &AppHandle) -> WorkTimer {
    let state_path = match state_file_path(app) {
        Ok(path) => path,
        Err(_) => {
            return create_default_timer();
        }
    };

    if let Ok(saved_state) = load_timer_state(&state_path) {
        let today = Local::now().date_naive();

        if today == saved_state.start_date {
            println!("[WorkGuard] Estado restaurado do arquivo");
            return WorkTimer::from_state(saved_state, schedule_default());
        } else {
            println!("[WorkGuard] Novo dia detectado, reiniciando timer");
        }
    } else {
        println!("[WorkGuard] Nenhum estado salvo encontrado, criando novo timer");
    }

    create_default_timer()
}

/// Cria um timer com valores padrão
fn create_default_timer() -> WorkTimer {
    WorkTimer::new(
        Local::now().time(), // hora atual como início
        schedule_default(),
        8 * 60, // meta em minutos (8 horas)
    )
}

fn schedule_default() -> Schedule {
    Schedule {
        blocks: vec![
            TimeBlock {
                label: "Almoço".to_string(),
                start: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                end: NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
                block_type: BlockType::HardPause,
            },
            TimeBlock {
                label: "Pausa para café".to_string(),
                start: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
                end: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                block_type: BlockType::SoftPause,
            },
        ]
    }
}