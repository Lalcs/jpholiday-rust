//! 祝日チェッカーのレジストリ。
//!
//! 組込み祝日を既定順で保持し、末尾に振替休日・国民の休日を置きます。
//! 独自祝日は型単位で重複排除しつつ末尾に追加します。

use crate::checker::{Builtin, Checker, OriginalEntry, OriginalHolidayChecker};
use std::any::TypeId;
use std::sync::Arc;

/// チェッカーの並びを保持するレジストリ。
pub(crate) struct HolidayCheckerRegistry {
    checkers: Vec<Checker>,
}

impl HolidayCheckerRegistry {
    /// 既定のレジストリを生成します（組込み祝日 → 振替休日 → 国民の休日）。
    pub(crate) fn new() -> Self {
        let mut checkers: Vec<Checker> =
            Builtin::ALL.iter().map(|b| Checker::Builtin(*b)).collect();
        checkers.push(Checker::Transfer);
        checkers.push(Checker::National);
        HolidayCheckerRegistry { checkers }
    }

    /// 登録済みチェッカーの並びを返します。
    pub(crate) fn checkers(&self) -> &[Checker] {
        &self.checkers
    }

    /// チェッカー一覧の複製（スナップショット）を返します。
    ///
    /// グローバル API はこのスナップショットをロックの外に持ち出して祝日を計算するため、
    /// ユーザー実装の実行中にロックを保持しません。複製は組込みチェッカーの単純コピーと
    /// 独自チェッカーの `Arc` 参照カウント増加のみで安価です。
    pub(crate) fn snapshot(&self) -> Vec<Checker> {
        self.checkers.clone()
    }

    /// 独自チェッカーを登録します。同一型が既に登録済みなら何もしません。
    pub(crate) fn register<C: OriginalHolidayChecker + 'static>(&mut self, checker: C) {
        let type_id = TypeId::of::<C>();
        if self
            .checkers
            .iter()
            .any(|c| matches!(c, Checker::Original(e) if e.type_id == type_id))
        {
            return;
        }
        self.checkers.push(Checker::Original(OriginalEntry {
            type_id,
            checker: Arc::new(checker),
        }));
    }

    /// 指定型の独自チェッカーをすべて登録解除します。
    pub(crate) fn unregister<C: OriginalHolidayChecker + 'static>(&mut self) {
        let type_id = TypeId::of::<C>();
        self.checkers
            .retain(|c| !matches!(c, Checker::Original(e) if e.type_id == type_id));
    }
}
