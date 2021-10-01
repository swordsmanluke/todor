use crate::scheduled_item::{Scheduler, ScheduleConfig, load_scheduler_config};
use crate::tasks::MasterScheduler;
use crate::google_scheduler::create_gcal_scheduler;
use crate::todoist_scheduler::create_todoist_scheduler;
use std::sync::mpsc::{Sender, Receiver};
use crate::commands::{UICommand, ScheduleCommand, SchedulerAccountId};
use std::error::Error;
use std::time::Duration;
use log::info;
use date_time_parser::DateParser;
use chrono::{Local, TimeZone, NaiveTime};
use itertools::Itertools;

impl MasterScheduler {
    pub fn new(ui_sched_tx: Sender<UICommand>, cmd_rx: Receiver<ScheduleCommand>) -> Self {
        let cfg = load_scheduler_config().unwrap();

        MasterScheduler {
            cmd_rx,
            ui_sched_tx,
            schedulers: load_schedulers(cfg).unwrap()
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()>{
        self.refresh().unwrap();

        loop {
            match self.cmd_rx.recv_timeout(Duration::from_secs(60)) {
                Ok(command) => {
                    match command {
                        ScheduleCommand::Refresh => { self.refresh()?; }
                        ScheduleCommand::AddTodo(account_id, task) => {
                            self.add_todo_task(account_id, &task)
                        }
                        ScheduleCommand::AddCal(account_id, task, time) => {}
                        ScheduleCommand::CloseTodo(account_id, task) => {

                            match self.schedulers.iter_mut().find(|f| f.id() == account_id) {
                                None => {info!("Could not find scheduler {} in {:?}", account_id, self.schedulers.iter().map(|s| s.id()).collect::<Vec<_>>());}
                                Some(scheduler) => {
                                    info!("Removing task {}", task);
                                    if let Ok(true) = scheduler.remove(&task) {
                                        self.refresh();
                                        self.ui_sched_tx.send(UICommand::ClearSelection);
                                    } else {
                                        // TODO: Display an error message
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    // The error we get here is always a RecvTimeoutErr.
                    // when it happens, it indicates we've been waiting long enough
                    // that we need to refresh. So... we do!
                    //
                    // The timing could get thrown off by other command processing
                    // but I don't think it'll be a big deal in practice, given the
                    // relative paucity of commands vs the longer waiting periods
                    self.refresh()?;
                }
            }
        }

        Ok(())
    }

    fn add_todo_task(&mut self, account_id: SchedulerAccountId, task: &String) {
        info!("Attempting to add '{}' to todo list '{}' ", task, account_id);
        match self.schedulers.iter_mut().find(|f| f.id() == account_id) {
            None => { info!("Could not find account '{}'. Schedulers: {:?}", account_id, self.schedulers.iter().map(|s| s.id()).collect::<Vec<_>>()) }
            Some(scheduler) => {
                // TODO: Replace this with the 'to_event' parser... as soon as I understand how to get data OUT of it.
                let mut due_date = match DateParser::parse(&task) {
                    None => { Local::today().and_hms(23, 59, 59) }
                    Some(d) => { Local.from_local_date(&d).and_time(NaiveTime::from_hms(23, 59, 59)).unwrap() }
                };
                scheduler.add(task, Some(due_date));
            }
        }
    }

    fn refresh(&mut self) -> anyhow::Result<()>{
        self.schedulers.
            iter_mut().
            for_each(|s| { s.refresh().unwrap(); });

        let mut final_schedule = self.schedulers.
            iter().
            flat_map(|s| s.schedule()).
            collect::<Vec<_>>();

        final_schedule.sort_by_key(|s| s.start_time);

        self.ui_sched_tx.send(UICommand::Schedules(final_schedule))?;

        Ok(())
    }

}

fn load_schedulers(cfg: ScheduleConfig) -> Result<Vec<Box<dyn Scheduler>>, Box<dyn Error>> {
    let mut schedulers: Vec<Box<dyn Scheduler>> = Vec::new();
    for gc in cfg.google_cal {
        let auth_file = format!("config/{}.json", gc.name);
        schedulers.push(Box::new(create_gcal_scheduler(auth_file, gc.cal_name)?));
    }
    for td in cfg.todoist {
        schedulers.push(Box::new(create_todoist_scheduler(td.name, td.project)?));
    }
    Ok(schedulers)
}