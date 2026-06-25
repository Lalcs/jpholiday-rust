//! 日付による祝日判定の検証。
//!
//! 本ライブラリは日付専用の [`Date`] のみを扱う（時刻成分を持たない）ため、各日付が
//! 時刻に左右されず一貫して判定されることを確認する。

use jpholiday::Date;

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

#[test]
fn sea_day() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 7, 23)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 7, 22)).as_deref(),
        Some("海の日")
    );
    assert!(jpholiday::is_holiday(d(2020, 7, 23)));
    assert!(jpholiday::is_holiday(d(2021, 7, 22)));
    assert_eq!(
        jpholiday::between(d(2020, 7, 23), d(2020, 7, 23)),
        vec![(d(2020, 7, 23), "海の日".to_string())]
    );
    assert_eq!(
        jpholiday::between(d(2021, 7, 22), d(2021, 7, 22)),
        vec![(d(2021, 7, 22), "海の日".to_string())]
    );
}

#[test]
fn mountain_day() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 8, 10)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 8, 8)).as_deref(),
        Some("山の日")
    );
    assert!(jpholiday::is_holiday(d(2020, 8, 10)));
    assert!(jpholiday::is_holiday(d(2021, 8, 8)));
    assert_eq!(
        jpholiday::between(d(2020, 8, 10), d(2020, 8, 10)),
        vec![(d(2020, 8, 10), "山の日".to_string())]
    );
    assert_eq!(
        jpholiday::between(d(2021, 8, 8), d(2021, 8, 8)),
        vec![(d(2021, 8, 8), "山の日".to_string())]
    );
}

#[test]
fn sports_day() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 7, 24)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 7, 23)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::between(d(2020, 7, 24), d(2020, 7, 24)),
        vec![(d(2020, 7, 24), "スポーツの日".to_string())]
    );
    assert_eq!(
        jpholiday::between(d(2021, 7, 23), d(2021, 7, 23)),
        vec![(d(2021, 7, 23), "スポーツの日".to_string())]
    );
}

#[test]
fn imperial_holidays() {
    let cases = [
        (d(1959, 4, 10), "皇太子・明仁親王の結婚の儀"),
        (d(1989, 2, 24), "昭和天皇の大喪の礼"),
        (d(1990, 11, 12), "即位の礼正殿の儀"),
        (d(1993, 6, 9), "皇太子・皇太子徳仁親王の結婚の儀"),
        (d(2019, 5, 1), "天皇の即位の日"),
        (d(2019, 10, 22), "即位礼正殿の儀"),
    ];
    for (date, name) in cases {
        assert_eq!(jpholiday::is_holiday_name(date).as_deref(), Some(name));
        assert!(jpholiday::is_holiday(date));
        assert_eq!(
            jpholiday::between(date, date),
            vec![(date, name.to_string())]
        );
    }
}
