use chrono::Local;
use crate::display::{Window, PromptMessage, PromptMessageType, PromptWindow};
use crate::commands::UICommand;
use std::io::Write;
use crate::scheduled_item::ScheduledItem;
use std::time::Duration;
use std::sync::mpsc::Sender;

impl PromptMessage {
    pub fn new(text: String, ttl: Duration, message_type: PromptMessageType) -> Self {
        PromptMessage { text, ttl: Local::now() + chrono::Duration::from_std(ttl).unwrap(), message_type }
    }
}

impl PromptWindow {
    pub fn new(ui_tx: Sender<UICommand>) -> Self {
        PromptWindow {
            ui_tx,
            active: true,
            user_input: String::new(),
            prompt: String::from(":>"),
            message: None
        }
    }

    fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
    }

    fn set_message(&mut self, message: PromptMessage) {
        self.message = Some(message)
    }
}

impl Window for PromptWindow {
    fn id(&self) -> String {
        "prompt".to_string()
    }

    fn active(&self) -> bool {
        self.active
    }

    fn enable(&mut self) {
        self.active = true;
    }

    fn disable(&mut self) {
        self.active = false;
    }

    fn handle(&mut self, data: &UICommand) -> bool {
        match data {
            UICommand::UpdateUserInput(input) => {
                self.user_input = input.clone();
                true
            }
            UICommand::Toast(msg) => {
                self.message = Some(msg.clone());
                true
            }
            UICommand::SubmitCommand(command) => {
                self.ui_tx.send(UICommand::Execute(command.clone()));
                true
            }
            _ => { false }
        }
    }

    fn render(&self, target: &mut dyn Write) {
        match &self.message {
            None => {}
            Some(msg) => {
               if msg.ttl > Local::now() {
                   // TODO: Color this line according to message type
                   write!(target, "{}{}{}{}",
                          termion::cursor::Goto(1, 999),
                          termion::cursor::Up(1),
                          termion::clear::AfterCursor,
                          msg.text
                   );
               }
            }
        }

        write!(target, "{}{}{} {}",
               termion::cursor::Goto(1, 999),
               termion::clear::AfterCursor,
               self.prompt, self.user_input);
    }

    fn selected_item(&self) -> Option<&ScheduledItem> {
        None
    }
}