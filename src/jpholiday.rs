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

    pub fn is_holiday(&self, date: &NaiveDate) -> bool {
        for holiday in self.registry.get_registry() {
            if holiday.is_holiday(&date) {
                return true;
            }
        }

        return false;
    }

    pub fn is_holiday_name(&self, date: &NaiveDate) -> Option<String> {
        for holiday in self.registry.get_registry() {
            if holiday.is_holiday(&date) {
                return Some(holiday.is_holiday_name(&date).unwrap());
            }
        }

        return None;
    }

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
