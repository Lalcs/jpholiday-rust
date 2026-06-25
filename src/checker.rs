//! 祝日チェッカー群。
//!
//! 組込みの祝日（祝日法の年代別ロジック）、振替休日、国民の休日の判定を行います。

use crate::astronomy;
use crate::date::{Date, days_in_month};
use crate::model::Holiday;
use std::any::TypeId;
use std::sync::Arc;

/// 祝日法（昭和23年法律第178号）の施行日 `(年, 月, 日)` = 1948-07-20。
///
/// この日より前には国民の祝日は法的に存在しないため、組込みの祝日（および振替休日・国民の休日）は
/// すべて施行日以降のみ有効とする。祝日法に忠実に施行日で区切り、施行日より前の年・日付には
/// 国民の祝日を返さない（利用者の独自祝日には影響しない）。
const NATIONAL_HOLIDAY_ACT_ENFORCEMENT: (i32, u32, u32) = (1948, 7, 20);

/// 利用者が独自の祝日を定義するためのトレイト。
///
/// 実装した型を [`crate::JPHoliday::register`] や [`crate::register`] で登録すると、
/// 判定に加わります。
///
/// # Examples
/// ```
/// use jpholiday::{Date, OriginalHolidayChecker};
///
/// struct CompanyFoundationDay;
/// impl OriginalHolidayChecker for CompanyFoundationDay {
///     fn is_holiday(&self, date: Date) -> bool {
///         date.month() == 2 && date.day() == 9
///     }
///     fn holiday_name(&self, _date: Date) -> String {
///         "特別休暇".to_string()
///     }
/// }
/// ```
pub trait OriginalHolidayChecker: Send + Sync {
    /// 指定日が独自の休日かどうかを返します。
    fn is_holiday(&self, date: Date) -> bool;
    /// 指定日の休日名を返します。
    fn holiday_name(&self, date: Date) -> String;
}

/// 登録済みの独自チェッカー 1 件。重複登録・登録解除を型単位で扱うため `TypeId` を保持します。
///
/// `Clone` は `Arc` の参照カウント増加のみで安価です（グローバル API がロックの外で計算するため
/// レジストリのスナップショットに用います）。
#[derive(Clone)]
pub(crate) struct OriginalEntry {
    pub(crate) type_id: TypeId,
    pub(crate) checker: Arc<dyn OriginalHolidayChecker>,
}

/// 「その月の第 `nth` 週の ISO 曜日 `iso_weekday`」に当たる日を返します。
pub(crate) fn nth_weekday_day(year: i32, month: u32, nth: u32, iso_weekday: u32) -> Option<u32> {
    if !(1..=5).contains(&nth) || !(1..=7).contains(&iso_weekday) {
        return None;
    }
    let dim = days_in_month(year, month)?;
    let mut count = 0;
    for day in 1..=dim {
        if let Ok(date) = Date::new(year, month, day)
            && date.iso_weekday() == iso_weekday
        {
            count += 1;
            if count == nth {
                return Some(day);
            }
        }
    }
    None
}

/// 組込みの祝日種別。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Builtin {
    /// 元日
    NewYear,
    /// 成人の日
    AdultDay,
    /// 建国記念の日
    FoundationDay,
    /// 天皇誕生日
    EmperorsBirthday,
    /// 春分の日
    VernalEquinox,
    /// みどりの日
    GreeneryDay,
    /// 昭和の日
    ShowaDay,
    /// 憲法記念日
    ConstitutionMemorialDay,
    /// こどもの日
    ChildrensDay,
    /// 海の日
    SeaDay,
    /// 山の日
    MountainDay,
    /// 敬老の日
    RespectForTheAgedDay,
    /// 秋分の日
    AutumnEquinox,
    /// 体育の日
    HealthAndSportsDay,
    /// スポーツの日
    SportsDay,
    /// 文化の日
    CultureDay,
    /// 勤労感謝の日
    LaborThanksgivingDay,
    /// 1959 年 皇太子・明仁親王の結婚の儀
    ExtraHoliday1959,
    /// 1989 年 昭和天皇の大喪の礼
    ExtraHoliday1989,
    /// 1990 年 即位の礼正殿の儀
    ExtraHoliday1990,
    /// 1993 年 皇太子・皇太子徳仁親王の結婚の儀
    ExtraHoliday1993,
    /// 2019 年 5 月 1 日 天皇の即位の日
    ExtraHoliday2019May,
    /// 2019 年 10 月 22 日 即位礼正殿の儀
    ExtraHoliday2019Oct,
}

