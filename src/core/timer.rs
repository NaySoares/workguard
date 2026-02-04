use chrono::{Local, NaiveTime};

use super::schedule::{Schedule, BlockType};
use super::calculator::WorkCalculator;
use super::state::WorkState;

pub struct WorkTimer {
    pub start_time: NaiveTime,
    pub schedule: Schedule,
    pub daily_target_minutes: i64,
}

impl WorkTimer {
    pub fn new(start_time: NaiveTime, schedule: Schedule, daily_target_minutes: i64) -> Self {
        Self {
            start_time,
            schedule,
            daily_target_minutes,
        }
    }

    pub fn get_state(&self) -> WorkState {
        let now = Local::now().time();

        let worked_minutes = WorkCalculator::calculate_worked_minutes(
            self.start_time,
            now,
            &self.schedule,
        );

        self.determine_state(now, worked_minutes)
    }

    pub fn determine_state(&self, now: NaiveTime, worked_minutes: i64) -> WorkState {
        if worked_minutes >= self.daily_target_minutes {
            return WorkState::Finished;
        }

        if let Some(block) = self.schedule.get_active_block(now) {
            match block.block_type {
                BlockType::HardPause => {
                    return WorkState::Paused {
                        reason: block.label.clone(),
                        until: Some(block.end),
                    }
                }

                BlockType::SoftPause => {
                    return WorkState::SoftBreak {
                        label: block.label.clone(),
                        until: Some(block.end),
                    }
                }
            }
        }

        WorkState::Working
    }
}