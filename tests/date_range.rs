//! 表現可能な日付範囲の端（`i32::MIN` 年 1 月 1 日 / `i32::MAX` 年 12 月 31 日）での挙動の検証。
//!
//! 以前は最大日付の翌日が `i32::MIN` 年へ黙って巻き戻ったため、`between(.., 最大日付)` が
//! 無限ループしていた（`JPHoliday` ではキャッシュも際限なく増えた）。現在は範囲問い合わせが
//! 末尾で確実に止まり、日付演算は巻き戻らずに `None`（`checked_*`）または panic になる。

use jpholiday::{Date, JPHoliday};

fn max() -> Date {
    Date::new(i32::MAX, 12, 31).unwrap()
}

fn min() -> Date {
    Date::new(i32::MIN, 1, 1).unwrap()
}

#[test]
fn checked_arithmetic_at_the_edges() {
    assert_eq!(max().checked_succ(), None);
    assert_eq!(min().checked_pred(), None);
    assert_eq!(
        max().checked_pred(),
        Some(Date::new(i32::MAX, 12, 30).unwrap())
    );
    assert_eq!(
        min().checked_succ(),
        Some(Date::new(i32::MIN, 1, 2).unwrap())
    );

    let d = Date::new(2020, 1, 1).unwrap();
    assert_eq!(d.checked_add_days(i64::MAX), None);
    assert_eq!(d.checked_add_days(i64::MIN), None);
    assert_eq!(
        d.checked_add_days(366),
        Some(Date::new(2021, 1, 1).unwrap())
    );
    assert_eq!(
        d.checked_add_days(-1),
        Some(Date::new(2019, 12, 31).unwrap())
    );
}

#[test]
fn checked_from_ordinal_round_trips_at_the_edges() {
    assert_eq!(Date::checked_from_ordinal(max().to_ordinal()), Some(max()));
    assert_eq!(Date::checked_from_ordinal(min().to_ordinal()), Some(min()));
    assert_eq!(Date::checked_from_ordinal(max().to_ordinal() + 1), None);
    assert_eq!(Date::checked_from_ordinal(min().to_ordinal() - 1), None);
    assert_eq!(Date::checked_from_ordinal(i64::MAX), None);
    assert_eq!(Date::checked_from_ordinal(i64::MIN), None);
}

#[test]
#[should_panic(expected = "out of the representable date range")]
fn succ_past_the_max_date_panics_instead_of_wrapping() {
    let _ = max().succ();
}

#[test]
#[should_panic(expected = "out of the representable date range")]
fn pred_before_the_min_date_panics_instead_of_wrapping() {
    let _ = min().pred();
}

#[test]
#[should_panic(expected = "out of the representable date range")]
fn add_days_overflow_panics_in_every_build() {
    let _ = Date::new(2020, 1, 1).unwrap().add_days(i64::MAX);
}

#[test]
#[should_panic(expected = "out of the representable date range")]
fn from_ordinal_out_of_range_panics_in_every_build() {
    let _ = Date::from_ordinal(i64::MIN);
}

#[test]
fn range_queries_terminate_at_the_max_date() {
    let december = Date::new(i32::MAX, 12, 1).unwrap();
    assert!(jpholiday::between(max(), max()).is_empty());
    assert!(jpholiday::between(december, max()).is_empty());
    assert!(jpholiday::month_holidays(i32::MAX, 12).is_empty());

    let jp = JPHoliday::new();
    assert!(jp.between(max(), max()).is_empty());
    assert!(jp.between(december, max()).is_empty());
    assert!(jp.month_holidays(i32::MAX, 12).is_empty());
}

#[test]
fn single_day_queries_do_not_panic_at_the_edges() {
    for date in [min(), max()] {
        assert!(!jpholiday::is_holiday(date));
        assert!(!JPHoliday::new().is_holiday(date));
    }
    assert!(jpholiday::year_holidays(i32::MIN).is_empty());
    assert!(JPHoliday::new().year_holidays(i32::MIN).is_empty());
}

#[test]
fn reversed_range_is_empty() {
    let a = Date::new(2020, 1, 1).unwrap();
    let b = Date::new(2020, 12, 31).unwrap();
    assert!(jpholiday::between(b, a).is_empty());
    assert!(JPHoliday::new().between(b, a).is_empty());
    assert_eq!(jpholiday::between(a, a), vec![(a, "元日".to_string())]);
}
