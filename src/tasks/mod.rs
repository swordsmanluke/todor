use std::sync::mpsc::{Sender, Receiver};
use crate::commands::{UICommand, ScheduleCommand};
use crate::scheduled_item::Scheduler;

mod master_scheduler;
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
    pub ui_tx: Sender<UICommand>,
}
