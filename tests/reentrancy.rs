//! グローバル関数 API が「独自チェッカーからの再入呼び出し」でデッドロックしないことを
//! 検証する回帰テスト（Codex アドバーサリアルレビューの指摘に対応）。
//!
//! 旧設計ではグローバル `Mutex` を祝日計算の間ずっと保持していたため、独自チェッカーが
//! `jpholiday::is_holiday` 等を呼ぶと同じ非再入 Mutex を二重取得して永久待ちになっていた。
//! 現設計はレジストリのスナップショットを取得後すぐロックを解放し、計算はロックの外で行う。
//!
//! グローバル状態を変更するため、競合を避けてこのバイナリ内のテストは 1 つだけにしている。

use jpholiday::{Date, OriginalHolidayChecker};

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

/// 特定日でのみ、グローバル API を再入呼び出しする独自チェッカー。
///
/// 再入先は自身が特別扱いしない日付（元日）なので、無限再帰にはならない。
struct ReentrantChecker;

impl OriginalHolidayChecker for ReentrantChecker {
    fn is_holiday(&self, date: Date) -> bool {
        if date == d(2098, 6, 6) {
            // 計算の最中にグローバル API を再入呼び出し。旧設計ではここでデッドロックした。
            return jpholiday::is_holiday(d(2017, 1, 1)); // 元日 -> true
        }
        false
    }
    fn holiday_name(&self, _date: Date) -> String {
        "再入テスト".to_string()
    }
}

#[test]
fn global_api_does_not_deadlock_on_reentrant_checker() {
    jpholiday::register(ReentrantChecker);

    // このテストが（ハングせず）完了すること自体がデッドロック解消の証拠。
    // ReentrantChecker は 2098-06-06 で is_holiday(2017-01-01)=true を返すため、true になる。
    assert!(jpholiday::is_holiday(d(2098, 6, 6)));

    // 再入を伴わない通常の判定も従来どおり動作する。
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 1, 1)).as_deref(),
        Some("元日")
    );
    assert!(!jpholiday::is_holiday(d(2017, 1, 3)));

    // 範囲問い合わせ中の各日でも再入が起こり得るが、デッドロックしない。
    let names: Vec<String> = jpholiday::between(d(2098, 6, 5), d(2098, 6, 7))
        .into_iter()
        .map(|(_, name)| name)
        .collect();
    assert!(names.iter().any(|n| n == "再入テスト"));

    jpholiday::unregister::<ReentrantChecker>();
    assert!(!jpholiday::is_holiday(d(2098, 6, 6)));
}
