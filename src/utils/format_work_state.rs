use crate::core::timer::WorkTimer;
use crate::core::state::WorkState;

pub fn format_work_state(timer: &WorkTimer) -> String {
    let state = timer.get_state();
    match state {
        WorkState::Working => "Trabalhando".to_string(),
        WorkState::Paused { reason, .. } => format!("Pausado: {}", reason),
        WorkState::SoftBreak { label, .. } => format!("Pausa leve: {}", label),
        WorkState::Finished => "Concluído".to_string(),
    }
}