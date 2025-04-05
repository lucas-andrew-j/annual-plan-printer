use std::io::Error;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday, Month, Duration};

pub struct ICal {
    //timezone: TimeZone, TODO
    events: Vec<Event>,
}

impl ICal {
    pub fn new() -> ICal {
        ICal { events: Vec::new() }
    }
}

pub struct TimeZone {
    id: String,
    std_offset: UtcOffset,
    dst_offset: UtcOffset,
    dst_start: RRule,
    dst_end: RRule,
}

// BEGIN:VEVENT
// END:VEVENT
pub struct Event {
    dt_stamp: OffsetDateTime,
    uid: String,
    dt_start: DateOrDateTime,
    description: String,
    location: String,
    rrule: Option<RRule>,
    dt_end: DateOrDateTime,
    duration: Duration,
}

// Using explicit values for properties because it is probably smaller than a BTreeMap, and
// definitely faster to access. Can look into the size comparison later.
pub struct RRule {
    freq: Freq,
    until: Option<DateOrDateTime>,
    count: Option<usize>,
    interval: usize,
    by_second: Option<Vec<usize>>,
    by_minute: Option<Vec<usize>>,
    by_hour: Option<Vec<usize>>,
    by_day: Option<Vec<(Weekday, Option<isize>)>>,
    by_month_day: Option<Vec<isize>>,
    by_year_day: Option<Vec<isize>>,
    by_week_no: Option<Vec<isize>>,
    by_month: Option<Vec<usize>>,
    by_set_pos: Option<Vec<isize>>,
    week_start: Weekday,
}

#[derive(PartialEq)]
pub enum Freq {
    SECONDLY,
    MINUTELY,
    HOURLY,
    DAILY,
    WEEKLY,
    MONTHLY,
    YEARLY,
}

pub enum DateOrDateTime {
    Date(Date),
    DateTime(OffsetDateTime),
}

pub struct RRuleBuilder {
    freq: Option<Freq>,
    until: Option<DateOrDateTime>,
    count: Option<usize>,
    interval: Option<usize>,
    by_second: Option<Vec<usize>>,
    by_minute: Option<Vec<usize>>,
    by_hour: Option<Vec<usize>>,
    by_day: Option<Vec<(Weekday, Option<isize>)>>,
    by_month_day: Option<Vec<isize>>,
    by_year_day: Option<Vec<isize>>,
    by_week_no: Option<Vec<isize>>,
    by_month: Option<Vec<usize>>,
    by_set_pos: Option<Vec<isize>>,
    week_start: Option<Weekday>,
}

impl RRuleBuilder {
    pub fn new() -> RRuleBuilder {
        RRuleBuilder {
            freq: None,
            until: None,
            count: None,
            interval: None,
            by_second: None,
            by_minute: None,
            by_hour: None,
            by_day: None,
            by_month_day: None,
            by_year_day: None,
            by_week_no: None,
            by_month: None,
            by_set_pos: None,
            week_start: None,
        }
    }

    pub fn set_freq(&mut self, freq: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.freq.is_some() {
            return Err(Error::other("freq cannot be specified more than once"));
        }

        match freq {
            "SECONDLY" => self.freq = Some(crate::components::Freq::SECONDLY),
            "MINUTELY" => self.freq = Some(crate::components::Freq::MINUTELY),
            "HOURLY" => self.freq = Some(crate::components::Freq::HOURLY),
            "DAILY" => self.freq = Some(crate::components::Freq::DAILY),
            "WEEKLY" => self.freq = Some(crate::components::Freq::WEEKLY),
            "MONTHLY" => self.freq = Some(crate::components::Freq::MONTHLY),
            "YEARLY" => self.freq = Some(crate::components::Freq::YEARLY),
            _ => return Err(Error::other("invalid frequency")),
        }

        Ok(self)
    }

    // - UNTIL must be UTC if this is a sub-component of a STANDARD or DAYLIGHT component
    pub fn set_until(&mut self, until: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.until.is_some() {
            return Err(Error::other("until cannot be specified more than once"));
        }

        todo!()
    }

    pub fn set_count(&mut self, count: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.count.is_some() {
            return Err(Error::other("count cannot be specified more than once"));
        }

        todo!()
    }

    pub fn set_interval(&mut self, interval: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.interval.is_some() {
            return Err(Error::other("interval cannot be specified more than once"));
        }

        let interval = interval.parse::<usize>();

        match interval {
            Err(_) => return Err(Error::other("invalid interval")),
            Ok(interval) => self.interval = Some(interval),
        }

        Ok(self)
    }

    pub fn set_by_second(&mut self, by_second: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.by_second.is_some() {
            return Err(Error::other("by_second cannot be specified more than once"));
        }

        todo!()
    }

    // - BYMINUTE values should be 0-59
    pub fn set_by_minute(&mut self, by_minute: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.by_minute.is_some() {
            return Err(Error::other("by_minute cannot be specified more than once"));
        }

        todo!()
    }

    // - BYHOUR values should be 0-23
    pub fn set_by_hour(&mut self, by_hour: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.by_hour.is_some() {
            return Err(Error::other("by_hour cannot be specified more than once"));
        }

        todo!()
    }

