#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate colored; // not needed in Rust 2018

// use crate::google_scheduler::*;
use crate::schedule_formatter::*;
use crate::todoist_scheduler::*;
use crate::todoist_client::*;
use std::fs::File;
use crate::schedule_trait::ScheduledItem;
use crate::schedule_colorer::color_item;

// mod google_scheduler;
mod schedule_trait;
mod schedule_formatter;
mod schedule_colorer;
mod todoist_scheduler;
mod todoist_client;

fn main() {
    // Convert them to standard format
    // Colorize emergency out
    let file = File::open("config/todoist.json").unwrap();
    let todoist_token: ApiToken = serde_json::from_reader(file).expect("Badly formatted auth token file!");
    let tdc = TodoistRestClient::new(todoist_token.token);
    let mut tds = TodoistScheduler::new(tdc);

    for sched in tds.get_schedule() {
        for item in sched.iter().collect::<Vec<&ScheduledItem>>() {
            match format_item(item, 20) {
                Some(s) => println!("{}", color_item(item, &s)),
                None => {}
            }
        }
    }

}
