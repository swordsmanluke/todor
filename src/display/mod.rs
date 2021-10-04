use crate::commands::UICommand;
use chrono::{DateTime, Local};
use crate::scheduled_item::ScheduledItem;
use std::sync::mpsc::Sender;

mod schedule_window;
mod prompt_window;
mod window_stack;
mod schedule_selection_window;

pub struct WindowStack {
    windows: Vec<Box<dyn Window>>,
    scheduler_ids: Vec<String>,
    pub ui_tx: Sender<UICommand>,
}

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
    active: bool,
    user_input: String,
    prompt: String,
    message: Option<PromptMessage>,
    pub ui_tx: Sender<UICommand>,
}

#[derive(Clone, Debug)]
pub struct ScheduleWindow {
    active: bool,
    schedules: Vec<ScheduledItem>,
    selected_item_idx: i32,
    pub ui_tx: Sender<UICommand>,
}

#[derive(Clone, Debug)]
pub struct ScheduleSelectionWindow {
    ui_tx: Sender<UICommand>,
    schedules: Vec<String>,
    selected_item_idx: i32,
    pub task: Option<String>,
}

pub trait Window {
    fn id(&self) -> String;
    fn active(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn handle(&mut self, data: &UICommand) -> bool;
    fn render(&self, target: &mut dyn std::io::Write);
    // TODO: This is the wrong abstraction. Come up with a better way to retrieve this
    fn selected_item(&self) -> Option<&ScheduledItem>;
}