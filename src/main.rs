use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use time::{Date, OffsetDateTime, Time, UtcOffset};
use time::Month;

struct RRule {
    freq: String,
    until: String,
    count: u16,
}

struct TimeZone {
    id: String,
    offset: i32,
    dst_start: RRule,
    dst_end: RRule,
    dst_offset: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(args.get(1).expect("Input file not specified"));
}

fn offset_date_time_from_ical(ical_str: &str) -> OffsetDateTime {
    let mut start = 0;
    let mut local_time = false;

    if ical_str[0..4].eq("TZID") {
        local_time = true;
        for c in ical_str.chars() {
            start += 1;
            if c == ':' {
                break;
            }
        }
    } else if ical_str.chars().nth(15) != Some('Z') {
        panic!("Invalid date time format.");
    }

    let year = ical_str[start..start + 4].parse::<i32>().expect("Invalid date time format");
    let month = ical_str[start + 4..start + 6].parse::<u8>().expect("Invalid date time format");
    let month = match month {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => panic!("Invalid month value")
    };
    let day = ical_str[start + 6..start + 8].parse::<u8>().expect("Invalid date time format");

    let hour = ical_str[start + 9..start + 11].parse::<u8>().expect("Invalid date time format");
    let minute = ical_str[start + 11..start + 13].parse::<u8>().expect("Invalid date time format");
    let second = ical_str[start + 13..start + 15].parse::<u8>().expect("Invalid date time format");

    let date = Date::from_calendar_date(year, month, day).expect("Invalid date values");
    let time = Time::from_hms(hour, minute, second).expect("Invalid time values");

    let offset =
    if local_time {
        UtcOffset::from_hms(0, 0, 0).expect("Error processing offset")
        //TODO Get the TZID and find the matching offset to go with it. Use that.
    } else {
        UtcOffset::from_hms(0, 0, 0).expect("Error processing offset")
    };

    OffsetDateTime::new_in_offset(date, time, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ical_datetime_utc_to_pst() {
        let dt = offset_date_time_from_ical("20241103T120000Z");

        assert!(dt.year() == 2024);
        assert!(dt.month() == Month::November);
        assert!(dt.day() == 3);
        assert!(dt.hour() == 4);
        assert!(dt.minute() == 00);
        assert!(dt.second() == 00);
    }

    #[test]
    fn ical_datetime_utc_to_pdt() {
        let dt = offset_date_time_from_ical("20240310T120000Z");

        assert!(dt.year() == 2024);
        assert!(dt.month() == Month::March);
        assert!(dt.day() == 10);
        assert!(dt.hour() == 5);
        assert!(dt.minute() == 00);
        assert!(dt.second() == 00);
    }

    #[test]
    fn ical_datetime_pst() {
        let dt = offset_date_time_from_ical("TZID=America/Los_Angeles:20240228T221518");

        assert!(dt.year() == 2024);
        assert!(dt.month() == Month::February);
        assert!(dt.day() == 28);
        assert!(dt.hour() == 22);
        assert!(dt.minute() == 15);
        assert!(dt.second() == 18);
    }

    #[test]
    fn ical_datetime_pdt() {
        let dt = offset_date_time_from_ical("TZID=America/Los_Angeles:20240828T221518");

        assert!(dt.year() == 2024);
        assert!(dt.month() == Month::August);
        assert!(dt.day() == 28);
        assert!(dt.hour() == 22);
        assert!(dt.minute() == 15);
        assert!(dt.second() == 18);
    }
}