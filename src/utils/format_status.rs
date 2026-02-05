use crate::core::timer::WorkTimer;
use crate::core::calculator::WorkCalculator;
use chrono::Local;

/// Formata o status atual de trabalho em uma string legível
pub fn format_status(timer: &WorkTimer) -> String {
    let now = Local::now().time();

    let (worked_minutes, remaining_minutes) =
        WorkCalculator::calculate_worked_and_remaining(
            timer.start_time,
            now,
            &timer.schedule,
            timer.daily_target_minutes,
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
