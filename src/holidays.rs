use super::chrono::{Datelike, NaiveDate, Weekday, Duration};
use super::utils::{week_day};
use std::borrow::Borrow;
use super::registry::Registry;
use dyn_clone::DynClone;
use std::ops::Add;

pub trait Holiday: DynClone {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        if self.exists_holiday(date) {
            return true;
        }

        return false;
    }
    fn is_holiday_name(&self, date: &NaiveDate) -> Option<String> {
        if self.exists_holiday(date) {
            return Some(self.resolve_holiday_name(date));
        }

        return None;
    }
    fn is_basic_holiday(&self) -> bool {
        return true;
    }
    fn exists_holiday(&self, date: &NaiveDate) -> bool;
    fn resolve_holiday_name(&self, date: &NaiveDate) -> String;
}
dyn_clone::clone_trait_object!(Holiday);

// 元日
#[derive(Clone)]
pub struct NewYear {}

impl Holiday for NewYear {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 1 && date.day() == 1 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "元日".to_string()
    }
}

// 成人の日
#[derive(Clone)]
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
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "成人の日".to_string()
    }
}

// 建国記念の日
#[derive(Clone)]
pub struct FoundationDay {}

impl Holiday for FoundationDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.year() >= 1967 && date.month() == 2 && date.day() == 11 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "建国記念の日".to_string()
    }
}

// 天皇誕生日
#[derive(Clone)]
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
        else if date.year() >= 2020 && date.month() == 2 && date.day() == 23 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "天皇誕生日".to_string()
    }
}

// 春分の日
#[derive(Clone)]
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

        let i: f64;

        if (1851..1899 + 1).contains(&year) {
            i = 19.8277;
        } else if (1900..1979 + 1).contains(&year) {
            i = 20.8357;
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
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "春分の日".to_string()
    }
}

// みどりの日
#[derive(Clone)]
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
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "みどりの日".to_string()
    }
}

// 昭和の日
#[derive(Clone)]
pub struct ShowaDay {}

impl Holiday for ShowaDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.year() >= 2007 && date.month() == 4 && date.day() == 29 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "昭和の日".to_string()
    }
}

// 憲法記念日
#[derive(Clone)]
pub struct ConstitutionMemorialDay {}

impl Holiday for ConstitutionMemorialDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 5 && date.day() == 3 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "憲法記念日".to_string()
    }
}

// こどもの日
#[derive(Clone)]
pub struct ChildrensDay {}

impl Holiday for ChildrensDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 5 && date.day() == 5 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "こどもの日".to_string()
    }
}

// 海の日
#[derive(Clone)]
pub struct SeaDay {}

impl Holiday for SeaDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if (1996..2002 + 1).contains(&date.year()) && date.month() == 7 && date.day() == 20 {
            return true;
        }
        // 2020: 国民の祝日に関する法律の一部を改正する法律(平成30年法律第57号)
        else if date.year() >= 2003 && date.year() != 2020 && date.month() == 7 && date.day() == week_day(date, 3, Weekday::Mon).unwrap().day() {
            return true;
        }

        // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
        if date == NaiveDate::from_ymd(2020, 7, 23).borrow() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "海の日".to_string()
    }
}

// 山の日
#[derive(Clone)]
pub struct MountainDay {}

impl Holiday for MountainDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        // 2016: 国民の祝日に関する法律の一部を改正する法律(平成26年法律第43号)
        // 2020: 国民の祝日に関する法律の一部を改正する法律(平成30年法律第57号)
        if date.year() >= 2016 && date.year() != 2020 && date.month() == 8 && date.day() == 11 {
            return true;
        }

        // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
        if date == NaiveDate::from_ymd(2020, 8, 10).borrow() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "山の日".to_string()
    }
}

// 敬老の日
#[derive(Clone)]
pub struct RespectForTheAgedDay {}

impl Holiday for RespectForTheAgedDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if (1966..2002 + 1).contains(&date.year()) && date.month() == 9 && date.day() == 15 {
            return true;
        } else if date.year() >= 2003 && date.month() == 9 && date.day() == week_day(date, 3, Weekday::Mon).unwrap().day() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "敬老の日".to_string()
    }
}

// 秋分の日
#[derive(Clone)]
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

        let i: f64;

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
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "秋分の日".to_string()
    }
}

// 体育の日
#[derive(Clone)]
pub struct HealthAndSportsDay {}

impl Holiday for HealthAndSportsDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if (1966..1999 + 1).contains(&date.year()) && date.month() == 10 && date.day() == 10 {
            return true;
        } else if (2000..2019 + 1).contains(&date.year()) && date.month() == 10 && date.day() == week_day(date, 2, Weekday::Mon).unwrap().day() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "体育の日".to_string()
    }
}

// スポーツの日
#[derive(Clone)]
pub struct SportsDay {}

