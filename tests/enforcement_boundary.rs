//! 祝日法（昭和23年法律第178号）の施行日境界（1948-07-20）と、振替休日（1973-04-12 施行）・
//! 国民の休日（1985-12-27 施行）の施行日境界の検証。
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

#[test]
fn no_transfer_holiday_before_its_enforcement() {
    // 振替休日は 1973-04-12 施行。1973-02-11（日曜日の建国記念の日）の翌日は施行前なので休日ではない。
    assert_eq!(d(1973, 2, 11).iso_weekday(), 7);
    assert_eq!(jpholiday::is_holiday_name(d(1973, 2, 12)), None);
    assert!(!JPHoliday::new().is_holiday(d(1973, 2, 12)));

    // 施行後最初の振替休日（内閣府公表データどおり）。
    assert_eq!(
        jpholiday::is_holiday_name(d(1973, 4, 30)).as_deref(),
        Some("天皇誕生日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1973, 9, 24)).as_deref(),
        Some("秋分の日 振替休日")
    );
}

#[test]
fn no_national_holiday_before_its_enforcement() {
    // 国民の休日は 1985-12-27 施行。それ以前の 5/4 は憲法記念日とこどもの日に挟まれていても休日ではない。
    // 内閣府公表データでは 1949〜1985 年の 5/4 のうち休日は 1981-05-04（5/3 が日曜日の振替休日）のみ。
    for year in 1949..=1985 {
        let may4 = d(year, 5, 4);
        assert_eq!(jpholiday::is_holiday(may4), year == 1981, "{may4}");
    }
    assert_eq!(
        jpholiday::is_holiday_name(d(1981, 5, 4)).as_deref(),
        Some("憲法記念日 振替休日")
    );
    assert_eq!(jpholiday::is_holiday_name(d(1985, 5, 4)), None); // 施行直前の年（土曜日）
    assert!(!JPHoliday::new().is_holiday(d(1971, 5, 4)));

    // 施行後: 1986-05-04 は日曜日、1987-05-04 は振替休日、1988-05-04 が最初の国民の休日。
    assert_eq!(jpholiday::is_holiday_name(d(1986, 5, 4)), None);
    assert_eq!(
        jpholiday::is_holiday_name(d(1987, 5, 4)).as_deref(),
        Some("憲法記念日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 5, 4)).as_deref(),
        Some("国民の休日")
    );
}
