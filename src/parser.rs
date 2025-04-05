use std::fs::File;
use std::io::{BufReader, Error, Lines};
use time::{Date, OffsetDateTime, Time, UtcOffset, Weekday, Month, Duration};
use super::components::*;

pub fn parse_ical (mut lines: Lines<BufReader<File>>) -> Result<ICal, Error> {
    let mut line = match lines.next() {
        None => return Err(Error::other("Empty ics file provided")),
        Some(Err(e)) => return Err(e),
        Some(Ok(line)) => line,
    };

    if line.as_str() != "BEGIN:VCALENDAR" { //format!("{}{}", BEGIN, CALENDAR) {
        return Err(Error::other("Improperly formatted ics file provided"));
    }

    let ical = ICal::new();

    while let Some(line) = lines.next() {
        match line?.as_str() {
            "BEGIN:VTIMEZONE" => println!("Parsing timezone"),
            "BEGIN:VEVENT" => println!("Parsing event"),
            "END:VCALENAR" => break,
            _ => {},
        };
    }

    //Not checking for the end of the calendar because I don't care :)

    Ok(ical)
}

fn parse_ical_timezone(mut lines: &Lines<BufReader<File>>) -> Result<TimeZone, String> {
    // TODO

    //     while line does not start with "TZID:": if read line fails, invalid timezone format
    //     read next line
    //
    // the rest of the line starting with "TZID:" is the TimeZone's ID
    //
    // two loops:
    // std = for each line from line reader:
    // match:
    // "BEGIN:DAYLIGHT": false
    // "BEGIN:STANDARD": true
    // _ : read next line
    // if std == true:
    //     (std_offset, dst_end) = parse_ical_offset(file_reader)
    // else:
    // (dst_offset, dst_start) = parse_ical_offset(file_reader)
    //
    // for each line
    // match:
    // EOF: invalid timezone format
    // "END:VTIMEZONE": BREAK

    Err("Not implemented yet".to_owned())
}

fn parse_ical_event(mut lines: &Lines<BufReader<File>>) -> Result<TimeZone, String> {
    Err("Not implemented yet".to_owned())
}

fn parse_ical_offset(mut lines: Lines<BufReader<File>>) -> Result<(UtcOffset, RRule), String> {
    // TODO

    // let mut offset value = None;
    // let mut rrule = None;
    //
    // for each line from the line reader:
    // match:
    // "TZOFFSETTO:": set the offset value
    // "RRULE:": rrule = parse_ical_rrule(file_reader)
    // "END:DAYLIGHT" or "END:STANDARD": BREAK
    // _ : read next line

    Err("not implemented yet".to_owned())
}

fn parse_ical_rrule(mut lines: Lines<BufReader<File>>) -> Result<RRule, String> {
    // TODO

    // freq_start = index of "FREQ="
    // freq_end = index of ";" after "FREQ="
    // let freq_val = slice from freq_start + 5 to freq_end;
    //
    // bymonth_start = index of "BYMONTH="
    // bymonth_end = index of ";" after "BYMONTH="
    // let by_month_val = slice from bymonth_start + 8 to bymonth_end
    //
    // byday_start = index of "BYDAY="
    // byday_end = index of ";" after "BYDAY="
    // let by_day_n_val = slice from byday_start + 6 to byday_start + 7 cast as u8
    // let by_day_weekday_val = slice from byday_start + 7 to byday_start + 9
    // by_day_weekday_val = match by_day_weekday_val {
    //     "SU": Weekday::Sunday,
    //     "SA": Weekday::Saturday,
    //     and so on...
    // }
    //
    // return new RRule {
    //     freq_val = String,
    //     until = None
    //     count = None,
    //     by_month = bymonth,
    //     by_day_n = by_day_n,
    //     by_day_weekday = by_day_weekday_val,
    // }

    Err("Not implemented yet".to_owned())
}

