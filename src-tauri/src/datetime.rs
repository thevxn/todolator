use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc, Weekday};

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

pub fn add_workdays(mut date: DateTime<Utc>, mut n: i64) -> DateTime<Utc> {
    while n > 0 {
        date = date + Duration::days(1);
        match date.weekday() {
            Weekday::Sat | Weekday::Sun => continue,
            _ => n -= 1,
        }
    }
    date
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2024, 2), 29); // Leap year
        assert_eq!(days_in_month(2023, 2), 28);
        assert_eq!(days_in_month(2023, 1), 31);
        assert_eq!(days_in_month(2023, 4), 30);
    }

    #[test]
    fn test_add_months_basic() {
        let date = Utc.with_ymd_and_hms(2023, 1, 15, 12, 0, 0).unwrap();
        let result = add_months(date, 1);
        assert_eq!(result, Utc.with_ymd_and_hms(2023, 2, 15, 12, 0, 0).unwrap());
    }

    #[test]
    fn test_add_months_wrap_year() {
        let date = Utc.with_ymd_and_hms(2023, 11, 30, 9, 0, 0).unwrap();
        let result = add_months(date, 2);
        assert_eq!(result, Utc.with_ymd_and_hms(2024, 1, 30, 9, 0, 0).unwrap());
    }

    #[test]
    fn test_add_months_day_adjustment() {
        let date = Utc.with_ymd_and_hms(2023, 1, 31, 8, 0, 0).unwrap();
        let result = add_months(date, 1);
        // February 2023 only has 28 days
        assert_eq!(result, Utc.with_ymd_and_hms(2023, 2, 28, 8, 0, 0).unwrap());
    }

    #[test]
    fn test_add_workdays_simple() {
        let monday = Utc.with_ymd_and_hms(2023, 7, 24, 10, 0, 0).unwrap(); // Monday
        let result = add_workdays(monday, 5);
        assert_eq!(result.weekday(), Weekday::Mon);
        assert_eq!(
            result.date_naive(),
            monday.date_naive() + chrono::Days::new(7)
        );
    }

    #[test]
    fn test_add_workdays_over_weekend() {
        let friday = Utc.with_ymd_and_hms(2023, 7, 21, 10, 0, 0).unwrap(); // Friday
        let result = add_workdays(friday, 1);
        assert_eq!(result.weekday(), Weekday::Mon);
        assert_eq!(
            result.date_naive(),
            friday.date_naive() + chrono::Days::new(3)
        );
    }

    #[test]
    fn test_add_workdays_multiple_weeks() {
        let tuesday = Utc.with_ymd_and_hms(2023, 7, 18, 10, 0, 0).unwrap(); // Tuesday
        let result = add_workdays(tuesday, 10);
        assert_eq!(result.weekday(), Weekday::Tue);
        assert_eq!(
            result.date_naive(),
            tuesday.date_naive() + chrono::Days::new(14)
        );
    }
}
