//! 祝日を表すモデル。

use crate::date::Date;

/// 1 件の祝日（日付と名称）を表す不変の値。
///
/// 本家 Python 版の `Holiday`（frozen dataclass）に対応します。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Holiday {
    /// 祝日の日付。
    pub date: Date,
    /// 祝日の名称（例: `"元日"`、`"元日 振替休日"`、`"国民の休日"`）。
    pub name: String,
}

impl Holiday {
    /// 日付と名称から祝日を生成します。
    pub fn new(date: Date, name: impl Into<String>) -> Self {
        Holiday {
            date,
            name: name.into(),
        }
    }

    /// `(date, name)` のタプルへ変換します（参照を保持したまま複製）。
    ///
    /// 本家 Python 版の `Holiday.to_tuple()` に対応します。
    pub fn to_tuple(&self) -> (Date, String) {
        (self.date, self.name.clone())
    }

    /// 所有権を消費して `(date, name)` のタプルへ変換します。
    pub fn into_tuple(self) -> (Date, String) {
        (self.date, self.name)
    }
}
