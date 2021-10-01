use crate::scheduled_item::{ScheduledItem, Scheduler, ScheduleItemType};
use crate::todoist_client::*;
use chrono::{DateTime, Local, TimeZone, Date};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use restson::Error::HttpError;
use log::info;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ApiToken {
    pub token: String
}

pub struct TodoistScheduler {
    client: Box<dyn TodoistClient>,
    project: String,
    cache: Vec<ScheduledItem>
}

pub(crate) fn create_todoist_scheduler(name: String, project: String) -> Result<TodoistScheduler, Box<dyn Error>> {
    let file = File::open(format!("config/{}.json", name))?;
    let todoist_token: ApiToken = serde_json::from_reader(file).expect("Badly formatted auth token file!");
    let tdc = TodoistRestClient::new(todoist_token.token);
    Ok(TodoistScheduler::new(Box::new(tdc), project))
}

impl TodoistScheduler {
    pub fn new(client: Box<dyn TodoistClient>, project: String) -> Self {
        TodoistScheduler { client, project, cache: Vec::new() }
    }
}

impl Scheduler for TodoistScheduler{
    fn id(&self) -> String {
        format!("todoist:{}", self.project)
    }

    fn refresh(&mut self) -> Result<(), Box<dyn Error>> {
        self.cache = self.client.tasks(self.project.as_str())?.iter().
            map(|t| task_to_scheduled_item(&self.project, t)).collect();

        Ok(())
    }

    fn schedule(&self) -> Vec<ScheduledItem> {
        self.cache.clone()
    }

    fn add(&mut self, target: &String, due_date: Option<DateTime<Local>>) -> Result<bool, String> {
        let mut commands = target.split(" ");
        let target = commands.next().unwrap_or("");
        let handled = match target {
            "todo" => {
                info!("Adding to Todoist project '{}'", self.project);
                let description = commands.map(|s| s.to_string()).collect::<Vec<String>>().join(" ");

                match self.client.add(self.project.as_str(), description, due_date) {
                    Ok(result) => result,
                    Err(e) => return Err(e.to_string())
                }
            },
            _ => false
        };

        Ok(handled)
    }

    fn remove(&mut self, target: &String) -> Result<bool, String> {
        info!("Looking for a task '{}' in project {}", target, self.project);
        let tasks = self.client.tasks(self.project.as_str());
        let res = match tasks {
            Ok(tasks) => {
                let task = tasks.iter().find(|t| t.clone().content == *target);
                match task{
                    Some(t) => {
                        info!("Found '{}'! Attempting to close it!", t.content);
                        match self.client.close(t.id) {
                            Ok(result) => {
                                info!("Closed {}: {}", t.content, result);
                                result
                            },
                            Err(e) => {
                                match e {
                                    HttpError(_code, msg) => { return Err(msg); },
                                    _ => return Err(e.to_string())
                                }
                            }
                        }
                    },
                    None => {
                        info!("Could not find task {:?} in {:?}", task, tasks);
                        false
                    }
                }
            },
            Err(e) => return Err(e.to_string())
        };

        Ok(res)
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

fn task_to_scheduled_item(account_id: &String, t: &Task) -> ScheduledItem {
    let id = format!("todoist:{}", t.id);
    ScheduledItem::new(
        id,
        format!("todoist:{}", account_id),
        ScheduleItemType::Todo,
        t.content.clone(),
        td_time_to_datetime(&t.due),
        None,
        None)
}

fn end_of_day(date: Date<Local>) -> DateTime<Local> {
    date.and_hms(23, 59, 59)
}



