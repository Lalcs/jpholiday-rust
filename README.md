# JPHoliday

[![crates.io](https://img.shields.io/crates/v/jpholiday.svg)](https://crates.io/crates/jpholiday)
[![docs.rs](https://docs.rs/jpholiday/badge.svg)](https://docs.rs/jpholiday)
[![License: MIT](https://img.shields.io/crates/l/jpholiday.svg)](./LICENSE)
[![CI](https://github.com/Lalcs/jpholiday-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/Lalcs/jpholiday-rust/actions/workflows/ci.yml)

このライブラリは、[内閣府](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html)
が公表しているデータを基に、日本の国民の祝日を簡単に取得できるようにしたものです。
標準ライブラリのみで動作し、外部クレートには一切依存しません。
**2027年**までの祝日は公式発表された内容に基づいて動作確認済みです。
それ以降についても取得は可能ですが、内閣府からの正式な公表がないため、正確性は保証されません。

## Installation

`Cargo.toml` に追記:

```toml
[dependencies]
jpholiday = "0.2"
```

## Class

### 指定日の祝日名を取得

```rust
use jpholiday::{Date, JPHoliday};

let jpholiday = JPHoliday::new();

jpholiday.holidays(Date::new(2017, 1, 1).unwrap());
// => [Holiday { date: 2017-01-01, name: "元日" }]

jpholiday.holidays(Date::new(2017, 1, 2).unwrap());
// => [Holiday { date: 2017-01-02, name: "元日 振替休日" }]

jpholiday.holidays(Date::new(2017, 1, 3).unwrap());
// => []
```

### 指定日が祝日か判定

```rust
use jpholiday::{Date, JPHoliday};

let jpholiday = JPHoliday::new();

jpholiday.is_holiday(Date::new(2017, 1, 1).unwrap());
// => true
jpholiday.is_holiday(Date::new(2017, 1, 2).unwrap());
// => true
jpholiday.is_holiday(Date::new(2017, 1, 3).unwrap());
// => false
```

### 指定年の祝日を取得

```rust
use jpholiday::{Date, JPHoliday};

let jpholiday = JPHoliday::new();

jpholiday.year_holidays(2017);
// => [
//   Holiday { date: 2017-01-01, name: "元日" },
//   Holiday { date: 2017-01-02, name: "元日 振替休日" },
//   Holiday { date: 2017-01-09, name: "成人の日" },
//   ...
// ]
```

### 指定月の祝日を取得

```rust
use jpholiday::{Date, JPHoliday};

let jpholiday = JPHoliday::new();

jpholiday.month_holidays(2017, 5);
// => [
//   Holiday { date: 2017-05-03, name: "憲法記念日" },
//   Holiday { date: 2017-05-04, name: "みどりの日" },
//   Holiday { date: 2017-05-05, name: "こどもの日" },
// ]
```

### 指定範囲の祝日を取得

```rust
use jpholiday::{Date, JPHoliday};

let jpholiday = JPHoliday::new();

jpholiday.between(
    Date::new(2017, 1, 1).unwrap(),
    Date::new(2017, 5, 3).unwrap(),
);
// => [
//   Holiday { date: 2017-01-01, name: "元日" },
//   Holiday { date: 2017-01-02, name: "元日 振替休日" },
//   ...
//   Holiday { date: 2017-05-03, name: "憲法記念日" },
// ]
```

### 独自の休日を追加

```rust
use jpholiday::{Date, JPHoliday, OriginalHolidayChecker};

struct TestHoliday;

impl OriginalHolidayChecker for TestHoliday {
    fn is_holiday(&self, date: Date) -> bool {
        date == Date::new(2020, 2, 9).unwrap()
    }
    fn holiday_name(&self, _date: Date) -> String {
        "特別休暇".to_string()
    }
}

let mut jpholiday = JPHoliday::new();
jpholiday.register(TestHoliday);

jpholiday.holidays(Date::new(2020, 2, 9).unwrap());
// => [Holiday { date: 2020-02-09, name: "特別休暇" }]

jpholiday.is_holiday(Date::new(2020, 2, 9).unwrap());
// => true
```

### 独自の休日を削除

```rust
use jpholiday::{Date, JPHoliday, OriginalHolidayChecker};

struct TestHoliday;

impl OriginalHolidayChecker for TestHoliday {
    fn is_holiday(&self, date: Date) -> bool {
        date == Date::new(2020, 2, 9).unwrap()
    }
    fn holiday_name(&self, _date: Date) -> String {
        "特別休暇".to_string()
    }
}

let mut jpholiday = JPHoliday::new();
jpholiday.register(TestHoliday);

// 登録解除は型パラメータで指定
jpholiday.unregister::<TestHoliday>();

jpholiday.holidays(Date::new(2020, 2, 9).unwrap());
// => []

jpholiday.is_holiday(Date::new(2020, 2, 9).unwrap());
// => false
```

## Functions

### 指定日の祝日名を取得

```rust
use jpholiday::Date;

jpholiday::is_holiday_name(Date::new(2017, 1, 1).unwrap());
// => Some("元日")
jpholiday::is_holiday_name(Date::new(2017, 1, 2).unwrap());
// => Some("元日 振替休日")
jpholiday::is_holiday_name(Date::new(2017, 1, 3).unwrap());
// => None
```

### 指定日が祝日か判定

```rust
use jpholiday::Date;

jpholiday::is_holiday(Date::new(2017, 1, 1).unwrap());
// => true
jpholiday::is_holiday(Date::new(2017, 1, 2).unwrap());
// => true
jpholiday::is_holiday(Date::new(2017, 1, 3).unwrap());
// => false
```

### 指定年の祝日を取得

```rust
use jpholiday::Date;

jpholiday::year_holidays(2017);
// => [
//   (2017-01-01, "元日"),
//   (2017-01-02, "元日 振替休日"),
//   (2017-01-09, "成人の日"),
//   (2017-02-11, "建国記念の日"),
//   (2017-03-20, "春分の日"),
//   (2017-04-29, "昭和の日"),
//   (2017-05-03, "憲法記念日"),
//   (2017-05-04, "みどりの日"),
//   (2017-05-05, "こどもの日"),
//   (2017-07-17, "海の日"),
//   (2017-08-11, "山の日"),
//   (2017-09-18, "敬老の日"),
//   (2017-09-23, "秋分の日"),
//   (2017-10-09, "体育の日"),
//   (2017-11-03, "文化の日"),
//   (2017-11-23, "勤労感謝の日"),
//   (2017-12-23, "天皇誕生日"),
// ]
```

### 指定月の祝日を取得

```rust
use jpholiday::Date;

jpholiday::month_holidays(2017, 5);
// => [
//   (2017-05-03, "憲法記念日"),
//   (2017-05-04, "みどりの日"),
//   (2017-05-05, "こどもの日"),
// ]
```

### 指定範囲の祝日を取得

```rust
use jpholiday::Date;

jpholiday::between(
    Date::new(2017, 1, 1).unwrap(),
    Date::new(2017, 5, 3).unwrap(),
);
// => [
//   (2017-01-01, "元日"),
//   (2017-01-02, "元日 振替休日"),
//   (2017-01-09, "成人の日"),
//   (2017-02-11, "建国記念の日"),
//   (2017-03-20, "春分の日"),
//   (2017-04-29, "昭和の日"),
//   (2017-05-03, "憲法記念日"),
// ]
```

### 独自の休日を追加

```rust
use jpholiday::{Date, OriginalHolidayChecker};

struct TestHoliday;

impl OriginalHolidayChecker for TestHoliday {
    fn is_holiday(&self, date: Date) -> bool {
        date == Date::new(2020, 2, 9).unwrap()
    }
    fn holiday_name(&self, _date: Date) -> String {
        "特別休暇".to_string()
    }
}

jpholiday::register(TestHoliday);

jpholiday::is_holiday_name(Date::new(2020, 2, 9).unwrap());
// => Some("特別休暇")

jpholiday::is_holiday(Date::new(2020, 2, 9).unwrap());
// => true
```

### 独自の休日を削除

```rust
use jpholiday::{Date, OriginalHolidayChecker};

struct TestHoliday;

impl OriginalHolidayChecker for TestHoliday {
    fn is_holiday(&self, date: Date) -> bool {
        date == Date::new(2020, 2, 9).unwrap()
    }
    fn holiday_name(&self, _date: Date) -> String {
        "特別休暇".to_string()
    }
}

jpholiday::register(TestHoliday);

// 登録解除は型パラメータで指定
jpholiday::unregister::<TestHoliday>();

jpholiday::is_holiday_name(Date::new(2020, 2, 9).unwrap());
// => None

jpholiday::is_holiday(Date::new(2020, 2, 9).unwrap());
// => false
```

## License

MIT License. 詳細は [LICENSE](./LICENSE) を参照してください。
