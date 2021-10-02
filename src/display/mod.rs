use crate::commands::UICommand;
use chrono::{DateTime, Local};
use crate::scheduled_item::ScheduledItem;
use std::sync::mpsc::Sender;

mod schedule_window;
mod prompt_window;

#[derive(Clone, Debug)]
pub enum PromptMessageType {
    Normal,
    Error
}

#[derive(Clone, Debug)]
pub struct PromptMessage {
    text: String,
    ttl: DateTime<Local>,
    message_type: PromptMessageType
}

#[derive(Clone, Debug)]
pub struct PromptWindow {
    user_input: String,
    prompt: String,
    message: Option<PromptMessage>
}

#[derive(Clone, Debug)]
pub struct ScheduleWindow {
    schedules: Vec<ScheduledItem>,
    selected_item_idx: i32
}

pub trait Window {
    fn handle(&mut self, data: &UICommand) -> bool;
    fn render(&self, target: &mut dyn std::io::Write);
    fn selected_item(&self) -> Option<&ScheduledItem>;
}