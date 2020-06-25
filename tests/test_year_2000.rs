extern crate jpholiday;
use jpholiday::jpholiday::JPHoliday;
use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;

#[test]
fn test_holiday_2000() {
    let jpholiday = JPHoliday::new();
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 1, 1).borrow()).unwrap(), "元日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 1, 10).borrow()).unwrap(), "成人の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 2, 11).borrow()).unwrap(), "建国記念の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 3, 20).borrow()).unwrap(), "春分の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 4, 29).borrow()).unwrap(), "みどりの日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 5, 3).borrow()).unwrap(), "憲法記念日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 5, 4).borrow()).unwrap(), "国民の休日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 5, 5).borrow()).unwrap(), "こどもの日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 7, 20).borrow()).unwrap(), "海の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 9, 15).borrow()).unwrap(), "敬老の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 9, 23).borrow()).unwrap(), "秋分の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 10, 9).borrow()).unwrap(), "体育の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 11, 3).borrow()).unwrap(), "文化の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 11, 23).borrow()).unwrap(), "勤労感謝の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2000, 12, 23).borrow()).unwrap(), "天皇誕生日".to_string());
}


#[test]
fn test_count_month_2000() {
    let jpholiday = JPHoliday::new();
    assert_eq!(jpholiday.year_holidays(2000).len(), 15);
}

#[test]
fn test_count_year_2000() {
    let jpholiday = JPHoliday::new();
    assert_eq!(jpholiday.year_holidays(2000).len(), 15);
}
