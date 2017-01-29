extern crate chrono;
use chrono::*;

const GIGASECOND: i64 = 1_000_000_000;

pub fn after<Tz: TimeZone>(date: DateTime<Tz>) -> DateTime<Tz> {
    date + Duration::seconds(GIGASECOND)
}
