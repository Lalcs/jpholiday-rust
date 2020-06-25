# JPHoliday
日本の祝日を取得するライブラリ

## Usage
`Cargo.toml`に追記
```toml
[dependencies]
jpholiday = "0.1.0"
```

## Sample Code

### 指定日の祝日名を取得
```rust
use jpholiday::jpholiday::JPHoliday;
use chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

jpholiday.is_holiday_name(NaiveDate::from_ymd(2017, 1, 1).borrow())
> "元日"
jpholiday.is_holiday_name(NaiveDate::from_ymd(2017, 1, 2).borrow())
> "元日 振替休日"
jpholiday.is_holiday_name(NaiveDate::from_ymd(2017, 1, 3).borrow())
> None
```

### 指定日が祝日か判定
```rust
use jpholiday::jpholiday::JPHoliday;
use chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 1).borrow())
> true
jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 2).borrow())
> true
jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 3).borrow())
> false
```

### 指定年の祝日を取得
```rust
use jpholiday::jpholiday::JPHoliday;
use chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

jpholiday.year_holidays(2017)
>Vec((NaiveDate(2017, 1, 1), "元日"),
 (NaiveDate(2017, 1, 2), "元日 振替休日"),
 (NaiveDate(2017, 1, 9), "成人の日"),
 (NaiveDate(2017, 2, 11), "建国記念の日"),
 (NaiveDate(2017, 3, 20), "春分の日"),
 (NaiveDate(2017, 4, 29), "昭和の日"),
 (NaiveDate(2017, 5, 3), "憲法記念日"),
 (NaiveDate(2017, 5, 4), "みどりの日"),
 (NaiveDate(2017, 5, 5), "こどもの日"),
 (NaiveDate(2017, 7, 17), "海の日"),
 (NaiveDate(2017, 8, 11), "山の日"),
 (NaiveDate(2017, 9, 18), "敬老の日"),
 (NaiveDate(2017, 9, 23), "秋分の日"),
 (NaiveDate(2017, 10, 9), "体育の日"),
 (NaiveDate(2017, 11, 3), "文化の日"),
 (NaiveDate(2017, 11, 23), "勤労感謝の日"),
 (NaiveDate(2017, 12, 23), "天皇誕生日"))
```
### 指定月の祝日を取得
```rust
use jpholiday::jpholiday::JPHoliday;
use chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

jpholiday.month_holidays(2017, 5)
>Vec((NaiveDate(2017, 5, 3), "憲法記念日"),
 (NaiveDate(2017, 5, 4), "みどりの日"),
 (NaiveDate(2017, 5, 5), "こどもの日"))
```

### 指定範囲の祝日を取得
```rust
use jpholiday::jpholiday::JPHoliday;
use chrono::{NaiveDate};
use std::borrow::Borrow;

let jpholiday = JPHoliday::new();

jpholiday.between(NaiveDate::from_ymd(2017, 1, 1).borrow(), NaiveDate::from_ymd(2017, 5, 3).borrow())
>Vec((NaiveDate(2017, 1, 1), "元日"),
 (NaiveDate(2017, 1, 2), "元日 振替休日"),
 (NaiveDate(2017, 1, 9), "成人の日"),
 (NaiveDate(2017, 2, 11), "建国記念の日"),
 (NaiveDate(2017, 3, 20), "春分の日"),
 (NaiveDate(2017, 4, 29), "昭和の日"),
 (NaiveDate(2017, 5, 3), "憲法記念日"))
```
