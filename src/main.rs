#[macro_use]
extern crate serde_derive;

use crate::schedule_trait::*;
use crate::google_scheduler::*;
use crate::todoist_scheduler::*;
use crate::todoist_client::*;
use std::fs::File;

mod schedule_trait;
mod google_scheduler;
mod todoist_scheduler;
mod todoist_client;

fn main() {
    // Get calendar from gcal
    // Get json from todoist
    // Convert them to standard format
    // Write to stdout
    // Colorize emergency out
    let file = File::open("config/todoist.json").unwrap();
    let todoist_token: ApiToken = serde_json::from_reader(file).expect("Badly formatted auth token file!");
    let mut tdc = TodoistClient::new(todoist_token.token);
    println!("{:?}", tdc.projects());
    println!("{:?}", tdc.tasks("Inbox"));

}
