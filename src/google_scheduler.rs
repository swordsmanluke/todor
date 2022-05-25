use google_calendar3::{CalendarHub, Event, EventDateTime};
use hyper::Client;
use yup_oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use crate::scheduled_item::{ScheduledItem, Scheduler, ScheduleItemType};
use chrono::{DateTime, Local, Duration, TimeZone, Datelike, NaiveDate};
use std::ops::Add;
use std::error::Error;
use crate::google_calendar_client::{JsonTokenStorage, create_gcal_client};
use std::str::FromStr;
use log::info;

pub struct GoogleScheduler {
    pub calendar_name: String,
    pub hub: CalendarHub<Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, Client>>,
    cache: Vec<ScheduledItem>,
}

pub(crate) fn create_gcal_scheduler(auth_file: String, cal_name: String) -> Result<GoogleScheduler, Box<dyn Error>> {
    let gcc = create_gcal_client(auth_file)?;
    Ok(GoogleScheduler::new(cal_name, gcc))
}

impl GoogleScheduler {
    pub fn new(cal_name: String, hub: CalendarHub<Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, Client>>) -> GoogleScheduler {
        GoogleScheduler {
            calendar_name: cal_name,
            hub,
            cache: Vec::new()
        }
    }
}

impl Scheduler for GoogleScheduler {
    fn id(&self) -> String {
        format!("google:{}", self.calendar_name)
    }

    fn refresh(&mut self) -> Result<(), Box<dyn Error>> {
        let start_time = Local::now().add(Duration::minutes(-10)).to_rfc3339().clone();
        let end_time = Local::now().add(Duration::days(2)).to_rfc3339();

        let events = self.hub.events().list(self.calendar_name.as_ref()).
            time_min(start_time.as_str()).
            time_max(end_time.as_str()).
            single_events(true).
            doit()?.1;

        self.cache = events.items.unwrap().iter().
            map(|t| cal_event_to_scheduled_item(self.calendar_name.clone(),  t)).
            filter(|t| t.is_some()).
            map(|t| t.unwrap()).
            collect();

        Ok(())
    }

    fn schedule(&self) -> Vec<ScheduledItem> {
        self.cache.clone()
    }

    fn add(&mut self, target: &String, due_date: Option<DateTime<Local>>) -> anyhow::Result<bool> {
        // Not yet implemented
        Ok(false)
    }

    fn update(&mut self, id: &String, description: &String, due_date: Option<DateTime<Local>>) -> anyhow::Result<bool> {
        // Not yet implemented
        Ok(false)
    }

    fn remove(&mut self, prefix: &String) -> anyhow::Result<bool> {
        // Not yet implemented
        Ok(false)
    }
}

fn cal_event_to_scheduled_item(account_id: String, e: &Event) -> Option<ScheduledItem> {

    let description = e.summary.clone().unwrap_or("no desc".to_string());
    let place = format_location(e.location.clone());

    let start_time = event_start_time(e);
    let end_time = event_end_time(e);
    match (start_time, end_time) {
        (Some(start_time), _) => Some(ScheduledItem::new(e.i_cal_uid.clone().unwrap_or("".to_string()),
                                                         format!("google:{}", account_id),
                                                         ScheduleItemType::Calendar,
                                                         description,
                                                         start_time,
                                                         end_time,
                                                         place)),
        _ => None
    }
}

fn event_start_time(e: &Event) -> Option<DateTime<Local>> {

    let time = match (e.start.as_ref(), e.original_start_time.as_ref()) {
        (None, None) => None,
        (Some(s), None) => Some(s.clone()),
        (None, Some(s)) => Some(s.clone()),
        (Some(s1), Some(s2)) => Some(s1.clone())
    };

    Some(convert_event_time(time.unwrap()))
}

fn event_end_time(e: &Event) -> Option<DateTime<Local>> {
    let time = match e.end.as_ref() {
        None => None,
        Some(end) => Some(end.clone()),
    };

    if time.is_none() { return None; }

    Some(convert_event_time(time.unwrap()))
}

fn convert_event_time(time: EventDateTime) -> DateTime<Local> {
    let datetime = match time.date_time {
        None => None,
        Some(s) => Some(DateTime::from_str(&s).unwrap())
    };

    let all_day = match time.date {
        None => None,
        Some(d) => {
            let nd = NaiveDate::from_str(&d.as_ref()).unwrap();
            Some(Local.ymd(nd.year(), nd.month(), nd.day()))
        }
    };

    let final_date = match datetime {
        Some(dt) => dt,
        None => match all_day {
            Some(ad) => ad.and_hms(23, 59, 59),
            None => Local::now()
        }
    };

    if final_date.date() >= Local::now().date() {
        final_date
    } else {
        Local::now().date().and_hms(23, 59, 59)
    }
}

fn format_location(location: Option<String>) -> Option<String> {
    match location {
        None => None,
        Some(place) => Some(strip_flexe(place))
    }
}

fn strip_flexe(location: String) -> String {
    location.replace("Flexe HQ-6-", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_locations_include_flexe_prefix_we_remove_it() {
        let before = "Flexe HQ-6-Warehouser 12".to_string();
        let after = "Warehouser 12".to_string();
        assert_eq!(strip_flexe(before), after);
    }
}








