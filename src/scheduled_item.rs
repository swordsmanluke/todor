use chrono::{DateTime, Local};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ScheduledItem {
    pub description: String,
    pub time: DateTime<Local>,
    pub place: Option<String>
}

impl ScheduledItem {
    pub fn new(description: String, time: DateTime<Local>, place: Option<String>) -> ScheduledItem {
        ScheduledItem{ description, time, place }
    }
}
