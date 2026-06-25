//! 本家 `test_basic.py` のうち、基本判定と皇室慶弔行事に伴う祝日の移植。

use jpholiday::Date;

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

#[test]
fn non_holiday() {
    assert_eq!(jpholiday::is_holiday_name(d(2020, 2, 3)), None);
    assert!(!jpholiday::is_holiday(d(2020, 2, 3)));
}

#[test]
fn imperial_holidays() {
    assert_eq!(
        jpholiday::is_holiday_name(d(1959, 4, 10)).as_deref(),
        Some("皇太子・明仁親王の結婚の儀")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 2, 24)).as_deref(),
        Some("昭和天皇の大喪の礼")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1990, 11, 12)).as_deref(),
        Some("即位の礼正殿の儀")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1993, 6, 9)).as_deref(),
        Some("皇太子・皇太子徳仁親王の結婚の儀")
    );
}
