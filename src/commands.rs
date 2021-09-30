use crate::scheduled_item::ScheduledItem;

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

#[derive(Clone, Debug)]
pub enum ScheduleCommand {
    Refresh,
}