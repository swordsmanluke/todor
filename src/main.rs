#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate colored; // not needed in Rust 2018

use crate::google_scheduler::*;
use crate::schedule_formatter::*;
use crate::todoist_scheduler::*;
use crate::todoist_client::*;
use std::fs::File;
use crate::scheduled_item::ScheduledItem;
use crate::schedule_colorer::color_item;
use crate::google_calendar_client::create_gcal_client;
use std::error::Error;
use itertools::Itertools;
use std::cmp::min;
use colored::*;

mod google_calendar_client;
mod google_scheduler;
mod scheduled_item;
mod schedule_formatter;
mod schedule_colorer;
mod todoist_scheduler;
mod todoist_client;

const MAX_WIDTH: usize = 48; // max size of my terminal window

fn main() -> Result<(), Box<dyn Error>> {
    let mut tds = create_todoist_scheduler();
    let mut work_cal = create_gcal_scheduler("config/work_cal.json", "lucas@flexe.com")?;
    let mut home_cal = create_gcal_scheduler("config/home_cal.json", "swordsmanluke@gmail.com")?;

    let mut items = work_cal.get_schedule()?;
    items.append(&mut home_cal.get_schedule()?);
    items.append(&mut tds.get_schedule()?);

    items.sort_by_key(|f| f.time);

    let max_width = min(MAX_WIDTH, items.iter().map(|i| i.description.len()).max().unwrap());
    let grouped_by_date = items.into_iter().group_by(|item| item.time.date());

    for (date, items_for_date) in &grouped_by_date
    {
        let ds: String = date.to_string();
        println!("{}", ds.get(0..ds.len() - 6).unwrap());
        let item_vec: Vec<ScheduledItem> = items_for_date.collect::<Vec<ScheduledItem>>();
        println!("--------{}---------", item_vec.len());
        for item in item_vec {
            match format_item(&item, max_width) {
                Some(s) => println!("  {}", color_item(&item, &s)),
                None => {}
            }
        }
        println!()
    }

    Ok(())
}

fn create_gcal_scheduler(auth_file: &'static str, cal_name: &str) -> Result<GoogleScheduler, Box<dyn Error>> {
    let gcc = create_gcal_client(auth_file)?;
    Ok(GoogleScheduler::new(cal_name, gcc))
}

fn create_todoist_scheduler() -> TodoistScheduler<TodoistRestClient> {
    let file = File::open("config/todoist.json").unwrap();
    let todoist_token: ApiToken = serde_json::from_reader(file).expect("Badly formatted auth token file!");
    let tdc = TodoistRestClient::new(todoist_token.token);
    TodoistScheduler::new(tdc)
}
