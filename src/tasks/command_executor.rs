use crate::tasks::CommandExecutor;
use crate::commands::ScheduleCommand;
use std::sync::mpsc::Sender;
use log::info;

impl CommandExecutor {
    pub fn new(cmd_tx: Sender<ScheduleCommand>) -> Self {
        CommandExecutor { cmd_tx }
    }

    pub fn execute_command(&mut self, command: &String) -> anyhow::Result<()> {
        let mut parts = command.trim().split_ascii_whitespace();
        let cmd = parts.next().unwrap_or("");

        match cmd.to_lowercase().as_str() {
            "" => {}, // User just hit <enter> on an empty string.
            "refresh" => { self.cmd_tx.send(ScheduleCommand::Refresh)? },
            _ => { info!("Unknown TodoR command: {}", cmd); }
        }

        Ok(())
    }
}