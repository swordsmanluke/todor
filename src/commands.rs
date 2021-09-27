use crate::scheduled_item::ScheduledItem;

#[derive(Clone, Debug)]
pub enum UICommand {
    UpdateUserInput(String),
    Exit,
    Execute(String),
    Schedules(Vec<ScheduledItem>)
}

#[derive(Clone, Debug)]
pub enum ScheduleCommand {
    Refresh,
    Exit
}