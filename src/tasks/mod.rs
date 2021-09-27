use std::sync::mpsc::{Sender, Receiver};
use crate::commands::{UICommand, ScheduleCommand};
use crate::scheduled_item::Scheduler;

mod schedule_refresh;
mod user_input_task;
mod command_executor;

pub struct MasterScheduler {
    ui_sched_tx: Sender<UICommand>,
    schedulers: Vec<Box<dyn Scheduler>>,
    cmd_rx: Receiver<ScheduleCommand>,
}

pub struct UserInputTask {
    ui_tx: Sender<UICommand>,
    user_input: String
}

pub struct CommandExecutor {
    cmd_tx: Sender<ScheduleCommand>,
}
