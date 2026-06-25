//! 本家 `test_exception.py` に対応する移植。
//!
//! 本家は `date`/`datetime` 以外の型を渡したときの `JPHolidayTypeError` を検証しているが、
//! Rust ではそれが型システムによりコンパイル時に排除される。代わりに、実在しない日付を
//! 生成しようとしたときに [`DateError`] が返ることを検証する。

use jpholiday::{Date, DateError};

#[test]
fn invalid_dates_return_error() {
    assert!(matches!(
        Date::new(2021, 2, 29),
        Err(DateError::InvalidDate { .. })
    ));
    assert!(matches!(
        Date::new(2021, 2, 30),
        Err(DateError::InvalidDate { .. })
    ));
    assert!(matches!(
        Date::new(2021, 13, 1),
        Err(DateError::InvalidDate { .. })
    ));
    assert!(matches!(
        Date::new(2021, 0, 1),
        Err(DateError::InvalidDate { .. })
    ));
    assert!(matches!(
        Date::new(2021, 1, 0),
        Err(DateError::InvalidDate { .. })
    ));
}

#[test]
fn valid_dates_are_ok() {
    assert!(Date::new(2021, 1, 1).is_ok());
    assert!(Date::new(2024, 2, 29).is_ok()); // 閏年
    assert!(Date::new(2021, 12, 31).is_ok());
}

#[test]
fn error_is_displayable() {
    let err = Date::new(2021, 2, 30).unwrap_err();
    assert_eq!(err.to_string(), "invalid date: 2021-02-30");
}