impl Builtin {
    /// 既定のレジストリ順。
    pub(crate) const ALL: [Builtin; 23] = [
        Builtin::NewYear,
        Builtin::AdultDay,
        Builtin::FoundationDay,
        Builtin::EmperorsBirthday,
        Builtin::VernalEquinox,
        Builtin::GreeneryDay,
        Builtin::ShowaDay,
        Builtin::ConstitutionMemorialDay,
        Builtin::ChildrensDay,
        Builtin::SeaDay,
        Builtin::MountainDay,
        Builtin::RespectForTheAgedDay,
        Builtin::AutumnEquinox,
        Builtin::HealthAndSportsDay,
        Builtin::SportsDay,
        Builtin::CultureDay,
        Builtin::LaborThanksgivingDay,
        Builtin::ExtraHoliday1959,
        Builtin::ExtraHoliday1989,
        Builtin::ExtraHoliday1990,
        Builtin::ExtraHoliday1993,
        Builtin::ExtraHoliday2019May,
        Builtin::ExtraHoliday2019Oct,
    ];

    /// 指定日がこの祝日に当たるかを返します。
    pub(crate) fn is_holiday(self, date: Date) -> bool {
        let (y, m, d) = (date.year(), date.month(), date.day());

        // 祝日法の施行日（1948-07-20）より前に国民の祝日は存在しない。
        // この境界判定により、振替休日・国民の休日の遡及参照も含めて一律に施行日で区切られる。
        if (y, m, d) < NATIONAL_HOLIDAY_ACT_ENFORCEMENT {
            return false;
        }

        match self {
            Builtin::NewYear => m == 1 && d == 1,

            Builtin::AdultDay => {
                (y <= 1999 && m == 1 && d == 15)
                    || (y >= 2000 && m == 1 && nth_weekday_day(y, 1, 2, 1) == Some(d))
            }

            Builtin::FoundationDay => y >= 1967 && m == 2 && d == 11,

            Builtin::EmperorsBirthday => {
                ((1948..=1988).contains(&y) && m == 4 && d == 29)
                    || ((1989..=2018).contains(&y) && m == 12 && d == 23)
                    || (y >= 2020 && m == 2 && d == 23)
            }

            Builtin::VernalEquinox => m == 3 && d == astronomy::calculate_vernal_equinox(y),

            Builtin::GreeneryDay => {
                ((1989..=2006).contains(&y) && m == 4 && d == 29) || (y >= 2007 && m == 5 && d == 4)
            }

            Builtin::ShowaDay => y >= 2007 && m == 4 && d == 29,

            Builtin::ConstitutionMemorialDay => m == 5 && d == 3,

            Builtin::ChildrensDay => m == 5 && d == 5,

            Builtin::SeaDay => {
                // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
                if y == 2020 {
                    return m == 7 && d == 23;
                }
                // 2021: 五輪特別措置法改正案
                if y == 2021 {
                    return m == 7 && d == 22;
                }
                ((1996..=2002).contains(&y) && m == 7 && d == 20)
                    || (y >= 2003 && m == 7 && nth_weekday_day(y, 7, 3, 1) == Some(d))
            }

            Builtin::MountainDay => {
                // 2020: 特例
                if y == 2020 {
                    return m == 8 && d == 10;
                }
                // 2021: 五輪特別措置法改正案
                if y == 2021 {
                    return m == 8 && d == 8;
                }
                y >= 2016 && m == 8 && d == 11
            }

            Builtin::RespectForTheAgedDay => {
                ((1966..=2002).contains(&y) && m == 9 && d == 15)
                    || (y >= 2003 && m == 9 && nth_weekday_day(y, 9, 3, 1) == Some(d))
            }

            Builtin::AutumnEquinox => m == 9 && d == astronomy::calculate_autumn_equinox(y),

            Builtin::HealthAndSportsDay => {
                ((1966..=1999).contains(&y) && m == 10 && d == 10)
                    || ((2000..=2019).contains(&y)
                        && m == 10
                        && nth_weekday_day(y, 10, 2, 1) == Some(d))
            }

            Builtin::SportsDay => {
                // 2020: 特例
                if y == 2020 {
                    return m == 7 && d == 24;
                }
                // 2021: 五輪特別措置法改正案
                if y == 2021 {
                    return m == 7 && d == 23;
                }
                y >= 2020 && m == 10 && nth_weekday_day(y, 10, 2, 1) == Some(d)
            }

            Builtin::CultureDay => m == 11 && d == 3,

            Builtin::LaborThanksgivingDay => m == 11 && d == 23,

            Builtin::ExtraHoliday1959 => y == 1959 && m == 4 && d == 10,
            Builtin::ExtraHoliday1989 => y == 1989 && m == 2 && d == 24,
            Builtin::ExtraHoliday1990 => y == 1990 && m == 11 && d == 12,
            Builtin::ExtraHoliday1993 => y == 1993 && m == 6 && d == 9,
            Builtin::ExtraHoliday2019May => y == 2019 && m == 5 && d == 1,
            Builtin::ExtraHoliday2019Oct => y == 2019 && m == 10 && d == 22,
        }
    }

