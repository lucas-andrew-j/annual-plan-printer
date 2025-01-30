

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

struct Event {
    // TODO
}