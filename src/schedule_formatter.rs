use crate::schedule_trait::ScheduledItem;
use chrono::{Local, DateTime, Timelike};

const FIVE_MIN_PAST: i64 = -5 * 60 * 1000;
const TEN_MIN_TIL: i64 = 10 * 60 * 1000;
const AFTER_TEN_MIN: i64 = 1 + TEN_MIN_TIL;
const ETERNITY: i64 = 9223372036854775807; // max_val, basically.

pub fn format_item(item: &ScheduledItem, max_width: usize) -> Option<String> {
    let time_remaining = (Local::now() - item.time).num_milliseconds();
    format_item_with_time_remaining(item, time_remaining, max_width)
}

fn format_item_with_time_remaining(item: &ScheduledItem, time_remaining: i64, max_width: usize) -> Option<String> {
    match time_remaining {
        FIVE_MIN_PAST..=TEN_MIN_TIL  => Some(format_with_location(item, max_width)),
        AFTER_TEN_MIN..=ETERNITY     => Some(format_without_location(item, max_width)),
        _                            => None
    }
}

fn format_without_location(item: &ScheduledItem, max_width: usize) -> String {
    format!("{0:<2$}  {1}", item.description, format_time(item.time), max_width - 7)
}

fn format_with_location(item: &ScheduledItem, max_width: usize) -> String {
    match item.place.as_ref() {
        Some(place) => format!("{0:<3$}  {1}\n\t{2}", item.description, format_time(item.time) , place, max_width - 7),
        None => format_without_location(item, max_width)
    }
}

fn format_time(time: DateTime<Local>) -> String {
    format!("{}:{:02}", time.time().hour(), time.time().minute())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn requests_before_start_time_do_not_include_location() {
        let formatted_str = format_item_with_time_remaining(&item(), 30 * 60 * 1000, 20).unwrap();
        assert_eq!("A meeting      12:01", formatted_str);
    }

    #[test]
    fn requests_close_to_start_time_include_location() {
        let formatted_str = format_item_with_time_remaining(&item(), 2, 20).unwrap();
        assert_eq!("A meeting      12:01\n\tlocation", formatted_str);
    }

    #[test]
    fn requests_after_start_time_but_within_window_include_location() {
        let formatted_str = format_item_with_time_remaining(&item(), -6000, 20).unwrap();
        assert_eq!("A meeting      12:01\n\tlocation", formatted_str);
    }

    #[test]
    fn requests_after_start_time_and_outside_window_are_none() {
        let formatted_str = format_item_with_time_remaining(&item(), -6 * 60 * 1000, 20);
        assert_eq!(None, formatted_str);
    }

    fn item() -> ScheduledItem {
        let scheduled_time = Local.ymd(2020, 4, 2).and_hms(12, 1, 13);
        let location = Some("location".to_string());
        ScheduledItem::new("A meeting".to_string(), scheduled_time, location)
    }

}