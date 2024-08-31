use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use time::OffsetDateTime;
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
    todo!()
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