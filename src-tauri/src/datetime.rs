use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};

pub fn add_months(dt: DateTime<Utc>, months: u32) -> DateTime<Utc> {
    let mut year = dt.year();
    let mut month = dt.month() + months;

    while month > 12 {
        month -= 12;
        year += 1;
    }

    let day = std::cmp::min(dt.day(), days_in_month(year, month));
    Utc.with_ymd_and_hms(year, month, day, dt.hour(), dt.minute(), dt.second())
        .unwrap()
}

pub fn days_in_month(year: i32, month: u32) -> u32 {
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    let first_day = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
    let next_first_day = Utc
        .with_ymd_and_hms(next_year, next_month, 1, 0, 0, 0)
        .unwrap();
    (next_first_day - first_day).num_days() as u32
}
