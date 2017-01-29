extern crate chrono;
use chrono::*;

const GIGASECOND: i64 = 1_000_000_000;

pub fn after(date: DateTime<UTC>) -> DateTime<UTC> {
    let dur = Duration::seconds(GIGASECOND);

    date.checked_add(dur).expect("Invalid duration")
}
