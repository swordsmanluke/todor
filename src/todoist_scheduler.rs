use crate::scheduled_item::ScheduledItem;
use crate::todoist_client::*;
use chrono::{DateTime, Local, TimeZone, Date};
use restson::Error;
use regex::Regex;


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

    pub fn get_schedule(&mut self) -> Result<Vec<ScheduledItem>, Error> {
        let scheduled_items = self.client.tasks("Inbox")?.iter().
            map(|t| task_to_scheduled_item(t)).collect();

        Ok(scheduled_items)
    }
}

fn td_time_to_datetime(due: &Option<TodoistDate>) -> DateTime<Local> {
    let date = match due {
        None => Local::today(),
        Some(_td) => extract_date(_td)
    };

    end_of_day(date)
}

fn extract_date(td: &TodoistDate) -> Date<Local> {
    lazy_static! { // <- Helper to make sure we only compile the regex once!
        // Date is in yyyy-mm-dd format
        static ref DATE_REGEX: Regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
    }

    match DATE_REGEX.captures(&td.date) {
        Some(cap) => Local.ymd(cap[1].parse::<i32>().unwrap(), cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap()),
        None => Local::today()
    }
}

fn task_to_scheduled_item(t: &Task) -> ScheduledItem {
    ScheduledItem::new(
        t.content.clone(),
        td_time_to_datetime(&t.due),
        None)
}

fn end_of_day(date: Date<Local>) -> DateTime<Local> {
    date.and_hms(11, 59, 59)
}








