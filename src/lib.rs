//! # jpholiday
//!
//! 日本の祝日を判定する依存ゼロの Rust ライブラリです。
//! [内閣府が公表しているデータ](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html)
//! に基づき、振替休日・国民の休日・春分/秋分の日（天文計算）まで再現します。
//!
//! **2027 年**までの祝日は公式発表に基づいて動作確認済みです。それ以降も取得できますが、
//! 正式な公表がないため正確性は保証されません。
//!
//! ## 関数 API
//!
//! ```
//! use jpholiday::Date;
//!
//! // 指定日が祝日か
//! assert!(jpholiday::is_holiday(Date::new(2017, 1, 1).unwrap()));
//! assert!(!jpholiday::is_holiday(Date::new(2017, 1, 3).unwrap()));
//!
//! // 祝日名（振替休日も含む）
//! assert_eq!(
//!     jpholiday::is_holiday_name(Date::new(2017, 1, 2).unwrap()).as_deref(),
//!     Some("元日 振替休日")
//! );
//! assert_eq!(jpholiday::is_holiday_name(Date::new(2017, 1, 3).unwrap()), None);
//!
//! // その年・月・範囲の祝日一覧
//! let year = jpholiday::year_holidays(2017);
//! assert_eq!(year[0], (Date::new(2017, 1, 1).unwrap(), "元日".to_string()));
//! ```
//!
//! ## クラス API
//!
//! ```
//! use jpholiday::{Date, JPHoliday};
//!
//! let jp = JPHoliday::new();
//! let holidays = jp.holidays(Date::new(2020, 1, 1).unwrap());
//! assert_eq!(holidays[0].name, "元日");
//! ```
//!
//! ## 独自の祝日を追加する
//!
//! ```
//! use jpholiday::{Date, JPHoliday, OriginalHolidayChecker};
//!
//! struct CompanyHoliday;
//! impl OriginalHolidayChecker for CompanyHoliday {
//!     fn is_holiday(&self, date: Date) -> bool {
//!         date == Date::new(2020, 2, 9).unwrap()
//!     }
//!     fn holiday_name(&self, _date: Date) -> String {
//!         "特別休暇".to_string()
//!     }
//! }
//!
//! let mut jp = JPHoliday::new();
//! jp.register(CompanyHoliday);
//! assert!(jp.is_holiday(Date::new(2020, 2, 9).unwrap()));
//! jp.unregister::<CompanyHoliday>();
//! assert!(!jp.is_holiday(Date::new(2020, 2, 9).unwrap()));
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod astronomy;
mod checker;
mod date;
mod error;
mod jpholiday;
mod model;
mod registry;

pub use checker::OriginalHolidayChecker;
pub use date::{Date, days_in_month, is_leap_year};
pub use error::DateError;
pub use jpholiday::JPHoliday;
pub use model::Holiday;

use crate::checker::{Checker, compute_holidays};
use crate::registry::HolidayCheckerRegistry;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

/// 関数 API が共有するグローバルなチェッカーレジストリ。
///
/// スレッド安全性のため [`Mutex`] で保護しており、[`register`]/[`unregister`] による
/// 登録状態は全関数で共有されます。
///
/// グローバル状態として保持するのは「レジストリ（チェッカーの並び）」のみで、祝日計算の結果は
/// キャッシュしません。これにより、範囲問い合わせ（[`between`] 等）でプロセス寿命のキャッシュが
/// 際限なく増えること（メモリ枯渇）を防ぎます。日付単位のキャッシュが必要なら [`JPHoliday`]
/// インスタンスを生成して利用してください（インスタンスを drop すればキャッシュも解放されます）。
fn registry() -> &'static Mutex<HolidayCheckerRegistry> {
    static INSTANCE: OnceLock<Mutex<HolidayCheckerRegistry>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(HolidayCheckerRegistry::new()))
}

/// レジストリのロックを取得します。
///
/// 独自チェッカー（[`OriginalHolidayChecker`]）が panic してもロックは保持していない
/// （[`snapshot`] が即座に解放する）ため、poison は通常発生しませんが、万一に備えて回復します。
fn locked() -> MutexGuard<'static, HolidayCheckerRegistry> {
    registry().lock().unwrap_or_else(PoisonError::into_inner)
}

/// レジストリのスナップショット（チェッカー一覧の複製）を取得し、直ちにロックを解放します。
///
/// ユーザー実装（[`OriginalHolidayChecker`]）はこのロックの**外**で実行されるため、独自
/// チェッカーがグローバル API（[`is_holiday`] 等）を再入呼び出ししても、同じ非再入 Mutex を
/// 二重取得することはなく、デッドロックしません。
fn snapshot() -> Vec<Checker> {
    locked().snapshot()
}

/// その日に該当するすべての祝日を返します。
pub fn holidays(date: Date) -> Vec<Holiday> {
    compute_holidays(&snapshot(), date)
}

/// その日が祝日かどうかを返します。
pub fn is_holiday(date: Date) -> bool {
    !holidays(date).is_empty()
}

/// その日の祝日名を返します（該当しなければ `None`）。
pub fn is_holiday_name(date: Date) -> Option<String> {
    holidays(date).into_iter().next().map(|h| h.name)
}

/// その年のすべての祝日を `(日付, 名称)` のタプルで返します。
pub fn year_holidays(year: i32) -> Vec<(Date, String)> {
    let checkers = snapshot();
    let mut out = Vec::new();
    let mut date = Date::new(year, 1, 1).expect("January 1st is always a valid date");
    while date.year() == year {
        for holiday in compute_holidays(&checkers, date) {
            out.push(holiday.into_tuple());
        }
        date = date.succ();
    }
    out
}

/// その月のすべての祝日を `(日付, 名称)` のタプルで返します。`month` が範囲外なら空を返します。
pub fn month_holidays(year: i32, month: u32) -> Vec<(Date, String)> {
    let checkers = snapshot();
    let mut out = Vec::new();
    let mut date = match Date::new(year, month, 1) {
        Ok(d) => d,
        Err(_) => return out,
    };
    while date.month() == month {
        for holiday in compute_holidays(&checkers, date) {
            out.push(holiday.into_tuple());
        }
        date = date.succ();
    }
    out
}

/// 指定範囲（両端を含む）のすべての祝日を `(日付, 名称)` のタプルで返します。
pub fn between(start: Date, end: Date) -> Vec<(Date, String)> {
    let checkers = snapshot();
    let mut out = Vec::new();
    let mut current = start;
    while current <= end {
        for holiday in compute_holidays(&checkers, current) {
            out.push(holiday.into_tuple());
        }
        current = current.succ();
    }
    out
}

/// 独自の祝日チェッカーをグローバル API に登録します。
///
/// 同一型が既に登録済みなら何もしません。
pub fn register<C: OriginalHolidayChecker + 'static>(checker: C) {
    locked().register(checker);
}

/// 指定型の独自祝日チェッカーをグローバル API から登録解除します。
///
/// 登録解除は型パラメータで指定します（例: `jpholiday::unregister::<MyHoliday>()`）。
pub fn unregister<C: OriginalHolidayChecker + 'static>() {
    locked().unregister::<C>();
}