    // - BYDAY values should be "SU", "MO", "TU", "WE", "TH", "FR", "SA"
    //   with an optional positive or negative integer
    pub fn set_by_day(&mut self, by_day: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.by_day.is_some() {
            return Err(Error::other("by_day cannot be specified more than once"));
        }

        todo!()
    }

    // - BYMONTHDAY values must be -31 to -1 or 1 to 31
    pub fn set_by_month_day(&mut self, by_month_day: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.by_month_day.is_some() {
            return Err(Error::other("by_month_day cannot be specified more than once"));
        }

        todo!()
    }

    // - BYYEARDAY values must be -366 to -1 or 1 to 366
    pub fn set_by_year_day(&mut self, by_year_day: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.by_year_day.is_some() {
            return Err(Error::other("by_year_day cannot be specified more than once"));
        }

        todo!()
    }

    // - BYWEEKNO values must be -53 to -1 or 1 to 53
    pub fn set_by_week_no(&mut self, by_week_no: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.by_week_no.is_some() {
            return Err(Error::other("by_week_no cannot be specified more than once"));
        }

        todo!()
    }

    // - BYMONTH values must be 1 to 12
    pub fn set_by_month(&mut self, by_month: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.by_month.is_some() {
            return Err(Error::other("by_month cannot be specified more than once"));
        }

        todo!()
    }

    // - BYSETPOS value must be -366 to -1 OR 1 TO 366
    pub fn set_by_set_pos(&mut self, by_set_pos: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.by_set_pos.is_some() {
            return Err(Error::other("by_set_pos cannot be specified more than once"));
        }

        todo!()
    }

    // - WKST values should be "SU", "MO", "TU", "WE", "TH", "FR", "SA"
    pub fn set_week_start(&mut self, week_start: &str) -> Result<&mut RRuleBuilder, Error> {
        if self.week_start.is_some() {
            return Err(Error::other("week_start cannot be specified more than once"));
        }

        todo!()
    }

    // - DTSTART and UNTIL must have matching data types (both date or both date-time)
    //     - If they are date-time, they must both use local time, or DTSTART can be UTC with TZ reference while UNTIL is UTC
    // - BYSECOND, BYMINUTE, AND BYHOUR cannot be specified when DTSTART is a DATE value type
    //     - If this occurs (can happen from old ical versions), delete them
    // - BYDAY must not be specified with a numeric value when:
    //     - FREQ is not monthly or yearly
    //     - FREQ is yearly and BYWEEKNO is specified
    // - BYMONTHDAY must not be specified when FREQ is weekly
    // - BYYEARDAY must not be specified when FREQ is daily, weekly, or monthly
    // - BYWEEKNO must not be specified when FREQ is anything other than yearly
    // - BYSETPOS must be used with another BYxxx rule
    pub fn build(self) -> Result<RRule, Error> {
        let Some(freq) = self.freq else {
            return Err(Error::other("freq must be set for rrule"));
        };

        let interval = self.interval.unwrap_or(1);
        let week_start = self.week_start.unwrap_or(Weekday::Monday);

        let by_day = if freq != Freq::MONTHLY && freq != Freq::YEARLY || (freq == Freq::YEARLY && self.by_week_no.is_some()) {
            if let Some(inner_by_day) = self.by_day {
                if inner_by_day.iter().any(|(_, number)| number.is_some()) {
                    return Err(Error::other("by_day cannot have numeric values when freq is not monthly or yearly, or is yearly with by_week_no"));
                } else {
                    Some(inner_by_day)
                }
            } else {
                None
            }
        } else {
            None
        };

        if freq == Freq::WEEKLY && self.by_month_day.is_some() {
            return Err(Error::other("frequency cannot be weekly with by_month_day"));
        }

        if (freq == Freq::DAILY || freq == Freq::WEEKLY || freq == Freq::MONTHLY) && self.by_year_day.is_some() {
            return Err(Error::other("frequency cannot be daily, weekly, or monthly with by_year_day"));
        }

        if freq != Freq::YEARLY && self.by_week_no.is_some() {
            return Err(Error::other("by_week_no cannot be specified when freq is not yearly"));
        }

        if self.by_set_pos.is_some()
            && self.by_second.is_none()
            && self.by_minute.is_none()
            && self.by_hour.is_none()
            && by_day.is_none()
            && self.by_month_day.is_none()
            && self.by_year_day.is_none()
            && self.by_week_no.is_none()
            && self.by_month.is_none() {
            return Err(Error::other("by_set_pos must be used with another by_XXXX property"));
        }

        Ok(RRule {
            freq: freq,
            until: self.until,
            count: self.count,
            interval: interval,
            by_second: self.by_second,
            by_minute: self.by_minute,
            by_hour: self.by_hour,
            by_day: by_day,
            by_month_day: self.by_month_day,
            by_year_day: self.by_year_day,
            by_week_no: self.by_week_no,
            by_month: self.by_month,
            by_set_pos: self.by_set_pos,
            week_start: week_start,
        })
    }
}