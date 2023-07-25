use super::*;

const HOURS_PER_DAY: u64 = 24;
const MINS_PER_HOUR: u64 = 60;
const SECS_PER_MIN: u64 = 60;
const SECS_PER_HOUR: u64 = SECS_PER_MIN * MINS_PER_HOUR;
const SECS_PER_DAY: u64 = SECS_PER_HOUR * HOURS_PER_DAY;
const MOSCOW_TIMESTAMP_CORRECTION_SECS: u64 = 3 * SECS_PER_HOUR;
const SPREADSHEETS_UNIX_TIMESTAMP_BASE: u64 = 25569;

pub fn spreahsheet_number_value_to_datetime(number_value: f64) -> Option<DateTime<Utc>> {
    let timestamp = ((number_value - SPREADSHEETS_UNIX_TIMESTAMP_BASE as f64) * SECS_PER_DAY as f64)
        as i64
        - MOSCOW_TIMESTAMP_CORRECTION_SECS as i64;
    Utc.from_local_datetime(&NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap())
        .single()
}

pub fn spreahsheet_number_value_to_naive_date(number_value: f64) -> NaiveDate {
    let timestamp =
        ((number_value - SPREADSHEETS_UNIX_TIMESTAMP_BASE as f64) * SECS_PER_DAY as f64) as i64;
    let naive_date_time = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    use chrono::Datelike;
    NaiveDate::from_ymd_opt(
        naive_date_time.year(),
        naive_date_time.month(),
        naive_date_time.day(),
    )
    .unwrap()
}

pub fn datetime_to_spreadsheet_number_value(datetime: DateTime<Utc>) -> f64 {
    let timestamp = datetime.timestamp();
    (timestamp + MOSCOW_TIMESTAMP_CORRECTION_SECS as i64) as f64 / SECS_PER_DAY as f64
        + SPREADSHEETS_UNIX_TIMESTAMP_BASE as f64
}

pub fn spreahsheet_number_value_to_naive_time(number_value: f64) -> NaiveTime {
    NaiveTime::from_num_seconds_from_midnight_opt((number_value * 86400f64 + 0.5f64) as u32, 0)
        .unwrap()
}

pub fn spreahsheet_number_value_to_duration(number_value: f64) -> std::time::Duration {
    std::time::Duration::from_secs((number_value * 86400f64 + 0.5f64) as u64)
}
