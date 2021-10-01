use chrono::{DateTime, Local};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug,Clone)]
pub struct ScheduledItem {
    pub id: String,
    pub scheduler: String,
    pub item_type: ScheduleItemType,
    pub description: String,
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
    pub place: Option<String>
}

#[derive(Debug,Clone)]
pub enum ScheduleItemType {
    Todo,
    Calendar
}

impl ScheduledItem {
    pub fn new(id: String, scheduler: String, item_type: ScheduleItemType, description: String, start_time: DateTime<Local>, end_time: Option<DateTime<Local>>, place: Option<String>) -> ScheduledItem {
        ScheduledItem{ id, scheduler, item_type, description, start_time, end_time, place }
    }
}

pub trait Scheduler {
    fn id(&self) -> String;
    fn refresh(&mut self) -> Result<(), Box<dyn Error>>;
    fn schedule(&self) -> Vec<ScheduledItem>;
    fn add(&mut self, target: &String, due_date: Option<DateTime<Local>>) -> Result<bool, String>;
    fn remove(&mut self, task: &String) -> Result<bool, String>;
}

// Configuration loading
#[derive(Deserialize)]
pub struct ScheduleConfig {
    pub google_cal: Vec<GoogleConfig>,
    pub todoist: Vec<TodoistConfig>
}

#[derive(Deserialize)]
pub struct GoogleConfig {
    pub name: String,
    pub cal_name: String
}

#[derive(Deserialize)]
pub struct TodoistConfig {
    pub name: String,
    pub project: String
}

pub fn load_scheduler_config() -> Result<ScheduleConfig, Box<dyn Error>> {
    let mut conf_file = File::open("config/config.toml")?;
    let mut toml_tasks = String::new();
    conf_file.read_to_string(&mut toml_tasks)?;
    let config: ScheduleConfig = toml::from_str(&toml_tasks)?;
    Ok(config)
}