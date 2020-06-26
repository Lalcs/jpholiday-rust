extern crate jpholiday;
use jpholiday::jpholiday::JPHoliday;
use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;

#[test]
fn test_holiday_2020() {
    let jpholiday = JPHoliday::new();
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 1, 1).borrow()).unwrap(), "元日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 1, 13).borrow()).unwrap(), "成人の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 2, 11).borrow()).unwrap(), "建国記念の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 2, 23).borrow()).unwrap(), "天皇誕生日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 2, 24).borrow()).unwrap(), "天皇誕生日 振替休日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 3, 20).borrow()).unwrap(), "春分の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 4, 29).borrow()).unwrap(), "昭和の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 5, 3).borrow()).unwrap(), "憲法記念日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 5, 4).borrow()).unwrap(), "みどりの日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 5, 5).borrow()).unwrap(), "こどもの日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 5, 6).borrow()).unwrap(), "憲法記念日 振替休日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 7, 23).borrow()).unwrap(), "海の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 7, 24).borrow()).unwrap(), "スポーツの日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 8, 10).borrow()).unwrap(), "山の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 9, 21).borrow()).unwrap(), "敬老の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 9, 22).borrow()).unwrap(), "秋分の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 11, 3).borrow()).unwrap(), "文化の日".to_string());
    assert_eq!(jpholiday.is_holiday_name(NaiveDate::from_ymd(2020, 11, 23).borrow()).unwrap(), "勤労感謝の日".to_string());
}


#[test]
fn test_count_month_2020() {
    let jpholiday = JPHoliday::new();
    assert_eq!(jpholiday.month_holidays(2020, 1).len(), 2);
    assert_eq!(jpholiday.month_holidays(2020, 2).len(), 3);
    assert_eq!(jpholiday.month_holidays(2020, 3).len(), 1);
    assert_eq!(jpholiday.month_holidays(2020, 4).len(), 1);
    assert_eq!(jpholiday.month_holidays(2020, 5).len(), 4);
    assert_eq!(jpholiday.month_holidays(2020, 6).len(), 0);
    assert_eq!(jpholiday.month_holidays(2020, 7).len(), 2);
    assert_eq!(jpholiday.month_holidays(2020, 8).len(), 1);
    assert_eq!(jpholiday.month_holidays(2020, 9).len(), 2);
    assert_eq!(jpholiday.month_holidays(2020, 10).len(), 0);
    assert_eq!(jpholiday.month_holidays(2020, 11).len(), 2);
    assert_eq!(jpholiday.month_holidays(2020, 12).len(), 0);
}

#[test]
fn test_count_year_2020() {
    let jpholiday = JPHoliday::new();
    assert_eq!(jpholiday.year_holidays(2020).len(), 18);
}

#[test]
fn test_between_2020() {
    let jpholiday = JPHoliday::new();
    assert_eq!(
        jpholiday.between(NaiveDate::from_ymd(2020, 1, 1).borrow(), NaiveDate::from_ymd(2020, 5, 3).borrow()),
        vec![
            (NaiveDate::from_ymd(2020, 1, 1), "元日".to_string()),
            (NaiveDate::from_ymd(2020, 1, 13), "成人の日".to_string()),
            (NaiveDate::from_ymd(2020, 2, 11), "建国記念の日".to_string()),
            (NaiveDate::from_ymd(2020, 2, 23), "天皇誕生日".to_string()),
            (NaiveDate::from_ymd(2020, 2, 24), "天皇誕生日 振替休日".to_string()),
            (NaiveDate::from_ymd(2020, 3, 20), "春分の日".to_string()),
            (NaiveDate::from_ymd(2020, 4, 29), "昭和の日".to_string()),
            (NaiveDate::from_ymd(2020, 5, 3), "憲法記念日".to_string())
        ]
    );
}