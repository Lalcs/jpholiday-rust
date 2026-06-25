//! グローバル関数 API のロックが、独自チェッカーの panic によって恒久的に壊れない
//! （poison から回復する）ことを検証する回帰テスト。
//!
//! グローバル状態を panic で汚すため、競合を避けてこのバイナリ内のテストは 1 つだけにしている。

use jpholiday::{Date, OriginalHolidayChecker};
use std::panic::{AssertUnwindSafe, catch_unwind};

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

/// 特定日でのみ panic する独自チェッカー。
struct PanicOnce;

impl OriginalHolidayChecker for PanicOnce {
    fn is_holiday(&self, date: Date) -> bool {
        if date == d(2099, 1, 2) {
            panic!("intentional panic from a custom checker");
        }
        false
    }
    fn holiday_name(&self, _date: Date) -> String {
        "x".to_string()
    }
}

#[test]
fn global_api_survives_a_panicking_checker() {
    jpholiday::register(PanicOnce);

    // チェッカーが panic する呼び出し。panic はこの 1 回の呼び出しで捕捉される。
    let panicked = catch_unwind(AssertUnwindSafe(|| jpholiday::is_holiday(d(2099, 1, 2))));
    assert!(
        panicked.is_err(),
        "the panicking checker should propagate for this one call"
    );

    // poison から回復し、以降の呼び出しは正常に動作し続ける（panic は一時的失敗にとどまる）。
    assert!(jpholiday::is_holiday(d(2017, 1, 1))); // 元日
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 1, 1)).as_deref(),
        Some("元日")
    );
    assert!(!jpholiday::is_holiday(d(2017, 1, 3)));

    jpholiday::unregister::<PanicOnce>();
    assert!(jpholiday::is_holiday(d(2017, 1, 1)));
}
