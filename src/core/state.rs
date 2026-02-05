use chrono::NaiveTime;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum WorkState {
    Working,
    Paused {
        reason: String,

        #[serde(with = "time_format")]
        until: Option<NaiveTime>,
    },
    SoftBreak {
        label: String,
        #[serde(with = "time_format")]
        until: Option<NaiveTime>,
    },
    Finished,
}

mod time_format {
    use chrono::NaiveTime;
    use serde::{self, Serializer};

    const FORMAT: &str = "%H:%M:%S";

    pub fn serialize<S>(
        time: &Option<NaiveTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match time {
            Some(t) => serializer.serialize_str(&t.format(FORMAT).to_string()),
            None => serializer.serialize_none(),
        }
    }
}