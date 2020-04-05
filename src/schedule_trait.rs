use std::time::SystemTime;

pub struct ScheduledItem {
    pub description: String,
    pub time: SystemTime,
    pub place: String
}
