use chrono::{Datelike, NaiveDate, Weekday, Duration};
use std::ops::{RangeTo, Add};
use ::lib::jpholiday::JPHoliday;
use std::borrow::Borrow;


fn main() {
    let a = JPHoliday::new();
    let date = NaiveDate::from_ymd(2019, 2, 11);
    let date2 = NaiveDate::from_ymd(2019, 1, 1);
    // println!("{}", a.is_holiday_name(&date).unwrap());
    // println!("{}", a.is_holiday_name(&date2).unwrap());
    // println!("{}", a.is_holiday_name(&date).unwrap());
    // println!("{}", a.is_holiday_name(&date2).unwrap());
    // for ttt in a.between(NaiveDate::from_ymd(2019, 1, 1).borrow(), NaiveDate::from_ymd(2019, 10, 11).borrow()) {
    //     println!("{:?}", ttt);
    // }
    for ttt in a.year_holidays(2000) {
        println!("{:?}", ttt);
    }
    // for ttt in a.month_holidays(2020, 1) {
    //     println!("{:?}", ttt);
    // }

    // let datew = NaiveDate::from_ymd(2020, 1, 11);
    // println!("{:?}", week_day(&datew, 1, Weekday::Mon));
}
