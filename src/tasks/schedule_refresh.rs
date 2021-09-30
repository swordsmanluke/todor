use crate::scheduled_item::{Scheduler, ScheduleConfig, load_scheduler_config};
use crate::tasks::MasterScheduler;
use crate::google_scheduler::create_gcal_scheduler;
use crate::todoist_scheduler::create_todoist_scheduler;
use std::sync::mpsc::{Sender, Receiver};
use crate::commands::{UICommand, ScheduleCommand};
use std::error::Error;
use std::time::Duration;
use log::info;

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
                            info!("Attempting to add '{}' to todo list '{}' ", task, account_id);
                            match self.schedulers.iter_mut().find(|f| f.id() == account_id) {
                                None => { info!("Could not find account '{}'. Schedulers: {:?}", account_id, self.schedulers.iter().map(|s| s.id()).collect::<Vec<_>>())}
                                Some(scheduler) => { scheduler.add(task); }
                            }
                        }
                        ScheduleCommand::AddCal(account_id, task, time) => {}
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

    fn refresh(&mut self) -> anyhow::Result<()>{
        self.schedulers.
            iter_mut().
            for_each(|s| { s.refresh().unwrap(); });

        let final_schedule = self.schedulers.
            iter().
            flat_map(|s| s.schedule()).
            collect();

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