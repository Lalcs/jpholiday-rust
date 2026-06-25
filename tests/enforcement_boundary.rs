//! 祝日法（昭和23年法律第178号）の施行日境界（1948-07-20）の検証。
//!
//! 祝日法に忠実に施行日以降のみを国民の祝日とし、施行日より前の年・日付には国民の祝日を
//! 返さないことを検証する。ただし、利用者が登録する独自祝日（[`OriginalHolidayChecker`]）は
//! この境界の対象外。

use jpholiday::{Date, JPHoliday, OriginalHolidayChecker};

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

#[test]
fn no_builtin_holidays_before_enforcement() {
    // 法律施行(1948-07-20)より前には国民の祝日は存在しない。
    assert!(!jpholiday::is_holiday(d(1900, 11, 3))); // 文化の日（施行前）
    assert!(!jpholiday::is_holiday(d(1947, 1, 1))); // 元日（施行前年）
    assert!(!jpholiday::is_holiday(d(1948, 1, 1))); // 1948年だが施行前
    assert!(!jpholiday::is_holiday(d(1948, 5, 3))); // 憲法記念日（施行前）
    assert!(!jpholiday::is_holiday(d(1948, 7, 19))); // 施行前日
    assert_eq!(jpholiday::is_holiday_name(d(1947, 1, 1)), None);
    assert_eq!(jpholiday::year_holidays(1947).len(), 0);
    assert_eq!(jpholiday::year_holidays(1900).len(), 0);
}

#[test]
fn holidays_on_and_after_enforcement() {
    // 施行日以降の1948年は 秋分の日・文化の日・勤労感謝の日 の3つのみ。
    assert_eq!(
        jpholiday::is_holiday_name(d(1948, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1948, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1948, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::year_holidays(1948).len(), 3);

    // 1949年は完全な年（施行日境界の影響を受けない）。
    assert_eq!(
        jpholiday::is_holiday_name(d(1949, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1949, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
}

#[test]
fn custom_holidays_are_exempt_from_enforcement_boundary() {
    struct FoundingAnniversary;
    impl OriginalHolidayChecker for FoundingAnniversary {
        fn is_holiday(&self, date: Date) -> bool {
            date == d(1900, 1, 1)
        }
        fn holiday_name(&self, _date: Date) -> String {
            "創業記念日".to_string()
        }
    }

    let mut jp = JPHoliday::new();
    jp.register(FoundingAnniversary);

    // 施行日より前でも独自祝日は有効（祝日法の境界は独自祝日に及ばない）。
    assert!(jp.is_holiday(d(1900, 1, 1)));
    assert_eq!(
        jp.is_holiday_name(d(1900, 1, 1)).as_deref(),
        Some("創業記念日")
    );

    // 一方、組込みの国民の祝日は施行前なので無効のまま。
    assert!(!jp.is_holiday(d(1900, 1, 15))); // 成人の日（施行前）
}
