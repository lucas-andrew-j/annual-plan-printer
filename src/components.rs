use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday, Month, Duration};

struct IcalContent {
    timezone: TimeZone,
    events: Vec<Event>,
}

struct TimeZone {
    id: String,
    std_offset: UtcOffset,
    dst_offset: UtcOffset,
    dst_start: RRule,
    dst_end: RRule,
}

// BEGIN:VEVENT
// END:VEVENT
struct Event {
    dt_stamp: OffsetDateTime,
    uid: Uid,
    dt_start: DateOrDateTime,
    description: String,
    location: String,
    rrule: Option<Rule>,
    dt_end: DateOrDateTime,
    duration: Duration,
}

// Using explicit values for properties because it is probably smaller than a BTreeMap, and
// definitely faster to access. Can look into the size comparison later.
struct RRule {
    freq: Freq,
    until: DateOrDateTime,
    count: usize,
    interval: Option<>,
    bysecond: Option<usize>,
    byminute: Option<usize>,
    byhour: Option<usize>,
    byday: Option<List<(Weekday, Option<isize>)>>,
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

enum Weekday {
    SU,
    MO,
    TU,
    WE,
    TH,
    FR,
    SA,
}

enum DateOrDateTime {
    Date(Date),
    DateTime(OffsetDateTime),
}