use crate::scheduled_item::ScheduledItem;
use chrono::{Local, DateTime, Timelike};

const DURING_MEETING: i64 = FIVE_MIN_PAST - 1;
const FIVE_MIN_PAST: i64 = -5 * 60 * 1000;
const TEN_MIN_TIL: i64 = 10 * 60 * 1000;
const NEG_ETERNITY: i64 = -9223372036854775800; // min_val, basically.

pub fn format_item(item: &ScheduledItem, is_selected: bool, max_width: usize) -> Option<String> {
    let time_remaining = (item.start_time - Local::now()).num_milliseconds();
    let formatted = format_item_with_time_remaining(item, time_remaining, max_width);
    if is_selected {
        format_selection(formatted, max_width)
    } else {
        formatted
    }
}

fn format_selection(formatted_item: Option<String>, max_width: usize) -> Option<String> {
    match formatted_item {
        None => None,
        Some(s) => Some(format!(">  {0:<width$}", s, width=max_width))
    }
}

fn format_item_with_time_remaining(item: &ScheduledItem, time_remaining: i64, max_width: usize) -> Option<String> {
    match time_remaining {
        NEG_ETERNITY..=DURING_MEETING=> Some(format_with_end_time(item, max_width)),
        FIVE_MIN_PAST..=TEN_MIN_TIL  => Some(format_with_location(item, max_width)),
        _                            => Some(format_without_location(item, max_width)),
    }
}

fn format_without_location(item: &ScheduledItem, max_width: usize) -> String {
    format!("{0:<2$}  {1}", item.description, format_time(item.start_time), max_width + 2)
}

fn format_with_location(item: &ScheduledItem, max_width: usize) -> String {
    match item.place.as_ref() {
        Some(place) => format!("{0:<3$}  {1}\n\t{2}", item.description, format_time(item.start_time), place, max_width + 2),
        None => format_without_location(item, max_width)
    }
}

fn format_with_end_time(item: &ScheduledItem, max_width: usize) -> String {
    match item.end_time {
        Some(end_time) => format!("{0:<2$}  -{1}", item.description, format_time(end_time), max_width + 1),
        None => format!("{0:<2$}  {1}", item.description, format_time(item.start_time), max_width + 1)
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
        assert_eq!("A meeting               12:01", formatted_str);
    }

    #[test]
    fn requests_close_to_start_time_include_location() {
        let formatted_str = format_item_with_time_remaining(&item(), 2, 20).unwrap();
        assert_eq!("A meeting               12:01\n\tlocation", formatted_str);
    }

    #[test]
    fn requests_after_start_time_but_within_window_include_location() {
        let formatted_str = format_item_with_time_remaining(&item(), -6000, 20).unwrap();
        assert_eq!("A meeting               12:01\n\tlocation", formatted_str);
    }

    #[test]
    fn requests_long_after_start_time_show_end_time() {
        let formatted_str = format_item_with_time_remaining(&item(), DURING_MEETING - 1, 20).unwrap();
        assert_eq!("A meeting              -13:13", formatted_str);
    }

    fn item() -> ScheduledItem {
        let start_time = Local.ymd(2020, 4, 2).and_hms(12, 1, 13);
        let end_time = Local.ymd(2020, 4, 2).and_hms(13, 13, 13);
        let location = Some("location".to_string());
        ScheduledItem::new("id".to_string(), "A meeting".to_string(), start_time, Some(end_time), location)
    }

}