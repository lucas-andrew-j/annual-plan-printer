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

enum Freq {
    SECONDLY,
    MINUTELY,
    HOURLY,
    DAILY,
    WEEKLY,
    MONTHLY,
    YEARLY,
}

enum DateOrDateTime {
    Date(Date),
    DateTime(OffsetDateTime),
}

pub struct RRuleBuilder {
    freq: Option<Freq>,
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

impl RRuleBuilder {
    fn new() -> RRuleBuilder {
        RRuleBuilder {
            freq: None,
            until: None,
            count: None,
            interval: 1,
            by_second: None,
            by_minute: None,
            by_hour: None,
            by_day: None,
            by_month_day: None,
            by_year_day: None,
            by_week_no: None,
            by_month: None,
            by_set_pos: None,
            week_start: Weekday::Monday,
        }
    }

    fn set_freq(&mut self, freq: &str) -> Result<&mut RRuleBuilder, Error> {
        match freq {
            "SECONDLY" => self.freq = Some(Freq::SECONDLY),
            "MINUTELY" => self.freq = Some(Freq::MINUTELY),
            "HOURLY" => self.freq = Some(Freq::HOURLY),
            "DAILY" => self.freq = Some(Freq::DAILY),
            "WEEKLY" => self.freq = Some(Freq::WEEKLY),
            "MONTHLY" => self.freq = Some(Freq::MONTHLY),
            "YEARLY" => self.freq = Some(Freq::YEARLY),
            _ => return Err(Error::other("Invalid frequency")),
        }

        Ok(self)
    }

    fn set_until(&mut self, until: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_count(&mut self, count: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_interval(&mut self, interval: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_by_second(&mut self, by_second: &str) -> &mut RRuleBuilder { todo!(); }

    fn set_by_minute(&mut self, by_minute: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_by_hour(&mut self, by_hour: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_by_day(&mut self, by_day: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_by_month_day(&mut self, by_month_day: &str) -> &mut RRuleBuilder { todo!(); }

    fn set_by_year_day(&mut self, by_year_day: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_by_week_no(&mut self, by_week_no: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_by_month(&mut self, by_month: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_by_set_pos(&mut self, by_set_pos: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn set_week_start(&mut self, week_start: &str) -> &mut RRuleBuilder {
        todo!();
    }

    fn build(self) -> RRule {
        todo!();
    }
}