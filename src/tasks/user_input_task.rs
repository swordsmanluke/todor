use crate::tasks::UserInputTask;
use std::io::{stdin};
use crate::commands::UICommand;
use std::sync::mpsc::Sender;
use termion::input::TermRead;
use log::{info, error};
use termion::event::Key;
use std::cmp::max;

impl UserInputTask {
    pub fn new(cmd_input: Sender<UICommand>) -> Self {
        UserInputTask {
            ui_tx: cmd_input,
            user_input: String::new(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()>{
        let stdin = stdin();

        for key in stdin.keys() {
            match key {
                Ok(key) => {
                    self.process_input(key)?;
                    self.ui_tx.send(UICommand::UpdateUserInput(self.user_input.clone()))?;
                }
                Err(e) => { error!("{}", e); }
            }
        }

        Ok(())
    }

    fn process_input(&mut self, key: Key) -> anyhow::Result<()>{
        info!("Processing key event: {:?}", key);
        match key {
            Key::Char('\r') |
            Key::Char('\n') => {
                self.ui_tx.send(UICommand::Execute(self.user_input.clone()))?;
                self.user_input.clear();
            },
            Key::Backspace => { self.user_input.truncate(max(0, self.user_input.len() - 1)); }
            Key::Ctrl('c') |
            Key::Ctrl('d') => { self.ui_tx.send(UICommand::Exit)?; },
            Key::Ctrl('u') => { self.user_input.clear(); }
            Key::Char(c) => { self.user_input.push(c); },

            Key::Esc => { self.ui_tx.send(UICommand::ClearSelection)?; }
            Key::Up => { self.ui_tx.send(UICommand::SelectPrev)?; }
            Key::Down => { self.ui_tx.send(UICommand::SelectNext)?; }

            // Not implemented below here
            Key::Delete => {/* TODO: Add delete support once we have arrow keys */}
            // Arrows
            Key::Left => {}
            Key::Right => {}

            // Scrolling
            Key::Home => {}
            Key::End => {}
            Key::PageUp => {}
            Key::PageDown => {}

            Key::BackTab => {}
            Key::Insert => {} // TODO: ... I don't think I'll need this
            Key::F(_) => {}
            Key::Alt(_) => {}

            _ => {}
        }

        Ok(())
    }
}