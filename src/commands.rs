use crate::scheduled_item::{ScheduledItem, Scheduler};
use crate::display::PromptMessage;

#[derive(Clone, Debug)]
pub enum UICommand {
    Schedules(Vec<ScheduledItem>),
    Schedulers(Vec<String>),

    Toast(PromptMessage),
    SetPrompt(String),
    UpdateUserInput(String),
    SubmitCommand(String),

    ExecuteWithItem(String, ScheduledItem),
    Execute(String),

    TransitionPush(String),
    TransitionPop,

    ClearSelection,
    SelectPrev,
    SelectNext,

    AddGetScheduler(String),
    Add(SchedulerAccountId, String),

    Exit
}

pub type SchedulerAccountId = String;

#[derive(Clone, Debug)]
pub enum ScheduleCommand {
    Refresh,
    Add(SchedulerAccountId, String),
    CloseTodo(SchedulerAccountId, String),
}