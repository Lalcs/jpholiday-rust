use chrono::{Datelike, NaiveDate, Weekday};
use super::utils::{week_day};
use std::borrow::Borrow;
use crate::registry::Registry;

pub trait Holiday {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        if self.exists_holiday(date) {
            return true;
        }

        return false;
    }
    fn is_holiday_name(&self, date: &NaiveDate) -> Result<&str, &str> {
        if self.exists_holiday(date) {
            return Ok(self.resolve_holiday_name(date));
        }

        return Err("祝日ではありません。");
    }
    fn is_basic_holiday(&self) -> bool {
        return true;
    }
    fn exists_holiday(&self, date: &NaiveDate) -> bool;
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str;
}

// 元旦
pub struct NewYear {}

impl Holiday for NewYear {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 1 && date.day() == 1 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "元旦"
    }
}

// 成人の日
pub struct AdultDay {}

impl Holiday for AdultDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if (1948..1999 + 1).contains(&date.year()) && date.month() == 1 && date.day() == 15 {
            return true;
        } else if date.year() >= 2000 && date.month() == 1 && date.day() == week_day(date, 2, Weekday::Mon).unwrap().day() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "成人の日"
    }
}

// 建国記念の日
pub struct FoundationDay {}

impl Holiday for FoundationDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.year() >= 1967 && date.month() == 2 && date.day() == 11 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "建国記念の日"
    }
}

// 天皇誕生日
pub struct EmperorsBirthday {}

impl Holiday for EmperorsBirthday {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        // 1948-1988年
        if (1948..1988 + 1).contains(&date.year()) && date.month() == 4 && date.day() == 29 {
            return true;
        }
        // 1988-2018年
        // 2019: 国民の祝日に関する法律(昭和23年法律第178号)の一部改正
        else if (1988..2018 + 1).contains(&date.year()) && date.month() == 12 && date.day() == 23 {
            return true;
        }
        // 2019: 国民の祝日に関する法律(昭和23年法律第178号)の一部改正
        else if date == NaiveDate::from_ymd(2020, 2, 23).borrow() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "天皇誕生日"
    }
}

// 春分の日
pub struct VernalEquinoxDay {}

impl VernalEquinoxDay {
    fn vernal_equinox_day(&self, year: i32) -> u32 {
        // 春季皇霊祭: 1879-1947
        // 春分の日: 1948
        // 春分の日の日付を返します。
        // http://mt-soft.sakura.ne.jp/kyozai/excel_high/200_jissen_kiso/60_syunbun.htm
        if year <= 1948 {
            return 0;
        }

        let mut i: f64 = 0.0;

        if (1851..1899 + 1).contains(&year) {
            i = 0.0;
        } else if (1900..1979 + 1).contains(&year) {
            i = 19.8277;
        } else if (1980..2099 + 1).contains(&year) {
            i = 20.8431;
        } else if (2100..2150 + 1).contains(&year) {
            i = 21.8510;
        } else {
            i = 0.0;
        }

        return ((i + 0.242194 * ((year as f64) - 1980.0)) - (((year as f64) - 1980.0) / 4.0).floor()).floor() as u32;
    }
}

impl Holiday for VernalEquinoxDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 3 && date.day() == self.vernal_equinox_day(date.year()) {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "春分の日"
    }
}

// みどりの日
pub struct GreeneryDay {}

impl Holiday for GreeneryDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        // 1989-2006年
        if (1989..2006 + 1).contains(&date.year()) && date.month() == 4 && date.day() == 29 {
            return true;
        } else if date.year() >= 2007 && date.month() == 5 && date.day() == 4 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "みどりの日"
    }
}

// 昭和の日
pub struct ShowaDay {}

impl Holiday for ShowaDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.year() >= 2007 && date.month() == 4 && date.day() == 29 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "昭和の日"
    }
}

// 憲法記念日
pub struct ConstitutionMemorialDay {}

impl Holiday for ConstitutionMemorialDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 5 && date.day() == 3 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "憲法記念日"
    }
}

// こどもの日
pub struct ChildrensDay {}

impl Holiday for ChildrensDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 5 && date.day() == 5 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "こどもの日"
    }
}

// 海の日
pub struct SeaDay {}

impl Holiday for SeaDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if (1996..2002 + 1).contains(&date.year()) && date.month() == 7 && date.day() == 20 {
            return true;
        }
        // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
        // 2020: 国民の祝日に関する法律の一部を改正する法律(平成30年法律第57号)
        else if date == NaiveDate::from_ymd(2020, 7, 23).borrow() {
            return true;
        } else if date.year() >= 2003 && date.month() == 7 && date.day() == week_day(date, 3, Weekday::Mon).unwrap().day() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "海の日"
    }
}

// 山の日
pub struct MountainDay {}

impl Holiday for MountainDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        // 2020: 国民の祝日に関する法律の一部を改正する法律(平成30年法律第57号)
        // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
        if date == NaiveDate::from_ymd(2020, 8, 10).borrow() {
            return true;
        }
        // 2016: 国民の祝日に関する法律の一部を改正する法律(平成26年法律第43号)
        else if date.year() >= 2016 && date.month() == 8 && date.day() == 11 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "山の日"
    }
}

