//! エラー型。
//!
//! 不正な引数型による実行時エラーは型システムによりコンパイル時に排除されるため、
//! 本モジュールでは実在しない日付を生成しようとした場合のエラーのみを定義します。

use std::fmt;

/// 日付の生成に失敗したことを表すエラー。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateError {
    /// 実在しない年月日が指定された。
    InvalidDate {
        /// 指定された年。
        year: i32,
        /// 指定された月。
        month: u32,
        /// 指定された日。
        day: u32,
    },
}

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateError::InvalidDate { year, month, day } => {
                write!(f, "invalid date: {year:04}-{month:02}-{day:02}")
            }
        }
    }
}

impl std::error::Error for DateError {}
