use crate::display::{Window, ScheduleSelectionWindow};
use crate::commands::UICommand;
use std::io::Write;
use crate::scheduled_item::ScheduledItem;
use std::cmp::{max, min};
use std::sync::mpsc::Sender;

impl ScheduleSelectionWindow {
    pub fn new(schedules: Vec<String>, ui_tx: Sender<UICommand>) -> Self {
        ScheduleSelectionWindow {
            ui_tx,
            schedules,
            task: None,
            selected_item_idx: -1
        }
    }

    pub fn selected_scheduler_id(&self) -> Option<String> {
        match self.selected_item_idx {
            -1 => None,
            _ => Some(self.schedules.get(self.selected_item_idx as usize).unwrap().clone())
        }
    }

}

impl Window for ScheduleSelectionWindow {
    fn id(&self) -> String {
        "schedule_selection".to_string()
    }

    fn active(&self) -> bool {
        true
    }

    fn enable(&mut self) { }

    fn disable(&mut self) { }

    fn handle(&mut self, ui_cmd: &UICommand) -> bool {
        match ui_cmd {
            UICommand::ClearSelection => { self.selected_item_idx = -1; true }
            UICommand::SelectPrev => { self.selected_item_idx = max(-1, self.selected_item_idx - 1); true }
            UICommand::SelectNext => { self.selected_item_idx = min(self.schedules.len() as i32 - 1, self.selected_item_idx + 1); true }
            UICommand::AddGetScheduler(task) => { self.task = Some(task.clone()); true }
            UICommand::UpdateUserInput(_) => { true } // Block typing to the prompt until we have a selection
            UICommand::SubmitCommand(_) => {
                match self.selected_scheduler_id() {
                    None => {}
                    Some(sched_id) => {
                        match &self.task {
                            None => {}
                            Some(task) => { self.ui_tx.send(UICommand::Add(sched_id, task.clone())); }
                        }
                    }
                }
                self.ui_tx.send(UICommand::TransitionPop );
                true
            }
            _ => false
        }
    }

    fn render(&self, target: &mut dyn Write) {
        let mut output = vec![];

        write!(output, "{}{}Select Scheduler:\r\n-------------\r\n",
            termion::clear::All,
            termion::cursor::Goto(1, 1));

        for i in 0..self.schedules.len() {
            if self.selected_item_idx == i as i32 { write!(output, "> {}\r\n", self.schedules.get(i).unwrap()); }
            else { write!(output, "  {}\n\r", self.schedules.get(i).unwrap()); }
        }

        write!(target, "{}", String::from_utf8(output).unwrap());
    }

    fn selected_item(&self) -> Option<&ScheduledItem> {
        None
    }
}