use crate::tasks::CommandExecutor;
use crate::commands::{ScheduleCommand, UICommand};
use std::sync::mpsc::Sender;
use log::info;
use crate::scheduled_item::ScheduledItem;

impl CommandExecutor {
    pub fn new(cmd_tx: Sender<ScheduleCommand>, ui_tx: Sender<UICommand>) -> Self {
        CommandExecutor { cmd_tx, ui_tx }
    }

    pub fn execute_command(&mut self, command: &String, selected_item: Option<&ScheduledItem>) -> anyhow::Result<()> {
        let mut parts = command.trim().split_ascii_whitespace();
        let cmd = parts.next().unwrap_or("");
        let remainder: String = parts.collect::<Vec<_>>().join(" ");

        match cmd.to_lowercase().as_str() {
            "" => {}, // User just hit <enter> on an empty string.
            "refresh" => { self.cmd_tx.send(ScheduleCommand::Refresh)?; },
            "add" => {
                self.ui_tx.send(UICommand::TransitionPush("schedule_selection".to_string()));
                self.ui_tx.send(UICommand::AddGetScheduler(remainder));
            }
            "close" | "ack" => {
                let item_to_close = match &selected_item {
                    None => { remainder }
                    Some(item) => { item.description.clone() }
                };

                info!("Attempting to ack item: {:?}", item_to_close);

                let scheduler_id = match selected_item {
                    None => { "todoist:Inbox".to_string() } // TODO: Prompt for this
                    Some(item) => { item.scheduler.clone() }
                };

                self.cmd_tx.send(ScheduleCommand::CloseTodo(scheduler_id, item_to_close))?;
            }
            _ => { info!("Unknown TodoR command: {}", cmd); }
        }

        Ok(())
    }
}