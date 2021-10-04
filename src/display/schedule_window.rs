use crate::display::{Window, ScheduleWindow};
use crate::scheduled_item::ScheduledItem;
use crate::commands::UICommand;
use std::cmp::{min, max};
use crate::MAX_WIDTH;
use itertools::*;
use crate::schedule_formatter::format_item;
use crate::schedule_colorer::color_item;
use std::io::Write;
use std::sync::mpsc::Sender;

impl ScheduleWindow {
    pub fn new(ui_tx: Sender<UICommand>) -> Self {
        ScheduleWindow {
            ui_tx,
            active: true,
            schedules: vec![],
            selected_item_idx: -1
        }
    }

    fn render_schedule(&self, items: &Vec<ScheduledItem>, selected_item_idx: i32, stdout: &mut dyn Write) -> anyhow::Result<()> {
        let (_, rows) = termion::terminal_size().unwrap();

        // Clear the screen and go to the top line before we start
        stdout.write(b"\x1B[2J\x1B[1;1H")?;

        let mut items = items.clone();
        items.sort_by_key(|f| f.start_time);

        // max_width is determined by the widest description or MAX_WIDTH, whichever is smaller.
        let max_width = min(MAX_WIDTH, items.iter().map(|i| i.description.len()).max().unwrap_or(MAX_WIDTH));

        let grouped_by_date = items.into_iter().group_by(|item| item.start_time.date());
        let mut item_count = 0;

        let mut output = Vec::new();
        for (date, items_for_date) in &grouped_by_date
        {
            let ds: String = date.to_string();
            // Print the date
            write!(output, "{}\n\r", ds.get(0..ds.len() - 6).unwrap())?;
            let item_vec: Vec<ScheduledItem> = items_for_date.collect::<Vec<ScheduledItem>>();
            write!(output, "--------{}---------\r\n", item_vec.len())?; // divider

            // Print the date's schedule
            for item in item_vec {
                match format_item(&item, item_count == selected_item_idx, max_width) {
                    Some(s) => { write!(output, "  {}\n\r", color_item(&item, &s))?; },
                    None => {}
                }
                item_count += 1;
            }

            write!(output, "\n\r")?;
        }

        write!(stdout, "{}", String::from_utf8(output).unwrap().split("\n").take(rows as usize - 1).join("\n"))?;

        Ok(())
    }
}

impl Window for ScheduleWindow {
    fn id(&self) -> String {
        String::from("schedule")
    }

    fn active(&self) -> bool {
        self.active
    }

    fn enable(&mut self) {
        self.active = true
    }

    fn disable(&mut self) {
        self.active = false
    }

    fn handle(&mut self, data: &UICommand) -> bool {
        match data{
            UICommand::Schedules(sched) => { self.schedules = sched.clone(); true }
            UICommand::SelectNext => { self.selected_item_idx = min(self.schedules.len() as i32 - 1, self.selected_item_idx + 1); true }
            UICommand::SelectPrev => { self.selected_item_idx = max(-1, self.selected_item_idx - 1); true }
            UICommand::ClearSelection => { self.selected_item_idx = -1; true }
            UICommand::SubmitCommand(command) => {
                match command.to_lowercase().as_str() {
                    "ack" | "close" => {
                        // Check for a selected item
                        match self.selected_item() {
                            None => { self.ui_tx.send(UICommand::Execute(command.clone())); }
                            Some(item) => { self.ui_tx.send(UICommand::ExecuteWithItem(command.clone(), item.clone())); }
                        }
                        true
                    }

                    _ => { false }
                }
            }
            _ => { false }
        }
    }

    fn render(&self, target: &mut dyn Write) {
        self.render_schedule(&self.schedules, self.selected_item_idx, target);
    }

    fn selected_item(&self) -> Option<&ScheduledItem> {
        match self.selected_item_idx {
            -1 => None,
            _  => self.schedules.get(self.selected_item_idx as usize)
        }
    }
}