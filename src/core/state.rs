use chrono::NaiveTime;

#[derive(Debug, Clone, PartialEq)]
pub enum WorkState {
    Working,
    Paused {
        reason: String,
        until: Option<NaiveTime>,
    },
    SoftBreak {
        label: String,
        until: Option<NaiveTime>,
    },
    Finished,
}