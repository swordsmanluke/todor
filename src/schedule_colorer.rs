use crate::scheduled_item::ScheduledItem;
use chrono::Local;
use colored::*;


const THREE_MIN_TIL: i64 = 3 * 60 * 1000;
const CRITICAL_WARNING_STARTS: i64 = THREE_MIN_TIL - 1;
const TEN_MIN_TIL: i64 = 10 * 60 * 1000;
const ONE_MINUTE_PAST: i64 = -60 * 1000;

pub fn color_item(item: &ScheduledItem, text: &String) -> String {
    color_item_with_time_remaining(text, time_remaining_til(item))
}

fn time_remaining_til(item: &ScheduledItem) -> i64 {
    (item.time - Local::now()).num_milliseconds()
}

fn color_item_with_time_remaining(text: &String, time_remaining: i64) -> String {
    // All of these ranges are backwards, since we're looking at time remaining.
    // If you're 1 minute past, that's -60 seconds remaining.
    // If you're ten minutes 'til, that's 600 seconds remaining.
    // So, the ranges are out of order, but that's ok. It all works out right in the end.
    match time_remaining {
        ONE_MINUTE_PAST..=CRITICAL_WARNING_STARTS => text.red().bold().to_string(),
        THREE_MIN_TIL..=TEN_MIN_TIL => text.yellow().to_string(),
        _  => text.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    fn abs(val: i64) -> i64 {
        if val < 0 { -val }
        else { val }
    }
    #[test]
    fn when_time_remaining_is_two_minutes() {
        let item = item(120);
        let remaining = time_remaining_til(&item);
        assert!(abs(120 * 1000 - remaining) < 10);
    }

    #[test]
    fn when_more_than_ten_minutes_out_text_is_plain() {
        let colored = color_item_with_time_remaining(&"No rush".to_string(), TEN_MIN_TIL + 1);
        assert_eq!("No rush", colored);
    }

    #[test]
    fn when_within_ten_minutes_text_turns_yellow() {
        let colored = color_item_with_time_remaining(&"Almost Time".to_string(), TEN_MIN_TIL - 1);
        assert_eq!("Almost Time".yellow().to_string(), colored);
    }

    #[test]
    fn when_within_three_minutes_text_turns_yellow() {
        let colored = color_item_with_time_remaining(&"HURRY!".to_string(), THREE_MIN_TIL - 1);
        assert_eq!("HURRY!".red().bold().to_string(), colored);
    }

    #[test]
    fn when_within_one_minute_past_text_is_red() {
        let colored = color_item_with_time_remaining(&"On your way, I hope!".to_string(), ONE_MINUTE_PAST + 1);
        assert_eq!("On your way, I hope!".red().bold().to_string(), colored);
    }

    #[test]
    fn when_more_than_one_minute_past_text_is_plain() {
        let colored = color_item_with_time_remaining(&"Hope you're there".to_string(), ONE_MINUTE_PAST - 1);
        assert_eq!("Hope you're there", colored);
    }

    fn item(seconds_in_future: i64) -> ScheduledItem {
        let scheduled_time = Local.timestamp_millis_opt(Local::now().timestamp_millis() + seconds_in_future*1000).unwrap();
        let location = Some("location".to_string());
        ScheduledItem::new("A meeting".to_string(), scheduled_time, location)
    }

}