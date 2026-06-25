//! 本家 `test_basic.py::test_original_holiday` の移植。
//!
//! 独自祝日の登録・解除・再登録と、独自祝日が「国民の休日」「振替休日」を発生させない
//! （対象から除外される）ことを検証する。テスト分離のためインスタンス API を用いる。

use jpholiday::{Date, JPHoliday, OriginalHolidayChecker};

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

struct SpecialLeave;

impl OriginalHolidayChecker for SpecialLeave {
    fn is_holiday(&self, date: Date) -> bool {
        date == d(2020, 2, 3) || date == d(2020, 2, 5) || date == d(2020, 2, 9)
    }

    fn holiday_name(&self, _date: Date) -> String {
        "特別休暇".to_string()
    }
}

fn assert_registered(jp: &JPHoliday) {
    // 2/3 と 2/5 は独自祝日。
    assert_eq!(
        jp.is_holiday_name(d(2020, 2, 3)).as_deref(),
        Some("特別休暇")
    );
    assert_eq!(
        jp.is_holiday_name(d(2020, 2, 5)).as_deref(),
        Some("特別休暇")
    );
    // 2/4 は独自祝日に挟まれるが「国民の休日」にはならない（独自祝日は対象外）。
    assert!(!jp.is_holiday(d(2020, 2, 4)));
    // 2/9 (日曜) は独自祝日だが、翌 2/10 は「振替休日」にならない（独自祝日は対象外）。
    assert_eq!(
        jp.is_holiday_name(d(2020, 2, 9)).as_deref(),
        Some("特別休暇")
    );
    assert!(!jp.is_holiday(d(2020, 2, 10)));
}

#[test]
fn register_unregister_reregister() {
    let mut jp = JPHoliday::new();

    jp.register(SpecialLeave);
    assert_registered(&jp);

    // 登録解除すると独自祝日は消える。
    jp.unregister::<SpecialLeave>();
    assert!(!jp.is_holiday(d(2020, 2, 3)));
    assert!(!jp.is_holiday(d(2020, 2, 5)));
    assert!(!jp.is_holiday(d(2020, 2, 9)));

    // 再登録しても同じ結果。
    jp.register(SpecialLeave);
    assert_registered(&jp);

    jp.unregister::<SpecialLeave>();
}

#[test]
fn duplicate_registration_is_ignored() {
    let mut jp = JPHoliday::new();
    jp.register(SpecialLeave);
    jp.register(SpecialLeave); // 同一型の重複登録は無視される。

    // 重複しても祝日名は 1 つ（先頭）だけ。
    assert_eq!(jp.holidays(d(2020, 2, 3)).len(), 1);
}
