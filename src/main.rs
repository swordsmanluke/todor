#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate colored; // not needed in Rust 2018

use crate::schedule_formatter::*;
use crate::scheduled_item::{ScheduledItem, Scheduler, load_scheduler_config};
use crate::schedule_colorer::color_item;
use std::error::Error;
use itertools::Itertools;
use std::cmp::min;
use std::thread;
use std::sync::mpsc::{channel, Sender};
use crate::commands::{UICommand, ScheduleCommand};
use std::io::{stdout, stdin, Read, Write};
use termion::raw::IntoRawMode;
use termion::cursor::Goto;
use termion::clear;
use simplelog::{CombinedLogger, WriteLogger, LevelFilter, Config};
use std::fs::File;
use log::info;
use crate::tasks::{MasterScheduler, UserInputTask, CommandExecutor};

mod google_calendar_client;
mod google_scheduler;
mod scheduled_item;
mod schedule_formatter;
mod schedule_colorer;
mod todoist_scheduler;
mod todoist_client;
mod commands;
mod tasks;

const MAX_WIDTH: usize = 48; // max size of my terminal window TODO: Take this from command line Args

fn main() -> Result<(), Box<dyn Error>> {
    init_logging();

    let mut stdout = stdout().into_raw_mode().unwrap();
    let (ui_tx, ui_rx) = channel();
    let (cmd_tx, cmd_rx) = channel();

    // Refresh tasks loop
    let ui_sched_tx = ui_tx.clone();
    thread::spawn(move || { MasterScheduler::new(ui_sched_tx, cmd_rx).run(); });

    // input loop
    let cmd_in = ui_tx.clone();
    thread::spawn(move || { UserInputTask::new(cmd_in).run(); });

    // master I/O loop
    let mut command_executor = CommandExecutor::new(cmd_tx);
    let mut user_input = String::new();
    loop {
        match ui_rx.recv() {
            Ok(cmd) => {
                info!("Processing command: {:?}", cmd);
                match cmd {
                    UICommand::UpdateUserInput(new_input) => { user_input = new_input; }
                    UICommand::Schedules(sched) => { display_schedule(sched, &mut stdout); }
                    UICommand::Execute(command) => { command_executor.execute_command(&command); }
                    UICommand::Exit => { break; } // time to quit!
                }
            }
            Err(_) => {}
        }

        // No matter what happened... make sure the prompt is visible and up to date.
        display_prompt(user_input.clone(), &mut stdout);
        stdout.flush();
    }

    Ok(())
}

fn display_prompt(user_input: String, stdout: &mut dyn std::io::Write) {
    let prompt = format!("{}{}:> {}",
                         Goto(1, 999),
                         clear::CurrentLine,
                         user_input);

    info!("prompt: {:?}", prompt);

    write!(stdout, "{}", prompt);
    stdout.flush();
}

fn display_schedule(items: Vec<ScheduledItem>, stdout: &mut dyn std::io::Write) -> () {
    // Clear the screen and go to the top line before we start
    stdout.write(b"\x1B[2J\x1B[1;1H");

    let mut items = items.clone();
    items.sort_by_key(|f| f.start_time);

    // max_width is determined by the widest description or MAX_WIDTH, whichever is smaller.
    let max_width = min(MAX_WIDTH, items.iter().map(|i| i.description.len()).max().unwrap_or(MAX_WIDTH));

    // TODO: Group everything from a past date into a general "OVERDUE" bucket.
    let grouped_by_date = items.into_iter().group_by(|item| item.start_time.date());

    for (date, items_for_date) in &grouped_by_date
    {
        let ds: String = date.to_string();
        // Print the date
        write!(stdout, "{}\n\r", ds.get(0..ds.len() - 6).unwrap());
        let item_vec: Vec<ScheduledItem> = items_for_date.collect::<Vec<ScheduledItem>>();
        write!(stdout, "--------{}---------\r\n", item_vec.len()); // divider

        // Print the date's schedule
        for item in item_vec {
            match format_item(&item, max_width) {
                Some(s) => { write!(stdout, "  {}\n\r", color_item(&item, &s)); },
                None => {}
            }
        }
        write!(stdout, "\n\r");
    }
}

fn init_logging()  {
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("log/todor.log").expect("Could not open log file!")),
        ]
    ).expect("Could not initiate logging!");
}
