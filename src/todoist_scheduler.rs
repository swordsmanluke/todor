use crate::schedule_trait::ScheduledItem;
use crate::todoist_client::*;
use std::fs::File;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ApiToken {
    pub token: String
}

pub struct TodoistScheduler {
    pub token: String
}

impl TodoistScheduler {
    pub fn new(auth_file: &str) -> TodoistScheduler {
        // Read in the auth file and configure ourselves
        let file = File::open(auth_file).unwrap();
        let todoist_token: ApiToken = serde_json::from_reader(file).expect("Badly formatted auth token file!");

        TodoistScheduler {
            token: todoist_token.token
        }
    }

    pub fn get_schedule() -> Vec<ScheduledItem> {
        Vec::new()
    }
}








