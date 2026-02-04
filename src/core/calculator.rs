use chrono::{NaiveTime};
use super::schedule::{Schedule, BlockType};

pub struct WorkCalculator;

impl WorkCalculator {
  pub fn calculate_worked_minutes(
    start: NaiveTime,
    now: NaiveTime,
    schedule: &Schedule,
  ) -> i64 {
    // Tempo total trabalhado em minutos
    let total_duration = now - start;
    let mut worked_minutes = total_duration.num_minutes();

    // Subtrair pausas reais (HardPause)
    for block in &schedule.blocks {
      if block.block_type == BlockType::HardPause {
        let overlap = Self::calculate_overlap(start, now, block.start, block.end);
        worked_minutes -= overlap;
      }
    }

    worked_minutes.max(0) // Garantir que não seja negativo
  }

  /// Calcula minutos trabalhados e minutos restantes até a meta diária.
  /// Retorna (worked_minutes, remaining_minutes).
  pub fn calculate_worked_and_remaining(
    start: NaiveTime,
    now: NaiveTime,
    schedule: &Schedule,
    daily_target_minutes: i64,
  ) -> (i64, i64) {
    let worked = Self::calculate_worked_minutes(start, now, schedule);
    let remaining = if daily_target_minutes > worked {
      daily_target_minutes - worked
    } else {
      0
    };

    (worked, remaining)
  }

  fn calculate_overlap(
    start: NaiveTime,
    end: NaiveTime,
    block_start: NaiveTime,
    block_end: NaiveTime,
  ) -> i64 {
    let effective_start = start.max(block_start);
    let effective_end = end.min(block_end);

    if effective_end > effective_start {
      (effective_end - effective_start).num_minutes()
    } else {
      0
    }
  }
}