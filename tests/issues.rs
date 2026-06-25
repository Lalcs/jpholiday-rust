//! 本家 `test_issues_38.py` の移植。
//!
//! 本家 Python 版は `calendar.setfirstweekday` のグローバル状態が祝日判定に影響する不具合
//! (<https://github.com/Lalcs/jpholiday/issues/38>) を検証している。Rust 版は曜日計算に
//! グローバル状態を一切用いないため本質的にこの問題は発生しないが、同じ日付で回帰を確認する。

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