    /// この祝日の名称を返します。
    pub(crate) fn name(self) -> &'static str {
        match self {
            Builtin::NewYear => "元日",
            Builtin::AdultDay => "成人の日",
            Builtin::FoundationDay => "建国記念の日",
            Builtin::EmperorsBirthday => "天皇誕生日",
            Builtin::VernalEquinox => "春分の日",
            Builtin::GreeneryDay => "みどりの日",
            Builtin::ShowaDay => "昭和の日",
            Builtin::ConstitutionMemorialDay => "憲法記念日",
            Builtin::ChildrensDay => "こどもの日",
            Builtin::SeaDay => "海の日",
            Builtin::MountainDay => "山の日",
            Builtin::RespectForTheAgedDay => "敬老の日",
            Builtin::AutumnEquinox => "秋分の日",
            Builtin::HealthAndSportsDay => "体育の日",
            Builtin::SportsDay => "スポーツの日",
            Builtin::CultureDay => "文化の日",
            Builtin::LaborThanksgivingDay => "勤労感謝の日",
            Builtin::ExtraHoliday1959 => "皇太子・明仁親王の結婚の儀",
            Builtin::ExtraHoliday1989 => "昭和天皇の大喪の礼",
            Builtin::ExtraHoliday1990 => "即位の礼正殿の儀",
            Builtin::ExtraHoliday1993 => "皇太子・皇太子徳仁親王の結婚の儀",
            Builtin::ExtraHoliday2019May => "天皇の即位の日",
            Builtin::ExtraHoliday2019Oct => "即位礼正殿の儀",
        }
    }
}

/// レジストリに登録されるチェッカー 1 件。
#[derive(Clone)]
pub(crate) enum Checker {
    /// 組込みの祝日。
    Builtin(Builtin),
    /// 振替休日。
    Transfer,
    /// 国民の休日。
    National,
    /// 利用者が登録した独自の祝日。
    Original(OriginalEntry),
}

