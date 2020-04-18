use google_calendar3::{CalendarHub, Event, EventDateTime};
use hyper::Client;
use yup_oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use crate::scheduled_item::ScheduledItem;
use chrono::{DateTime, Local, Duration, TimeZone, Datelike, NaiveDate};
use std::ops::Add;
use std::error::Error;
use crate::google_calendar_client::JsonTokenStorage;
use std::str::FromStr;

pub struct GoogleScheduler {
    pub calendar_name: String,
    pub hub: CalendarHub<Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, Client>>,
}

impl GoogleScheduler {
    pub fn new(cal_name: &str, hub: CalendarHub<Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, Client>>) -> GoogleScheduler {
        GoogleScheduler {
            calendar_name: cal_name.to_string(),
            hub,
        }
    }

    pub fn get_schedule(&mut self) -> Result<Vec<ScheduledItem>, Box<dyn Error>> {
        let start_time = Local::now().add(Duration::minutes(-10)).to_rfc3339().clone();
        let end_time = Local::now().add(Duration::days(2)).to_rfc3339();

        let events = self.hub.events().list(self.calendar_name.as_ref()).
            time_min(start_time.as_str()).
            time_max(end_time.as_str()).
            single_events(true).
            doit()?.1;

        let scheduled_items = events.items.unwrap().iter().
            map(|t| cal_event_to_scheduled_item(t)).
            filter(|t| t.is_some()).
            map(|t| t.unwrap()).
            collect();

        Ok(scheduled_items)
    }
}

fn cal_event_to_scheduled_item(e: &Event) -> Option<ScheduledItem> {

    let description = e.summary.clone().unwrap_or("no desc".to_string());
    let place = format_location(e.location.clone());

    match event_start_time(e) {
        None => None,
        Some(time) => Some(ScheduledItem::new(description, time, place))
    }
}

fn event_start_time(e: &Event) -> Option<DateTime<Local>> {

    let time = match (e.start.as_ref(), e.original_start_time.as_ref()) {
        (None, None) => None,
        (Some(s), None) => Some(s.clone()),
        (_, Some(s)) => Some(s.clone())
    };

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

    match datetime {
        Some(dt) => dt,
        None => match all_day {
            Some(ad) => ad.and_hms(23, 59, 59),
            None => Local::now()
        }
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