impl Holiday for SportsDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        // 2020: 国民の祝日に関する法律の一部を改正する法律(平成30年法律第57号)
        if date.year() >= 2020 && date.year() != 2020 && date.month() == 10 && date.day() == week_day(date, 2, Weekday::Mon).unwrap().day() {
            return true;
        }

        // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
        if date == NaiveDate::from_ymd(2020, 7, 24).borrow() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "スポーツの日".to_string()
    }
}

// 文化の日
#[derive(Clone)]
pub struct CultureDay {}

impl Holiday for CultureDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 11 && date.day() == 3 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "文化の日".to_string()
    }
}

// 勤労感謝の日
#[derive(Clone)]
pub struct LaborThanksgivingDay {}

impl Holiday for LaborThanksgivingDay {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if date.month() == 11 && date.day() == 23 {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "勤労感謝の日".to_string()
    }
}

// 皇室慶弔行事に伴う祝日
#[derive(Clone)]
pub struct ExtraHolidays {}

impl ExtraHolidays {
    fn extra_holiday_name(&self, date: &NaiveDate) -> Option<String> {
        if date == NaiveDate::from_ymd(1959, 4, 10).borrow() {
            return Some("皇太子・明仁親王の結婚の儀".to_string());
        } else if date == NaiveDate::from_ymd(1989, 2, 24).borrow() {
            return Some("昭和天皇の大喪の礼".to_string());
        } else if date == NaiveDate::from_ymd(1990, 11, 12).borrow() {
            return Some("即位の礼正殿の儀".to_string());
        } else if date == NaiveDate::from_ymd(1993, 6, 9).borrow() {
            return Some("皇太子・皇太子徳仁親王の結婚の儀".to_string());
        } else if date == NaiveDate::from_ymd(2019, 5, 1).borrow() {
            return Some("天皇の即位の日".to_string());
        }
        // 2019: 天皇の即位の日及び即位礼正殿の儀の行われる日を休日とする法律
        else if date == NaiveDate::from_ymd(2019, 10, 22).borrow() {
            return Some("即位礼正殿の儀".to_string());
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
    fn resolve_holiday_name(&self, date: &NaiveDate) -> String {
        self.extra_holiday_name(date).unwrap()
    }
}

// 振替休日
#[derive(Clone)]
pub struct TransferHoliday<'a> {
    pub registry: Registry<'a>
}

impl<'a> TransferHoliday<'a> {
    fn transfer_holiday_name(&self, date: &NaiveDate) -> Option<String> {
        // 1973年(昭和48年)4月12日 - 改正・施行
        if date.year() < 1973 {
            return None;
        }

        // GW
        if date.month() == 5 && date.day() == 6 && (2..3 + 1).contains(&date.weekday().num_days_from_sunday()) {
            for holiday in self.registry.get_registry() {
                if holiday.is_holiday(date.add(Duration::days(-(date.weekday().num_days_from_sunday() as i64))).borrow()) {
                    let holiday_name = holiday.is_holiday_name(date.add(Duration::days(-(date.weekday().num_days_from_sunday() as i64))).borrow()).unwrap();
                    return Some(format!("{} {}", holiday_name, self.holiday_name()));
                }
            }
        }

        // 月曜日以外は無視
        if date.weekday() != Weekday::Mon {
            return None;
        }

        // GW以外
        for holiday in self.registry.get_registry() {
            if holiday.is_holiday(date.add(Duration::days(-1)).borrow()) {
                let holiday_name = holiday.is_holiday_name(date.add(Duration::days(-1)).borrow()).unwrap();
                return Some(format!("{} {}", holiday_name, self.holiday_name()));
            }
        }

        return None;
    }

    fn holiday_name(&self) -> &str {
        "振替休日"
    }
}

impl<'a> Holiday for TransferHoliday<'a> {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        if self.transfer_holiday_name(date).is_some() {
            return true;
        }

        return false;
    }
    fn resolve_holiday_name(&self, date: &NaiveDate) -> String {
        self.transfer_holiday_name(date).unwrap()
    }
    fn is_basic_holiday(&self) -> bool {
        return false;
    }
}

// 国民の休日
#[derive(Clone)]
pub struct NationalHoliday<'a> {
    pub registry: Registry<'a>
}

impl<'a> Holiday for NationalHoliday<'a> {
    fn exists_holiday(&self, date: &NaiveDate) -> bool {
        let mut result = (false, false);

        for holiday in self.registry.get_registry() {
            if holiday.is_holiday(date.add(Duration::days(-1)).borrow()) {
                result.0 = true;
            }
            if holiday.is_holiday(date.add(Duration::days(1)).borrow()) {
                result.1 = true;
            }

            if result == (true, true) {
                return true;
            }
        }

        return false;
    }
    fn resolve_holiday_name(&self, _date: &NaiveDate) -> String {
        "国民の休日".to_string()
    }
}
