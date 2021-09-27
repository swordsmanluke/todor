use crate::scheduled_item::{ScheduledItem, Scheduler};
use crate::todoist_client::*;
use chrono::{DateTime, Local, TimeZone, Date};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use restson::Error::HttpError;


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
    fn refresh(&mut self) -> Result<(), Box<dyn Error>> {
        self.cache = self.client.tasks(self.project.as_str())?.iter().
            map(|t| task_to_scheduled_item(t)).collect();

        Ok(())
    }

    fn schedule(&self) -> Vec<ScheduledItem> {
        self.cache.clone()
    }

    fn add(&mut self, target: String) -> Result<bool, String> {
        let mut commands = target.split(" ");
        let target = commands.next().unwrap_or("");
        let handled = match target {
            "todo" => {
                    match commands.next() {
                        Some(project) => {
                            println!("Looking for a project named {}", project);
                            if project == self.project {
                                println!("Found it! Adding task: {}", target.clone());
                                match self.client.add(self.project.as_str(), commands.map(|s| s.to_string()).collect::<Vec<String>>().join(" ")) {
                                    Ok(result) => result,
                                    Err(e) => return Err(e.to_string())
                                }
                            } else {
                                println!("Could not find a project named {}", project);
                                false
                            }
                        },
                        None => false
                    }
            },
            _ => false
        };

        Ok(handled)
    }

    fn remove(&mut self, target: String) -> Result<bool, String> {
        let mut commands = target.split(" ");
        let target = commands.next().unwrap_or("");
        Ok(match target {
            "todo" => {
                let prefix = commands.map(|s| s.to_string()).collect::<Vec<String>>().join(" ");
                println!("Looking for a task starting with '{}' in project {}", prefix, self.project);
                let tasks = self.client.tasks(self.project.as_str());
                match tasks {
                    Ok(tasks) => {
                        let task = tasks.iter().find(|t| t.clone().content.starts_with(&prefix));
                        match task{
                            Some(t) => {
                                println!("Found '{}'! Attempting to close it!", t.content);
                                match self.client.close(t.id) {
                                    Ok(result) => {
                                        println!("Closed: {}", result);
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
                            None => false
                        }
                    },
                    Err(e) => return Err(e.to_string())
                }
            },
            _ => false
        })
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
    let id = format!("todoist:{}", t.id);
    ScheduledItem::new(
        id,
        t.content.clone(),
        td_time_to_datetime(&t.due),
        None,
        None)
}

fn end_of_day(date: Date<Local>) -> DateTime<Local> {
    date.and_hms(23, 59, 59)
}



