use std::time::SystemTime;
use chrono::{DateTime, Local};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ScheduledItem {
    pub description: String,
    pub time: DateTime<Local>,
    pub place: String
}
