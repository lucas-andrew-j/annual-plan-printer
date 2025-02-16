use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday, Month, Duration};

pub struct ICal {
    timezone: TimeZone,
    events: Vec<Event>,
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
    until: DateOrDateTime,
    count: usize,
    interval: Option<usize>,
    bysecond: Option<usize>,
    byminute: Option<usize>,
    byhour: Option<usize>,
    byday: Option<Vec<(Weekday, Option<isize>)>>,
    bymonthday: Option<isize>,
    byyearday: Option<isize>,
    byweekno: Option<isize>,
    bymonth: Option<usize>,
    bysetpos: Option<isize>,
    wkst: Option<Weekday>,
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