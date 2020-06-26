extern crate jpholiday;
use jpholiday::jpholiday::JPHoliday;
use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;

#[test]
fn test_holiday_1971() {
    let jpholiday = JPHoliday::new();
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 1, 1).borrow()).unwrap(), "元日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 1, 15).borrow()).unwrap(), "成人の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 2, 11).borrow()).unwrap(), "建国記念の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 3, 21).borrow()).unwrap(), "春分の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 4, 29).borrow()).unwrap(), "天皇誕生日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 5, 3).borrow()).unwrap(), "憲法記念日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 5, 4).borrow()).unwrap(), "国民の休日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 5, 5).borrow()).unwrap(), "こどもの日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 9, 15).borrow()).unwrap(), "敬老の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 9, 24).borrow()).unwrap(), "秋分の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 10, 10).borrow()).unwrap(), "体育の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 11, 3).borrow()).unwrap(), "文化の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(1971, 11, 23).borrow()).unwrap(), "勤労感謝の日".to_string());
}


#[test]
fn test_count_month_1971() {
    let jpholiday = JPHoliday::new();
    assert_eq!(jpholiday.month_holidays(1971, 1).len(), 2);
    assert_eq!(jpholiday.month_holidays(1971, 2).len(), 1);
    assert_eq!(jpholiday.month_holidays(1971, 3).len(), 1);
    assert_eq!(jpholiday.month_holidays(1971, 4).len(), 1);
    assert_eq!(jpholiday.month_holidays(1971, 5).len(), 3);
    assert_eq!(jpholiday.month_holidays(1971, 6).len(), 0);
    assert_eq!(jpholiday.month_holidays(1971, 7).len(), 0);
    assert_eq!(jpholiday.month_holidays(1971, 8).len(), 0);
    assert_eq!(jpholiday.month_holidays(1971, 9).len(), 2);
    assert_eq!(jpholiday.month_holidays(1971, 10).len(), 1);
    assert_eq!(jpholiday.month_holidays(1971, 11).len(), 2);
    assert_eq!(jpholiday.month_holidays(1971, 12).len(), 0);
}

#[test]
fn test_count_year_1971() {
    let jpholiday = JPHoliday::new();
    assert_eq!(jpholiday.year_holidays(1971).len(), 13);
}

#[test]
fn test_between_1971() {
    let jpholiday = JPHoliday::new();
    assert_eq!(
        jpholiday.between(NaiveDate::from_ymd(1971, 1, 1).borrow(), NaiveDate::from_ymd(1971, 5, 3).borrow()),
        vec![
            (NaiveDate::from_ymd(1971, 1, 1), "元日".to_string()),
            (NaiveDate::from_ymd(1971, 1, 15), "成人の日".to_string()),
            (NaiveDate::from_ymd(1971, 2, 11), "建国記念の日".to_string()),
            (NaiveDate::from_ymd(1971, 3, 21), "春分の日".to_string()),
            (NaiveDate::from_ymd(1971, 4, 29), "天皇誕生日".to_string()),
            (NaiveDate::from_ymd(1971, 5, 3), "憲法記念日".to_string())
        ]
    );
}