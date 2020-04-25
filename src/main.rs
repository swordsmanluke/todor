#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate colored; // not needed in Rust 2018

use crate::google_scheduler::*;
use crate::schedule_formatter::*;
use crate::todoist_scheduler::*;
use crate::scheduled_item::{ScheduledItem, Scheduler, load_scheduler_config, ScheduleConfig};
use crate::schedule_colorer::color_item;
use std::error::Error;
use itertools::Itertools;
use std::cmp::min;

mod google_calendar_client;
mod google_scheduler;
mod scheduled_item;
mod schedule_formatter;
mod schedule_colorer;
mod todoist_scheduler;
mod todoist_client;

const MAX_WIDTH: usize = 48; // max size of my terminal window TODO: Take this from command line Args

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = load_scheduler_config()?;
    let schedulers = load_schedulers(cfg)?;

    let mut items: Vec<ScheduledItem> = schedulers.iter().flat_map(|s| s.get_schedule().unwrap()).collect();
    items.sort_by_key(|f| f.start_time);

    // max_width is determined by the widest description or MAX_WIDTH, whichever is smaller.
    let max_width = min(MAX_WIDTH, items.iter().map(|i| i.description.len()).max().unwrap());

    // TODO: Group everything from a past date into a general "OVERDUE" bucket.
    let grouped_by_date = items.into_iter().group_by(|item| item.start_time.date());

    for (date, items_for_date) in &grouped_by_date
    {
        let ds: String = date.to_string();
        // Print the date
        println!("{}", ds.get(0..ds.len() - 6).unwrap());
        let item_vec: Vec<ScheduledItem> = items_for_date.collect::<Vec<ScheduledItem>>();
        println!("--------{}---------", item_vec.len()); // divider

        // Print the date's schedule
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

fn load_schedulers(cfg: ScheduleConfig) -> Result<Vec<Box<dyn Scheduler>>, Box<dyn Error>> {
    let mut schedulers: Vec<Box<dyn Scheduler>> = Vec::new();
    for gc in cfg.google_cal {
        let auth_file = format!("config/{}.json", gc.name);
        schedulers.push(Box::new(create_gcal_scheduler(auth_file, gc.cal_name)?));
    }
    for td in cfg.todoist {
        schedulers.push(Box::new(create_todoist_scheduler(td.name, td.project)?));
    }
    Ok(schedulers)
}
