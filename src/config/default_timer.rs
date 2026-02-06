use chrono::{NaiveTime};
use crate::core::timer::WorkTimer;
use crate::core::schedule::{Schedule, TimeBlock, BlockType};

pub fn default_timer() -> WorkTimer {
    WorkTimer::new(
        NaiveTime::from_hms_opt(8, 0, 0).unwrap(), // hora de início
        schedule_default(),
        8 * 60, // meta em minutos
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
                start: NaiveTime::from_hms_opt(16, 00, 0).unwrap(),
                end: NaiveTime::from_hms_opt(17, 00, 0).unwrap(),
                block_type: BlockType::SoftPause,
            },
            TimeBlock {
                label: "Pausa Hard".to_string(),
                start: NaiveTime::from_hms_opt(20, 45, 0).unwrap(),
                end: NaiveTime::from_hms_opt(20, 50, 0).unwrap(),
                block_type: BlockType::HardPause,
            },
        ]
    }
}