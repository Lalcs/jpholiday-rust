//! 関数 API（グローバル singleton）の `register` / `unregister` を検証する。
//!
//! グローバル状態を変更するため、競合を避けてこのバイナリ内のテストは 1 つだけにしている。

use jpholiday::{Date, OriginalHolidayChecker};

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

struct CompanyHoliday;

impl OriginalHolidayChecker for CompanyHoliday {
    fn is_holiday(&self, date: Date) -> bool {
        date == d(2020, 2, 9)
    }
    fn holiday_name(&self, _date: Date) -> String {
        "特別休暇".to_string()
    }
}

#[test]
fn global_register_then_unregister() {
    assert!(!jpholiday::is_holiday(d(2020, 2, 9)));

    jpholiday::register(CompanyHoliday);
    assert!(jpholiday::is_holiday(d(2020, 2, 9)));
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 2, 9)).as_deref(),
        Some("特別休暇")
    );

    jpholiday::unregister::<CompanyHoliday>();
    assert!(!jpholiday::is_holiday(d(2020, 2, 9)));
    assert_eq!(jpholiday::is_holiday_name(d(2020, 2, 9)), None);
}
