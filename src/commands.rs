use crate::scheduled_item::ScheduledItem;
use crate::display::PromptMessage;

#[derive(Clone, Debug)]
pub enum UICommand {
    Schedules(Vec<ScheduledItem>),
    UpdateUserInput(String),
    Toast(PromptMessage),
    SetPrompt(String),
    Execute(String),
    ClearSelection,
    SelectPrev,
    SelectNext,
    Exit
}

pub type SchedulerAccountId = String;

#[derive(Clone, Debug)]
pub enum ScheduleCommand {
    Refresh,
    Add(SchedulerAccountId, String),
    CloseTodo(SchedulerAccountId, String),
}