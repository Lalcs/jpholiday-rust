use std::ops::Add;
use super::chrono::{Datelike, NaiveDate, Duration};

use super::registry::Registry;
use super::holidays::*;

pub struct JPHoliday<'a> {
    registry: Registry<'a>
}

impl<'a> JPHoliday<'a> {
    pub fn new() -> Self {
        let mut reg = Registry::new();
        // BasicHoliday
        reg.register(NewYear {});
        reg.register(AdultDay {});
        reg.register(FoundationDay {});
        reg.register(EmperorsBirthday {});
        reg.register(VernalEquinoxDay {});
        reg.register(GreeneryDay {});
        reg.register(ShowaDay {});
        reg.register(ConstitutionMemorialDay {});
        reg.register(ChildrensDay {});
        reg.register(SeaDay {});
        reg.register(MountainDay {});
        reg.register(RespectForTheAgedDay {});
        reg.register(AutumnEquinoxDay {});
        reg.register(HealthAndSportsDay {});
        reg.register(SportsDay {});
        reg.register(CultureDay {});
        reg.register(LaborThanksgivingDay {});
        reg.register(ExtraHolidays {});

        // OtherHoliday
        let basic_holiday_registry = reg.clone();
        reg.register(TransferHoliday { registry: basic_holiday_registry.clone()});
        reg.register(NationalHoliday { registry: basic_holiday_registry.clone()});

        return JPHoliday {
            registry: reg
        };
    }

    /// # 指定日が祝日か判定
    /// ```rust
    /// use jpholiday::jpholiday::JPHoliday;
    /// use jpholiday::chrono::{NaiveDate};
    /// use std::borrow::Borrow;
    ///
    /// let jpholiday = JPHoliday::new();
    ///
    /// assert_eq!(
    ///     jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 1).borrow()),
    ///     true
    /// );
    ///
    /// assert_eq!(
    ///     jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 2).borrow()),
    ///     true
    /// );
    ///
    /// assert_eq!(
    ///     jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 3).borrow()),
    ///     false
    /// );
    /// ```
    pub fn is_holiday(&self, date: &NaiveDate) -> bool {
        for holiday in self.registry.get_registry() {
            if holiday.is_holiday(&date) {
                return true;
            }
        }

        return false;
    }

    /// # 指定日の祝日名を取得
    /// ```
    /// use jpholiday::jpholiday::JPHoliday;
    /// use jpholiday::chrono::{NaiveDate};
    /// use std::borrow::Borrow;
    ///
    /// let jpholiday = JPHoliday::new();
    ///
    /// assert_eq(
    ///     jpholiday.is_holiday_name(NaiveDate::from_ymd(2017, 1, 1).borrow()).unwrap(),
    ///     "元日".to_string()
    /// );
    ///
    /// assert_eq(
    ///     jpholiday.is_holiday_name(NaiveDate::from_ymd(2017, 1, 2).borrow()).unwrap(),
    ///     "元日 振替休日".to_string()
    /// );
    ///
    /// assert_eq(
    ///     jpholiday.is_holiday_name(NaiveDate::from_ymd(2017, 1, 3).borrow()),
    ///     None
    /// );
    /// ```
    pub fn is_holiday_name(&self, date: &NaiveDate) -> Option<String> {
        for holiday in self.registry.get_registry() {
            if holiday.is_holiday(&date) {
                return Some(holiday.is_holiday_name(&date).unwrap());
            }
        }

        return None;
    }

    /// # 指定範囲の祝日を取得
    /// ```rust
    /// use jpholiday::jpholiday::JPHoliday;
    /// use jpholiday::chrono::{NaiveDate};
    /// use std::borrow::Borrow;
    ///
    /// let jpholiday = JPHoliday::new();
    ///
    /// assert_eq!(
    ///     jpholiday.between(NaiveDate::from_ymd(2017, 1, 1).borrow(), NaiveDate::from_ymd(2017, 5, 3).borrow()),
    ///     vec![
    ///         (NaiveDate::from_ymd(2017, 1, 1), "元日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 1, 2), "元日 振替休日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 1, 9), "成人の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 2, 11), "建国記念の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 3, 20), "春分の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 4, 29), "昭和の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 5, 3), "憲法記念日".to_string())
    ///     ]
    /// );
    /// ```
    pub fn between(&self, start_date: &NaiveDate, end_date: &NaiveDate) -> Vec<(NaiveDate, String)> {
        let mut date: NaiveDate = start_date.clone();
        let mut result: Vec<(NaiveDate, String)> = Vec::new();

        loop {
            if self.is_holiday(&date) {
                result.push((date.clone(), self.is_holiday_name(&date).unwrap()));
            }
            if &date == end_date {
                break;
            }

            date = date.add(Duration::days(1));
        }

        return result;
    }

    /// # 指定年の祝日を取得
    /// ```rust
    /// use jpholiday::jpholiday::JPHoliday;
    /// use jpholiday::chrono::{NaiveDate};
    /// use std::borrow::Borrow;
    ///
    /// let jpholiday = JPHoliday::new();
    ///
    /// assert_eq!(
    ///     jpholiday.year_holidays(2017),
    ///     vec![
    ///         (NaiveDate::from_ymd(2017, 1, 1), "元日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 1, 2), "元日 振替休日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 1, 9), "成人の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 2, 11), "建国記念の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 3, 20), "春分の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 4, 29), "昭和の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 5, 3), "憲法記念日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 5, 4), "みどりの日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 5, 5), "こどもの日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 7, 17), "海の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 8, 11), "山の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 9, 18), "敬老の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 9, 23), "秋分の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 10, 9), "体育の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 11, 3), "文化の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 11, 23), "勤労感謝の日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 12, 23), "天皇誕生日".to_string())
    ///     ]
    /// );
    /// ```
    pub fn year_holidays(&self, year: i32) -> Vec<(NaiveDate, String)> {
        let mut date: NaiveDate = NaiveDate::from_ymd(year.clone(), 1, 1);
        let mut result: Vec<(NaiveDate, String)> = Vec::new();

        loop {
            if self.is_holiday(&date) {
                result.push((date.clone(), self.is_holiday_name(&date).unwrap()));
            }

            date = date.add(Duration::days(1));

            if &date.year() != &year {
                break;
            }

        }

        return result;
    }

    /// # 指定月の祝日を取得
    /// ```rust
    /// use jpholiday::jpholiday::JPHoliday;
    /// use jpholiday::chrono::{NaiveDate};
    /// use std::borrow::Borrow;
    ///
    /// let jpholiday = JPHoliday::new();
    ///
    /// assert_eq!(
    ///     jpholiday.month_holidays(2017, 5),
    ///     vec![
    ///         (NaiveDate::from_ymd(2017, 5, 3), "憲法記念日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 5, 4), "みどりの日".to_string()),
    ///         (NaiveDate::from_ymd(2017, 5, 5), "こどもの日".to_string())
    ///     ]
    /// );
    /// ```
    pub fn month_holidays(&self, year: i32, month: u32) -> Vec<(NaiveDate, String)> {
        let mut date: NaiveDate = NaiveDate::from_ymd(year.clone(), month.clone(), 1);
        let mut result: Vec<(NaiveDate, String)> = Vec::new();

        loop {
            if self.is_holiday(&date) {
                result.push((date.clone(), self.is_holiday_name(&date).unwrap()));
            }

            date = date.add(Duration::days(1));

            if &date.month() != &month {
                break;
            }
        }

        return result;
    }
}
