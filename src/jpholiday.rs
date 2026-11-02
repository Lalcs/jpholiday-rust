//! 祝日判定の中核となる [`JPHoliday`] 型。

use crate::checker::{OriginalHolidayChecker, compute_holidays};
use crate::date::{Date, days_inclusive, month_bounds, year_bounds};
use crate::model::Holiday;
use crate::registry::HolidayCheckerRegistry;
use std::cell::RefCell;
use std::collections::HashMap;

/// 日本の祝日を判定するクラス。
///
/// 判定結果は日付単位でキャッシュされます。
/// キャッシュは問い合わせた日付ごとに増えるため、多数の日付を問い合わせる場合はインスタンスを
/// drop することでまとめて解放できます（グローバル関数 API はキャッシュを持ちません）。
///
/// # Examples
/// ```
/// use jpholiday::{Date, JPHoliday};
///
/// let jp = JPHoliday::new();
/// assert!(jp.is_holiday(Date::new(2017, 1, 1).unwrap()));
/// assert_eq!(
///     jp.is_holiday_name(Date::new(2017, 1, 2).unwrap()).as_deref(),
///     Some("元日 振替休日")
/// );
/// ```
pub struct JPHoliday {
    registry: HolidayCheckerRegistry,
    cache: RefCell<HashMap<Date, Vec<Holiday>>>,
}

impl JPHoliday {
    /// 既定のチェッカーを備えた新しいインスタンスを生成します。
    pub fn new() -> Self {
        JPHoliday {
            registry: HolidayCheckerRegistry::new(),
            cache: RefCell::new(HashMap::new()),
        }
    }

    /// その日に該当するすべての祝日を返します。
    pub fn holidays(&self, date: Date) -> Vec<Holiday> {
        if let Some(cached) = self.cache.borrow().get(&date) {
            return cached.clone();
        }

        let result = compute_holidays(self.registry.checkers(), date);

        self.cache.borrow_mut().insert(date, result.clone());
        result
    }

    /// その日が祝日かどうかを返します。
    pub fn is_holiday(&self, date: Date) -> bool {
        !self.holidays(date).is_empty()
    }

    /// その日の祝日名を返します（複数該当する場合は先頭、該当しなければ `None`）。
    pub fn is_holiday_name(&self, date: Date) -> Option<String> {
        self.holidays(date).into_iter().next().map(|h| h.name)
    }

    /// その年のすべての祝日を返します。
    pub fn year_holidays(&self, year: i32) -> Vec<Holiday> {
        let (start, end) = year_bounds(year);
        self.between(start, end)
    }

    /// その月のすべての祝日を返します。`month` が範囲外なら空を返します。
    pub fn month_holidays(&self, year: i32, month: u32) -> Vec<Holiday> {
        match month_bounds(year, month) {
            Some((start, end)) => self.between(start, end),
            None => Vec::new(),
        }
    }

    /// 指定範囲（両端を含む）のすべての祝日を返します。
    ///
    /// `start > end` の場合は空を返します。
    ///
    /// # 計算量・メモリに関する注意
    ///
    /// 処理時間は範囲内の**日数に比例**します（1 日あたり数マイクロ秒、release ビルドの目安）。
    /// さらにこのインスタンスは日付ごとに判定結果をキャッシュするため、**メモリ使用量も範囲内の
    /// 日数に比例して増えます**。`i32::MIN` 年〜`i32::MAX` 年のような極端に広い範囲を渡すと、
    /// 事実上終了せず CPU とメモリを使い尽くします。外部からの入力など信頼できない値を
    /// `start`/`end` にそのまま渡さず、呼び出し側で期間の上限を検証してください。
    /// 広い範囲を一度だけ走査したいだけなら、キャッシュを持たない関数 API
    /// [`crate::between`] が使えます（時間は同様に範囲に比例します）。
    pub fn between(&self, start: Date, end: Date) -> Vec<Holiday> {
        days_inclusive(start, end)
            .flat_map(|date| self.holidays(date))
            .collect()
    }

    /// 独自の祝日チェッカーを登録します。
    ///
    /// 同一型のチェッカーが既に登録されている場合は何もしません。
    pub fn register<C: OriginalHolidayChecker + 'static>(&mut self, checker: C) {
        self.cache.borrow_mut().clear();
        self.registry.register(checker);
    }

    /// 指定型の独自祝日チェッカーを登録解除します。
    ///
    /// 登録解除は型パラメータで指定します（例: `jp.unregister::<MyHoliday>()`）。
    pub fn unregister<C: OriginalHolidayChecker + 'static>(&mut self) {
        self.cache.borrow_mut().clear();
        self.registry.unregister::<C>();
    }
}

impl Default for JPHoliday {
    fn default() -> Self {
        Self::new()
    }
}
