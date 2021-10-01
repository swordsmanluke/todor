use crate::tasks::CommandExecutor;
use crate::commands::ScheduleCommand;
use std::sync::mpsc::Sender;
use log::info;
use crate::scheduled_item::ScheduledItem;

impl CommandExecutor {
    pub fn new(cmd_tx: Sender<ScheduleCommand>) -> Self {
        CommandExecutor { cmd_tx }
    }

    pub fn execute_command(&mut self, command: &String, selected_item: Option<&ScheduledItem>) -> anyhow::Result<()> {
        let mut parts = command.trim().split_ascii_whitespace();
        let cmd = parts.next().unwrap_or("");

        match cmd.to_lowercase().as_str() {
            "" => {}, // User just hit <enter> on an empty string.
            "refresh" => { self.cmd_tx.send(ScheduleCommand::Refresh)?; },
            "add" => {
                // TODO: Determine scheduler by prompting user... to select one?
                let scheduler_id = "todo:Inbox".to_string();
                let task = parts.collect::<Vec<_>>().join(" ");
                self.cmd_tx.send(ScheduleCommand::Add(scheduler_id, task))?;
            }
            "close" | "ack" => {
                let item_to_close = match &selected_item {
                    None => { parts.collect::<Vec<_>>().join(" ") }
                    Some(item) => { item.description.clone() }
                };

                info!("Attempting to ack item: {:?}", item_to_close);

                let scheduler_id = match selected_item {
                    None => { "todo:Inbox".to_string() } // TODO: Prompt for this
                    Some(item) => { item.scheduler.clone() }
                };

                self.cmd_tx.send(ScheduleCommand::CloseTodo(scheduler_id, item_to_close))?;
            }
            _ => { info!("Unknown TodoR command: {}", cmd); }
        }

        Ok(())
    }
}