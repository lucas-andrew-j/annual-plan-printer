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

    fn set_freq(&mut self, freq: Freq) -> &mut RRuleBuilder {
        todo!();
    }

    fn build(self) -> RRule {
        todo!();
    }
}