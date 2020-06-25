use chrono::{Datelike, NaiveDate, Weekday, Duration};
use std::ops::{Add};

pub fn week_day(date: &NaiveDate, week: u8, week_day: Weekday) -> Result<NaiveDate, &str> {
    let lines: Vec<Vec<u8>> = monthcalendar(date.year(), date.month());
    let mut days: Vec<u8> = Vec::with_capacity(5);

    for line in lines {
        if line[week_day.num_days_from_sunday() as usize] == 0 {
            continue
        }

        days.push(line[week_day.num_days_from_sunday() as usize])
    }

    if days.len() < week as usize {
        return Err("Error: week")
    }

    return Ok(NaiveDate::from_ymd(date.year(), date.month(), days[(week-1) as usize] as u32));
}

fn monthcalendar(year: i32, month: u32) -> Vec<Vec<u8>> {
    let mut date: NaiveDate = NaiveDate::from_ymd(year, month, 1);
    let mut days: Vec<u8> = Vec::with_capacity(31);

    for _i in 0..date.weekday().number_from_monday() {
        days.push(0 as u8)
    }

    loop {
        days.push(date.day() as u8);

        date = date.add(Duration::days(1));

        if date.month() != month {
            break;
        }
    }

    let mut result: Vec<Vec<u8>> = Vec::with_capacity(5);

    for i in days.chunks(7) {
        result.push(i.to_vec());
    }

    return result;
}
