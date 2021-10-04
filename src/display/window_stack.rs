use crate::display::{Window, PromptWindow, WindowStack, ScheduleSelectionWindow, ScheduleWindow};
use std::io::Write;
use crate::commands::UICommand;
use log::info;
use crate::scheduled_item::ScheduledItem;
use std::sync::mpsc::Sender;

impl WindowStack {
    pub fn new(ui_tx: Sender<UICommand>) -> Self {
        WindowStack {
            ui_tx: ui_tx.clone(),
            windows: vec![Box::new(PromptWindow::new(ui_tx))],
            scheduler_ids: vec![]
        }
    }

    pub fn handle_ui_command(&mut self, cmd: UICommand, stdout: &mut dyn Write) {
        match cmd {
            UICommand::Schedulers(schedulers) => { self.scheduler_ids = schedulers; }

            UICommand::TransitionPush(window_type) => {
                let new_window: Option<Box<dyn Window>> = match window_type.to_lowercase().as_str() {
                    "schedule_selection" => {
                        Some(Box::new(ScheduleSelectionWindow::new(self.scheduler_ids.clone(), self.ui_tx.clone())))
                    }
                    "schedule" => {
                        Some(Box::new(ScheduleWindow::new(self.ui_tx.clone())))
                    }
                    _ => None
                };

                match new_window {
                    None => {}
                    Some(window) => { self.push(window); }
                }
            }

            UICommand::TransitionPop => { self.pop(); }

            _ => {
                // Send events to the _last_ (e.g. topmost) window first, then work down.
                for w in self.windows.iter_mut().rev() {
                    if w.handle(&cmd) { info!("Handled by {}", w.id()); break; }
                }

                self.render(stdout);
            }
        }
    }

    pub fn render(&mut self, stdout: &mut dyn Write) {
        // save the prompt for the end - always the zeroth element
        self.windows.iter().skip(1).
            filter(|w| w.active()).
            for_each(|w|
            w.render(stdout));

        self.windows.first().unwrap().render(stdout);
        stdout.flush();
    }

    pub fn push(&mut self, mut window: Box<dyn Window>) {
        window.enable();
        self.windows.push(window);
    }

    pub fn pop(&mut self) {
        if self.windows.len() > 1 { // never pop the prompt!
            self.windows.pop();
        }
    }
}