// 敬老の日
pub struct RespectForTheAgedDay {}

impl Holiday for RespectForTheAgedDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if (1996..2002 + 1).contains(&date.year()) && date.month() == 9 && date.day() == 15 {
            return true;
        } else if date.year() >= 2003 && date.month() == 9 && date.day() == week_day(date, 3, Weekday::Mon).unwrap().day() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "敬老の日"
    }
}

// 秋分の日
pub struct AutumnEquinoxDay {}

impl AutumnEquinoxDay {
    fn autumn_equinox_day(&self, year: i32) -> u32 {
        // 秋分の日の日付を返します。
        // 秋季皇霊祭: 1879-1947
        // 秋分の日: 1948
        // http://mt-soft.sakura.ne.jp/kyozai/excel_high/200_jissen_kiso/60_syunbun.htm
        if year <= 1948 {
            return 0;
        }

        let mut i: f64 = 0.0;

        if (1851..1899 + 1).contains(&year) {
            i = 22.2588;
        } else if (1900..1979 + 1).contains(&year) {
            i = 23.2588;
        } else if (1980..2099 + 1).contains(&year) {
            i = 23.2488;
        } else if (2100..2150 + 1).contains(&year) {
            i = 24.2488;
        } else {
            i = 0.0;
        }

        return ((i + 0.242194 * ((year as f64) - 1980.0)) - (((year as f64) - 1980.0) / 4.0).floor()).floor() as u32;
    }
}

impl Holiday for AutumnEquinoxDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 9 && date.day() == self.autumn_equinox_day(date.year()) {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "秋分の日"
    }
}

// 体育の日
pub struct HealthAndSportsDay {}

impl Holiday for HealthAndSportsDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if (1996..1999 + 1).contains(&date.year()) && date.month() == 10 && date.day() == 10 {
            return true;
        } else if (2000..2019 + 1).contains(&date.year()) && date.month() == 10 && date.day() == week_day(date, 2, Weekday::Mon).unwrap().day() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "体育の日"
    }
}

// スポーツの日
pub struct SportsDay {}

impl Holiday for SportsDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        // 2020: 国民の祝日に関する法律の一部を改正する法律(平成30年法律第57号)
        // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
        if date == NaiveDate::from_ymd(2020, 7, 24).borrow() {
            return true;
        } else if date.year() >= 2020 && date.month() == 10 && date.day() == week_day(date, 2, Weekday::Mon).unwrap().day() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "スポーツの日"
    }
}

// 文化の日
pub struct CultureDay {}

impl Holiday for CultureDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 11 && date.day() == 3 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "文化の日"
    }
}

// 勤労感謝の日
pub struct LaborThanksgivingDay {}

impl Holiday for LaborThanksgivingDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 11 && date.day() == 23 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        "勤労感謝の日"
    }
}

// 皇室慶弔行事に伴う祝日
pub struct ExtraHolidays {}

impl ExtraHolidays {
    fn extra_holiday_name(&self, date: &NaiveDate) -> Option<&str> {
        if date == NaiveDate::from_ymd(1959, 4, 10).borrow() {
            return Some("皇太子・明仁親王の結婚の儀");
        } else if date == NaiveDate::from_ymd(1989, 2, 24).borrow() {
            return Some("昭和天皇の大喪の礼");
        } else if date == NaiveDate::from_ymd(1990, 11, 12).borrow() {
            return Some("即位の礼正殿の儀");
        } else if date == NaiveDate::from_ymd(1993, 6, 9).borrow() {
            return Some("皇太子・皇太子徳仁親王の結婚の儀");
        } else if date == NaiveDate::from_ymd(2019, 5, 1).borrow() {
            return Some("天皇の即位の日");
        }
        // 2019: 天皇の即位の日及び即位礼正殿の儀の行われる日を休日とする法律
        else if date == NaiveDate::from_ymd(2019, 10, 22).borrow() {
            return Some("即位礼正殿の儀");
        }

        return None;
    }
}

impl Holiday for ExtraHolidays {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if self.extra_holiday_name(date).is_some() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        self.extra_holiday_name(date).unwrap()
    }
}

// 振替休日
pub struct TransferHoliday<'a> {
    pub registry: &'a Registry<'a>
}

impl<'a> TransferHoliday<'a> {
    fn transfer_holiday_name(&self, date: &NaiveDate) -> Option<&str> {
        // 1973年(昭和48年)4月12日 - 改正・施行
        if date.year() < 1973 {
            return None;
        }

        if date.month() == 5 && date.day() == 6 && (2..3 + 1).contains(&date.weekday().num_days_from_monday()) {
            for holiday in self.registry.get_registry() {
                println!("{:?}", holiday.resolve_holiday_name(date));
            }
        }
        return Some("ddddddddd");
    }
}

impl<'a>  Holiday for TransferHoliday<'a> {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if self.transfer_holiday_name(date).is_some() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> &str {
        self.transfer_holiday_name(date).unwrap()
    }
    fn is_basic_holiday(&self) -> bool {
        return false;
    }
}
