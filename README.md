# JPHoliday
[![image](https://img.shields.io/crates/v/jpholiday)](https://crates.io/crates/jpholiday)
[![image](https://docs.rs/jpholiday/badge.svg)](https://docs.rs/jpholiday/)
[![codecov](https://codecov.io/gh/Lalcs/jpholiday-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/Lalcs/jpholiday-rust)

日本の祝日を取得するライブラリ

## Usage
`Cargo.toml`に追記
```toml
[dependencies]
jpholiday = "0.1.3"
```

## Sample Code

### 指定日の祝日名を取得
```rust
use jpholiday::jpholiday::JPHoliday;
use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

assert_eq!(
    jpholiday.is_holiday_name(NaiveDate::from_ymd(2017, 1, 1).borrow()).unwrap(),
    "元日".to_string()
);

assert_eq!(
    jpholiday.is_holiday_name(NaiveDate::from_ymd(2017, 1, 2).borrow()).unwrap(),
    "元日 振替休日".to_string()
);

assert_eq!(
    jpholiday.is_holiday_name(NaiveDate::from_ymd(2017, 1, 3).borrow()),
    None
);
```

### 指定日が祝日か判定
```rust
use jpholiday::jpholiday::JPHoliday;
use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

assert_eq!(
    jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 1).borrow()),
    true
);

assert_eq!(
    jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 2).borrow()),
    true
);

assert_eq!(
    jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 3).borrow()),
    false
);
```

### 指定年の祝日を取得
```rust
use jpholiday::jpholiday::JPHoliday;
use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

assert_eq!(
    jpholiday.year_holidays(2017),
    vec![
        (NaiveDate::from_ymd(2017, 1, 1), "元日".to_string()),
        (NaiveDate::from_ymd(2017, 1, 2), "元日 振替休日".to_string()),
        (NaiveDate::from_ymd(2017, 1, 9), "成人の日".to_string()),
        (NaiveDate::from_ymd(2017, 2, 11), "建国記念の日".to_string()),
        (NaiveDate::from_ymd(2017, 3, 20), "春分の日".to_string()),
        (NaiveDate::from_ymd(2017, 4, 29), "昭和の日".to_string()),
        (NaiveDate::from_ymd(2017, 5, 3), "憲法記念日".to_string()),
        (NaiveDate::from_ymd(2017, 5, 4), "みどりの日".to_string()),
        (NaiveDate::from_ymd(2017, 5, 5), "こどもの日".to_string()),
        (NaiveDate::from_ymd(2017, 7, 17), "海の日".to_string()),
        (NaiveDate::from_ymd(2017, 8, 11), "山の日".to_string()),
        (NaiveDate::from_ymd(2017, 9, 18), "敬老の日".to_string()),
        (NaiveDate::from_ymd(2017, 9, 23), "秋分の日".to_string()),
        (NaiveDate::from_ymd(2017, 10, 9), "体育の日".to_string()),
        (NaiveDate::from_ymd(2017, 11, 3), "文化の日".to_string()),
        (NaiveDate::from_ymd(2017, 11, 23), "勤労感謝の日".to_string()),
        (NaiveDate::from_ymd(2017, 12, 23), "天皇誕生日".to_string())
    ]
);
```

### 指定月の祝日を取得
```rust
use jpholiday::jpholiday::JPHoliday;
use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

assert_eq!(
    jpholiday.month_holidays(2017, 5),
    vec![
        (NaiveDate::from_ymd(2017, 5, 3), "憲法記念日".to_string()),
        (NaiveDate::from_ymd(2017, 5, 4), "みどりの日".to_string()),
        (NaiveDate::from_ymd(2017, 5, 5), "こどもの日".to_string())
    ]
);
```

### 指定範囲の祝日を取得
```rust
use jpholiday::jpholiday::JPHoliday;
use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

assert_eq!(
    jpholiday.between(NaiveDate::from_ymd(2017, 1, 1).borrow(), NaiveDate::from_ymd(2017, 5, 3).borrow()),
    vec![
        (NaiveDate::from_ymd(2017, 1, 1), "元日".to_string()),
        (NaiveDate::from_ymd(2017, 1, 2), "元日 振替休日".to_string()),
        (NaiveDate::from_ymd(2017, 1, 9), "成人の日".to_string()),
        (NaiveDate::from_ymd(2017, 2, 11), "建国記念の日".to_string()),
        (NaiveDate::from_ymd(2017, 3, 20), "春分の日".to_string()),
        (NaiveDate::from_ymd(2017, 4, 29), "昭和の日".to_string()),
        (NaiveDate::from_ymd(2017, 5, 3), "憲法記念日".to_string())
    ]
);
```
