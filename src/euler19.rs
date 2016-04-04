use chrono::*;

pub fn main() {
    let mut sundays = 0;
    let mut cursor = UTC.ymd(1901, 1, 1);
    let stop_date = UTC.ymd(2000, 12, 31);
    let duration = Duration::days(1);
    while cursor != stop_date {
        if cursor.weekday() == Weekday::Sun && cursor.day() == 1 {
            sundays += 1;
        }
        cursor = cursor + duration;
    }
    println!("{}", sundays);
}