/// 指定したチェッカーで `date` を判定し、祝日ならその名称を返します。
///
/// `all` は同一レジストリの全チェッカー（振替休日・国民の休日が他のチェッカーを参照するため）。
pub(crate) fn checker_holiday(checker: &Checker, date: Date, all: &[Checker]) -> Option<String> {
    match checker {
        Checker::Builtin(b) => {
            if b.is_holiday(date) {
                Some(b.name().to_string())
            } else {
                None
            }
        }
        Checker::Transfer => transfer_name(date, all),
        Checker::National => {
            if national_holiday(date, all) {
                Some("国民の休日".to_string())
            } else {
                None
            }
        }
        Checker::Original(e) => {
            if e.checker.is_holiday(date) {
                Some(e.checker.holiday_name(date))
            } else {
                None
            }
        }
    }
}

/// 指定日に該当するすべての祝日を計算します（キャッシュなし）。
///
/// `checkers` をレジストリ順に走査し、該当した各チェッカーの祝日を収集します。グローバル API は
/// レジストリのスナップショットをロックの外でこの関数に渡すため、独自チェッカーの再入呼び出しでも
/// デッドロックしません。
pub(crate) fn compute_holidays(checkers: &[Checker], date: Date) -> Vec<Holiday> {
    let mut result = Vec::new();
    for checker in checkers {
        if let Some(name) = checker_holiday(checker, date, checkers) {
            result.push(Holiday::new(date, name));
        }
    }
    result
}

/// `date` に当たる最初の組込み祝日を返します（レジストリ順）。
fn first_builtin_on(date: Date, all: &[Checker]) -> Option<Builtin> {
    all.iter().find_map(|c| match c {
        Checker::Builtin(b) if b.is_holiday(date) => Some(*b),
        _ => None,
    })
}

/// 振替休日の名称を返します（該当しなければ `None`）。
///
/// 組込み祝日のみを対象とし、振替休日・国民の休日・独自祝日は対象から除外します。
pub(crate) fn transfer_name(date: Date, all: &[Checker]) -> Option<String> {
    // 1973 年（昭和48年）4 月 12 日 改正・施行。
    if date.year() < 1973 {
        return None;
    }
    // 日曜日に振替休日は存在しない。
    if date.iso_weekday() == 7 {
        return None;
    }
    // 祝日が存在する日に振替休日は存在しない。
    if first_builtin_on(date, all).is_some() {
        return None;
    }

    let mut current = date.pred();
    loop {
        match first_builtin_on(current, all) {
            None => return None,
            Some(b) => {
                if current.iso_weekday() == 7 {
                    return Some(format!("{} 振替休日", b.name()));
                }
            }
        }
        current = current.pred();
    }
}

/// 国民の休日（祝日に挟まれた平日）かどうかを返します。
///
/// 国民の休日・独自祝日は対象から除外し、組込み祝日と振替休日を対象とします。
pub(crate) fn national_holiday(date: Date, all: &[Checker]) -> bool {
    if date.iso_weekday() == 7 {
        return false;
    }

    let is_hol = |d: Date| -> bool {
        all.iter().any(|c| match c {
            Checker::National | Checker::Original(_) => false,
            Checker::Builtin(b) => b.is_holiday(d),
            Checker::Transfer => transfer_name(d, all).is_some(),
        })
    };

    if is_hol(date) {
        return false;
    }

    is_hol(date.pred()) && is_hol(date.succ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_weekday_basic() {
        // 2017 年 1 月の第 2 月曜日は 9 日（成人の日）。
        assert_eq!(nth_weekday_day(2017, 1, 2, 1), Some(9));
        // 2017 年 7 月の第 3 月曜日は 17 日（海の日）。
        assert_eq!(nth_weekday_day(2017, 7, 3, 1), Some(17));
        assert_eq!(nth_weekday_day(2017, 1, 0, 1), None);
        assert_eq!(nth_weekday_day(2017, 1, 2, 8), None);
    }
}
