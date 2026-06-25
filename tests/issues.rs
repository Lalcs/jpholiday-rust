//! 曜日計算がグローバル状態に依存しないことの回帰テスト。
//!
//! 曜日計算にグローバル状態を一切用いないため、暦の設定によって祝日判定が変化することはない。
//! 過去に同種の不具合が報告された日付で回帰を確認する。

use jpholiday::Date;

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

#[test]
fn issue_38_weekday_state_independence() {
    assert!(jpholiday::is_holiday(d(2025, 9, 15))); // 敬老の日
    assert!(!jpholiday::is_holiday(d(2025, 9, 16)));
    assert!(!jpholiday::is_holiday(d(2025, 9, 17)));
    assert!(!jpholiday::is_holiday(d(2025, 9, 18)));
    assert!(!jpholiday::is_holiday(d(2025, 9, 19)));
    assert!(!jpholiday::is_holiday(d(2025, 9, 20)));
    assert!(!jpholiday::is_holiday(d(2025, 9, 21)));
    assert!(!jpholiday::is_holiday(d(2025, 9, 22)));
    assert!(jpholiday::is_holiday(d(2025, 9, 23))); // 秋分の日
    assert!(!jpholiday::is_holiday(d(2025, 9, 24)));
}
