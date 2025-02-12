use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday, Month};

struct ical_content {
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
    dt_start: OffsetDateTime,
    description: String,
    location: String,
    rrule: Option<Rule>,
    event_end: EvendEnd,
}

enum EventEnd {
    DtEnd(OffsetDateTime),
    Duration(Time),
}

struct Rule {
    // TODO
}