// fn get_tz_offset(date: Date, time: Time, local: bool, tz: TimeZone) -> Result<UtcOffset, time::error::ComponentRange> {
//     let checking_date = OffsetDateTime::new_in_offset(date, time, UtcOffset::from_hms(0, 0, 0)?);
//
//     //Find out which RRule was the most recent
//     let dst_start_datetime = OffsetDateTime::new_in_offset(
//         get_nth_weekday(tz.dst_start.by_day_n, date.year(), tz.dst_start.by_month, tz.dst_start.by_day_weekday),
//         Time::from_hms(2, 0, 0)?,
//         if local {
//             UtcOffset::from_hms(0, 0, 0)?
//         } else {
//             tz.std_offset
//         }
//     );
//
//     if checking_date < dst_start_datetime {
//         return Ok(tz.std_offset)
//     }
//
//     let dst_end_datetime = OffsetDateTime::new_in_offset(
//         get_nth_weekday(tz.dst_end.by_day_n, date.year(), tz.dst_end.by_month, tz.dst_end.by_day_weekday),
//         Time::from_hms(2, 0, 0)?,
//         if local {
//             UtcOffset::from_hms(0, 0, 0)?
//         } else {
//             tz.std_offset
//         }
//     );
//
//     if checking_date < dst_end_datetime {
//         return Ok(tz.dst_offset)
//     }
//
//     Ok(tz.std_offset)
//
//     // TODO
//     //if it's after 2 AM local time
//
//     //Need to hash out the details about how this transition happens at 2 AM
//     //From UTC and from Pacific Time, at both DST transitions
//     //When DST is ending, if the Std time calculated from UTC is 2 AM, the transition has occurred. Put in STD.
//     //When DST is ending, if the local time is 2 AM or later, the transition has occurred. Use STD offset to get UTC.
//     //When DST is starting, if the STD time calculated from UTC is 2 AM or later, make it 3 AM (Put in DST).
//     //When DST is starting, local times between >= 0200 and <0300 will move forward one hour. Before is STD, after is DST.
//     //Get the corresponding offset
// }

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

    let (date, time) = date_time_from_ical(&ical_str[start..start + 15]);

    let offset =
        if local_time {
            UtcOffset::from_hms(0, 0, 0).expect("Error processing offset")
            //TODO Get the TZID and find the matching offset to go with it. Use that.
        } else {
            UtcOffset::from_hms(0, 0, 0).expect("Error processing offset")
        };

    OffsetDateTime::new_in_offset(date, time, offset)
}

fn date_time_from_ical(ical_str: &str) -> (Date, Time) {
    let year = ical_str[0..4].parse::<i32>().expect("Invalid date time format");
    let month = ical_str[4..6].parse::<u8>().expect("Invalid date time format");
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
    let day = ical_str[6..8].parse::<u8>().expect("Invalid date time format");

    let hour = ical_str[9..11].parse::<u8>().expect("Invalid date time format");
    let minute = ical_str[11..13].parse::<u8>().expect("Invalid date time format");
    let second = ical_str[13..15].parse::<u8>().expect("Invalid date time format");

    let date = Date::from_calendar_date(year, month, day).expect("Invalid date values");
    let time = Time::from_hms(hour, minute, second).expect("Invalid time values");

    (date, time)
}

fn get_nth_weekday(n: u8, year: i32, month: Month, wkday: Weekday) -> Date {
    let mut first = Date::from_calendar_date(year, month, 1)
        .expect("Error parsing year and month for nth weekday.");
    let mut i = 0;
    while first.weekday().nth_next(i) != wkday {
        i += 1;
    }

    first.replace_day(i + 1 + (n - 1) * 7).expect("Error calculating day for nth weekday.")
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

    #[test]
    fn nth_weekday() {
        assert_eq!(get_nth_weekday(1, 2024, Month::September, Weekday::Sunday),
                   Date::from_calendar_date(2024, Month::September, 1).unwrap());
        assert_eq!(get_nth_weekday(5, 2024, Month::September, Weekday::Monday),
                   Date::from_calendar_date(2024, Month::September, 30).unwrap());
    }

    #[test]
    fn get_tz_offset_dst() {
        let (date, time) = date_time_from_ical("20240310T020000");
        // let result = get_tz_offset(date, time, true, "America/Los_Angeles");// -> Result<UtcOffset, time::error::ComponentRange>
        todo!()
    }

    #[test]
    fn get_tz_offset_std() {
        todo!()
    }

    #[test]
    fn get_tz_offset_utc_during_dst() {
        todo!()
    }

    #[test]
    fn get_tz_offset_utc_during_std() {
        todo!()
    }


}