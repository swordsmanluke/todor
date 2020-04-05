use crate::schedule_trait::ScheduledItem;
use crate::todoist_client::*;
use std::time::SystemTime;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Date};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ApiToken {
    pub token: String
}

pub struct TodoistScheduler<T> where T: TodoistClient {
    client: T
}

impl<T> TodoistScheduler<T> where T: TodoistClient{
    pub fn new(client: T) -> Self {
        TodoistScheduler { client }
    }

    pub fn get_schedule(&mut self) -> Vec<ScheduledItem> {
        self.client.tasks("Inbox").iter().map(|t| task_to_scheduled_item(t)).collect()
    }
}

fn td_time_to_datetime(due: &Option<TodoistDate>) -> DateTime<Local> {
    match due {
        None => end_of_day(Local::today()),
        Some(td) => end_of_day(Local.ymd(2014, 7, 8)) // TODO: Extract yyyy-mm-dd
    }
}

fn task_to_scheduled_item(t: &Task) -> ScheduledItem {
    ScheduledItem {
        description: t.content.clone(),
        time: td_time_to_datetime(&t.due),
        place: String::new(),
    }
}

fn end_of_day(date: Date<Local>) -> DateTime<Local> {
    date.and_hms(11, 59, 59)
}








