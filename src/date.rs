//! 依存ゼロの暦日付型。
//!
//! 年月日のみを保持する最小限の日付を提供します。先発グレゴリオ暦
//! (proleptic Gregorian calendar) を用い、曜日計算・日数加減算は
//! 序数 ↔ 年月日の相互変換アルゴリズムで実装しています。

use crate::error::DateError;
use std::fmt;

/// 非閏年における各月の日数（1 始まり、添字 0 は番兵）。
const DAYS_IN_MONTH: [i64; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// 非閏年において「その月より前」に存在する日数の累積（1 始まり、添字 0 は番兵）。
const DAYS_BEFORE_MONTH: [i64; 13] = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// 400 年周期に含まれる日数。
const DAYS_PER_400Y: i64 = 146_097;
/// 100 年周期に含まれる日数。
const DAYS_PER_100Y: i64 = 36_524;
/// 4 年周期に含まれる日数。
const DAYS_PER_4Y: i64 = 1_461;

/// 指定年が閏年かどうかを返します（先発グレゴリオ暦）。
#[inline]
pub const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// 指定した年月の日数を返します（`month` は 1〜12）。範囲外なら `None`。
#[inline]
pub const fn days_in_month(year: i32, month: u32) -> Option<u32> {
    if month < 1 || month > 12 {
        return None;
    }
    if month == 2 && is_leap_year(year) {
        Some(29)
    } else {
        Some(DAYS_IN_MONTH[month as usize] as u32)
    }
}

/// `year` より前（1 年 1 月 1 日からの起点）に存在する日数。
#[inline]
fn days_before_year(year: i32) -> i64 {
    let y = year as i64 - 1;
    y * 365 + y.div_euclid(4) - y.div_euclid(100) + y.div_euclid(400)
}

/// その年における `month` より前の日数（閏年を考慮）。
#[inline]
fn days_before_month(year: i32, month: u32) -> i64 {
    let mut days = DAYS_BEFORE_MONTH[month as usize];
    if month > 2 && is_leap_year(year) {
        days += 1;
    }
    days
}

/// 暦上の 1 日を表す型（先発グレゴリオ暦）。
///
/// 比較は年・月・日の辞書順で行われ、暦の前後関係と一致します。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl Date {
    /// 年月日から日付を生成します。実在しない日付（例: 2 月 30 日）は
    /// [`DateError::InvalidDate`] を返します。
    ///
    /// # Examples
    /// ```
    /// use jpholiday::Date;
    /// assert!(Date::new(2017, 1, 1).is_ok());
    /// assert!(Date::new(2017, 2, 30).is_err());
    /// ```
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, DateError> {
        match days_in_month(year, month) {
            Some(dim) if day >= 1 && day <= dim => Ok(Date { year, month, day }),
            _ => Err(DateError::InvalidDate { year, month, day }),
        }
    }

    /// 年月日から日付を生成します（[`Date::new`] のエイリアス）。
    #[inline]
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, DateError> {
        Date::new(year, month, day)
    }

    /// 年を返します。
    #[inline]
    pub const fn year(self) -> i32 {
        self.year
    }

    /// 月（1〜12）を返します。
    #[inline]
    pub const fn month(self) -> u32 {
        self.month
    }

    /// 日（1〜31）を返します。
    #[inline]
    pub const fn day(self) -> u32 {
        self.day
    }

    /// 1 年 1 月 1 日を 1 とする序数を返します。
    pub fn to_ordinal(self) -> i64 {
        days_before_year(self.year) + days_before_month(self.year, self.month) + self.day as i64
    }

    /// 序数から日付を復元します。
    pub fn from_ordinal(ordinal: i64) -> Self {
        // 0 始まりにする。
        let mut n = ordinal - 1;

        let n400 = n.div_euclid(DAYS_PER_400Y);
        n = n.rem_euclid(DAYS_PER_400Y);
        let mut year = n400 * 400 + 1;

        let n100 = n / DAYS_PER_100Y;
        n %= DAYS_PER_100Y;
        let n4 = n / DAYS_PER_4Y;
        n %= DAYS_PER_4Y;
        let n1 = n / 365;
        n %= 365;

        year += n100 * 100 + n4 * 4 + n1;

        // n1 == 4 または n100 == 4 のときは閏年周期の末日（12/31）。
        if n1 == 4 || n100 == 4 {
            return Date {
                year: (year - 1) as i32,
                month: 12,
                day: 31,
            };
        }

        let year = year as i32;
        let leap = n1 == 3 && (n4 != 24 || n100 == 3);

        let mut month = ((n + 50) >> 5) as u32;
        let mut preceding =
            DAYS_BEFORE_MONTH[month as usize] + if month > 2 && leap { 1 } else { 0 };
        if preceding > n {
            month -= 1;
            preceding -= DAYS_IN_MONTH[month as usize] + if month == 2 && leap { 1 } else { 0 };
        }
        n -= preceding;

        Date {
            year,
            month,
            day: (n + 1) as u32,
        }
    }

    /// `days` 日後（負なら前）の日付を返します。
    #[inline]
    pub fn add_days(self, days: i64) -> Self {
        Date::from_ordinal(self.to_ordinal() + days)
    }

    /// 翌日を返します。
    #[inline]
    pub fn succ(self) -> Self {
        self.add_days(1)
    }

    /// 前日を返します。
    #[inline]
    pub fn pred(self) -> Self {
        self.add_days(-1)
    }

    /// ISO 曜日を返します（月曜=1 〜 日曜=7）。
    #[inline]
    pub fn iso_weekday(self) -> u32 {
        ((self.to_ordinal() - 1).rem_euclid(7) + 1) as u32
    }

    /// 曜日を返します（月曜=0 〜 日曜=6）。
    #[inline]
    pub fn weekday(self) -> u32 {
        (self.to_ordinal() - 1).rem_euclid(7) as u32
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year_rules() {
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn invalid_dates_rejected() {
        assert!(Date::new(2017, 2, 30).is_err());
        assert!(Date::new(2017, 13, 1).is_err());
        assert!(Date::new(2017, 0, 1).is_err());
        assert!(Date::new(2017, 1, 0).is_err());
        assert!(Date::new(2023, 2, 29).is_err());
        assert!(Date::new(2024, 2, 29).is_ok());
    }

    #[test]
    fn ordinal_roundtrip() {
        // 既知の固定点: 1 年 1 月 1 日は序数 1。
        assert_eq!(Date::new(1, 1, 1).unwrap().to_ordinal(), 1);
        // 広範囲で往復一致を確認。
        let mut d = Date::new(1971, 1, 1).unwrap();
        let end = Date::new(2031, 1, 1).unwrap();
        while d < end {
            let o = d.to_ordinal();
            assert_eq!(Date::from_ordinal(o), d, "roundtrip failed at {d}");
            d = d.succ();
        }
    }

    #[test]
    fn known_weekdays() {
        // 2017-01-01 は日曜日。
        assert_eq!(Date::new(2017, 1, 1).unwrap().iso_weekday(), 7);
        // 2020-02-09 は日曜日。
        assert_eq!(Date::new(2020, 2, 9).unwrap().iso_weekday(), 7);
        // 2017-01-02 は月曜日。
        assert_eq!(Date::new(2017, 1, 2).unwrap().iso_weekday(), 1);
        assert_eq!(Date::new(2017, 1, 2).unwrap().weekday(), 0);
    }

    #[test]
    fn succ_pred_rollover() {
        assert_eq!(
            Date::new(2019, 12, 31).unwrap().succ(),
            Date::new(2020, 1, 1).unwrap()
        );
        assert_eq!(
            Date::new(2020, 1, 1).unwrap().pred(),
            Date::new(2019, 12, 31).unwrap()
        );
        assert_eq!(
            Date::new(2024, 2, 28).unwrap().succ(),
            Date::new(2024, 2, 29).unwrap()
        );
        assert_eq!(
            Date::new(2023, 2, 28).unwrap().succ(),
            Date::new(2023, 3, 1).unwrap()
        );
    }

    #[test]
    fn ordering_and_display() {
        assert!(Date::new(2017, 1, 1).unwrap() < Date::new(2017, 1, 2).unwrap());
        assert!(Date::new(2017, 1, 1).unwrap() < Date::new(2018, 1, 1).unwrap());
        assert_eq!(format!("{}", Date::new(2017, 1, 1).unwrap()), "2017-01-01");
    }
}
