use chrono::NaiveDate;


pub fn string_to_date(date_str: &str) -> NaiveDate {
let datetime = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
datetime
}
pub fn date_to_string(date_time: NaiveDate) -> String {
let formatted = format!("{}", date_time.format("%Y-%m-%d"));
formatted
}


pub fn difference_between_dates(date_time: NaiveDate, date_time_2: NaiveDate) -> i64 {
(date_time - date_time_2).num_days()
}
