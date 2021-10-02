#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate colored; // not needed in Rust 2018
extern crate event_parser;

use std::error::Error;
use std::thread;
use std::sync::mpsc::{channel};
use crate::commands::UICommand;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use simplelog::{CombinedLogger, WriteLogger, LevelFilter, Config};
use std::fs::File;
use log::info;
use crate::tasks::{MasterScheduler, UserInputTask, CommandExecutor};
use crate::display::{Window, ScheduleWindow, PromptWindow};

mod google_calendar_client;
mod google_scheduler;
mod scheduled_item;
mod schedule_formatter;
mod schedule_colorer;
mod todoist_scheduler;
mod todoist_client;
mod commands;
mod tasks;
mod display;

const MAX_WIDTH: usize = 48; // max size of my terminal window TODO: Take this from command line Args

fn main() -> Result<(), Box<dyn Error>> {
    init_logging();

    let mut windows: Vec<Box<dyn Window>> = vec![];

    let mut stdout = stdout().into_raw_mode().unwrap();
    let (ui_tx, ui_rx) = channel();
    let (cmd_tx, cmd_rx) = channel();

    // Refresh tasks loop
    let ui_sched_tx = ui_tx.clone();
    thread::spawn(move || { MasterScheduler::new(ui_sched_tx, cmd_rx).run().unwrap(); });

    // input loop
    let cmd_in = ui_tx.clone();
    thread::spawn(move || { UserInputTask::new(cmd_in).run().unwrap(); });

    // master I/O loop
    let mut command_executor = CommandExecutor::new(cmd_tx);

    windows.push(Box::new(ScheduleWindow::new()));
    windows.push(Box::new(PromptWindow::new()));

    loop {
        match ui_rx.recv() {
            Ok(cmd) => {
                info!("Processing command: {:?}", cmd);
                match cmd {
                    UICommand::Execute(command) => {
                        match command.to_lowercase().as_str() {
                            "exit" => break,
                            _ => {
                                let item = match windows.iter().find(|w| w.selected_item().is_some()) {
                                    None => None,
                                    Some(window) => { window.selected_item() }
                                };
                                info!("Selected item: {:?}", item);
                                command_executor.execute_command(&command, item)?;
                            }
                        }
                    }

                    UICommand::Exit => { break; } // time to quit!

                    ui_cmd => {
                        for w in windows.iter_mut() {
                            if w.handle(&ui_cmd) { break; }
                        }

                        for w in windows.iter_mut() {
                            w.render(&mut stdout);
                        }
                        stdout.flush()?;
                    }
                }
            }
            Err(_) => {}
        }
    }

    Ok(())
}

fn init_logging()  {
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("log/todor.log").expect("Could not open log file!")),
        ]
    ).expect("Could not initiate logging!");
}
