use chrono::{NaiveTime, NaiveDate};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTimerState {
    pub start_date: NaiveDate,
    pub start_time: NaiveTime,
    pub daily_target_minutes: i64,
}

mod time_format {
    use chrono::NaiveTime;
    use serde::{self, Serializer, Deserialize};
    use serde::de::{Error, Deserializer};

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

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => NaiveTime::parse_from_str(&s, FORMAT)
                .map(Some)
                .map_err(D::Error::custom),
            None => Ok(None),
        }
    }
}