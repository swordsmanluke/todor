use crate::scheduled_item::ScheduledItem;
use chrono::{DateTime, Local};

#[derive(Clone, Debug)]
pub enum UICommand {
    Schedules(Vec<ScheduledItem>),
    UpdateUserInput(String),
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
    AddTodo(SchedulerAccountId, String),
    CloseTodo(SchedulerAccountId, String),
    AddCal(SchedulerAccountId, String, DateTime<Local>)
}