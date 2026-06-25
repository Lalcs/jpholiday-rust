# jpholiday (Rust)

[![CI](https://github.com/Lalcs/jpholiday/actions/workflows/ci.yml/badge.svg)](https://github.com/Lalcs/jpholiday/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

日本の祝日を判定する**依存ゼロ**の Rust ライブラリです。Python 製
[jpholiday](https://github.com/Lalcs/jpholiday) を忠実に移植したもので、
[内閣府が公表しているデータ](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html)
に基づき、**振替休日**・**国民の休日**・**春分/秋分の日（天文計算）**まで再現します。

A dependency-free Rust library for determining Japanese public holidays — a faithful
port of the Python [jpholiday](https://github.com/Lalcs/jpholiday). It reproduces
substitute holidays (振替休日), citizens' holidays (国民の休日), and the
astronomically-computed spring/autumn equinox days.

> **2027 年**までの祝日は内閣府の公式発表に基づき動作確認済みです。それ以降も取得できますが、
> 正式な公表がないため正確性は保証されません。
>
> Holidays up to **2027** are verified against the official announcement. Later dates are
> computed but not officially confirmed.

## ✨ 特徴 / Features

- 🦀 **依存ゼロ** — 標準ライブラリのみ。`chrono` や `time` を必要としません。
- 📅 **独自の `Date` 型** — 先発グレゴリオ暦に基づく軽量な日付型を内蔵。
- 🌸 **天文計算** — 春分・秋分の日を太陽黄経から算出（1948〜3000 年で ±1 日）。
- 🔁 **振替休日・国民の休日** — 祝日法の年代別ロジックを完全再現。
- 🧩 **独自祝日の登録** — `OriginalHolidayChecker` トレイトで拡張可能。
- ✅ **1,000 件超のゴールデンテスト** — 本家の 1971〜2027 年テストを移植して検証。

## 📦 インストール / Installation

```toml
# Cargo.toml
[dependencies]
jpholiday-rust = "0.1"
```

クレート名は `jpholiday-rust`、ライブラリ名は `jpholiday` です。コードでは `jpholiday::` で参照します。

```rust
use jpholiday::Date;
```

## 🚀 使い方 / Usage

### 関数 API / Function API

```rust
use jpholiday::Date;

// 指定日が祝日か / Is it a holiday?
assert!(jpholiday::is_holiday(Date::new(2017, 1, 1).unwrap()));
assert!(!jpholiday::is_holiday(Date::new(2017, 1, 3).unwrap()));

// 祝日名（振替休日も含む）/ Holiday name (incl. substitute holidays)
assert_eq!(
    jpholiday::is_holiday_name(Date::new(2017, 1, 2).unwrap()).as_deref(),
    Some("元日 振替休日")
);
assert_eq!(jpholiday::is_holiday_name(Date::new(2017, 1, 3).unwrap()), None);

// その年の祝日一覧 / All holidays in a year
for (date, name) in jpholiday::year_holidays(2017) {
    println!("{date} {name}");
}

// その月の祝日一覧 / All holidays in a month
let may = jpholiday::month_holidays(2017, 5);
assert_eq!(may.len(), 3);

// 指定範囲（両端含む）/ Within a date range (inclusive)
let golden_week = jpholiday::between(
    Date::new(2017, 5, 1).unwrap(),
    Date::new(2017, 5, 5).unwrap(),
);
```

### クラス API / Class API

```rust
use jpholiday::{Date, JPHoliday};

let jp = JPHoliday::new();

let holidays = jp.holidays(Date::new(2020, 1, 1).unwrap());
assert_eq!(holidays[0].name, "元日");

assert!(jp.is_holiday(Date::new(2020, 1, 1).unwrap()));
let _ = jp.year_holidays(2020);
let _ = jp.month_holidays(2020, 5);
let _ = jp.between(Date::new(2020, 1, 1).unwrap(), Date::new(2020, 12, 31).unwrap());
```

### 独自の祝日を追加 / Custom holidays

```rust
use jpholiday::{Date, JPHoliday, OriginalHolidayChecker};

struct CompanyHoliday;

impl OriginalHolidayChecker for CompanyHoliday {
    fn is_holiday(&self, date: Date) -> bool {
        date == Date::new(2020, 2, 9).unwrap()
    }
    fn holiday_name(&self, _date: Date) -> String {
        "特別休暇".to_string()
    }
}

let mut jp = JPHoliday::new();
jp.register(CompanyHoliday);
assert!(jp.is_holiday(Date::new(2020, 2, 9).unwrap()));

// 登録解除は型パラメータで指定 / Unregister by type parameter
jp.unregister::<CompanyHoliday>();
assert!(!jp.is_holiday(Date::new(2020, 2, 9).unwrap()));
```

グローバル関数版の `jpholiday::register` / `jpholiday::unregister` も利用できます。

## 🔁 Python 版との違い / Differences from the Python version

| 項目 | Python | Rust |
|---|---|---|
| 日付型 | `datetime.date` / `datetime.datetime` | 内蔵の `jpholiday::Date`（依存ゼロ） |
| 型エラー | 実行時に `JPHolidayTypeError` | 型システムでコンパイル時に排除 |
| 無効な日付 | 例外 | `Date::new` が `Result<Date, DateError>` |
| `unregister` | インスタンスを渡す | 型パラメータ `unregister::<T>()` |
| `year_holidays` 等（関数 API） | `list[tuple[date, str]]` | `Vec<(Date, String)>` |
| `holidays` / クラス API | `list[Holiday]` | `Vec<Holiday>` |
| スレッド安全性 | 実質シングルスレッド | 関数 API は `Mutex` で保護（スレッド安全） |

`datetime.datetime`（時刻付き）は本ライブラリには存在しません。時刻が不要なため、すべて `Date` で扱います。

## 📜 対応する祝日 / Supported holidays

元日・成人の日・建国記念の日・天皇誕生日・春分の日・みどりの日・昭和の日・憲法記念日・
こどもの日・海の日・山の日・敬老の日・秋分の日・体育の日・スポーツの日・文化の日・勤労感謝の日、
および振替休日・国民の休日。さらに皇室慶弔行事に伴う特別な祝日（1959/1989/1990/1993/2019）と、
2020/2021 年の五輪特例も再現します。

## 🧪 開発 / Development

```bash
cargo test            # 全テスト（ユニット + ドキュメント + ゴールデン統合テスト）
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run --example basic
```

## 📄 ライセンス / License

MIT License. 本家 Python 版 [jpholiday](https://github.com/Lalcs/jpholiday)（MIT）に基づきます。
詳細は [LICENSE](./LICENSE) を参照してください。
