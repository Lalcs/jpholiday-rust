//! `astronomy` の公開関数が極端な入力でも契約を守ることの回帰テスト。
//!
//! - `julian_day` は極端な `i64` を渡しても、整数オーバーフローで panic（debug）したり、
//!   黙ってラップして誤った値を返したり（release）しない。
//! - `normalize_angle` は常に `[0, 360)` を返す。微小な負値では `angle + 360.0` が丸めで
//!   `360.0` ちょうどになるため、以前はドキュメントの契約に反して `360.0` を返していた。

use jpholiday::astronomy::{julian_day, normalize_angle};

/// 桁あふれを起こしうる境界を含む `i64` の代表値。
///
/// `i64::MAX - 4716` は `year + 4716` がちょうど `i64::MAX` になる値、`i64::MIN` は `year - 1` が
/// 桁あふれする値、`i64::MAX` は `month + 1` が桁あふれする値に当たる。
const EXTREMES: [i64; 11] = [
    i64::MIN,
    i64::MIN + 1,
    -4716,
    -1,
    0,
    1,
    4716,
    i64::MAX - 4717,
    i64::MAX - 4716,
    i64::MAX - 1,
    i64::MAX,
];

#[test]
fn julian_day_does_not_overflow_on_extreme_arguments() {
    // debug ビルドでは整数オーバーフローがここで panic する。
    for &y in &EXTREMES {
        for &m in &EXTREMES {
            for &d in &EXTREMES {
                let jd = julian_day(y, m, d, 12.0);
                assert!(jd.is_finite(), "julian_day({y}, {m}, {d}) = {jd}");
            }
        }
    }
}

#[test]
fn julian_day_keeps_its_sign_and_order_at_the_extremes() {
    // release ビルドでは桁あふれが黙ってラップし、符号や大小関係が壊れていた。
    let epoch = julian_day(2000, 6, 1, 0.0);
    assert!(julian_day(i64::MAX, 6, 1, 0.0) > epoch);
    assert!(julian_day(i64::MAX - 4716, 6, 1, 0.0) > epoch);
    assert!(julian_day(i64::MIN, 6, 1, 0.0) < epoch);
    assert!(julian_day(2000, i64::MAX, 1, 0.0) > epoch);
    assert!(julian_day(2000, 6, i64::MAX, 0.0) > epoch);
    assert!(julian_day(2000, 6, i64::MIN, 0.0) < epoch);
}

/// 修正前の `julian_day` と同じ整数演算による参照実装。通常の範囲での出力が
/// 1 ビットも変わらないこと（祝日判定への影響がないこと）を確かめるために使う。
fn julian_day_reference(year: i64, month: i64, day: i64, hour: f64) -> f64 {
    let (mut y, mut m) = (year, month);
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    let a = y.div_euclid(100);
    let b = 2 - a + a.div_euclid(4);

    (365.25 * (y + 4716) as f64).floor()
        + (30.6001 * (m + 1) as f64).floor()
        + day as f64
        + hour / 24.0
        + b as f64
        - 1524.5
}

#[test]
fn julian_day_is_bit_identical_for_ordinary_arguments() {
    for y in -5000..=5000_i64 {
        for m in 1..=12_i64 {
            for d in [1, 15, 28, 31] {
                for hour in [0.0, 12.0, 23.5] {
                    let expected = julian_day_reference(y, m, d, hour);
                    let actual = julian_day(y, m, d, hour);
                    assert_eq!(
                        actual.to_bits(),
                        expected.to_bits(),
                        "julian_day({y}, {m}, {d}, {hour})"
                    );
                }
            }
        }
    }
}

#[test]
fn normalize_angle_never_returns_360() {
    let tiny_negatives = [
        -1e-20,
        -1e-17,
        -1e-300,
        -f64::MIN_POSITIVE,
        -5e-324, // 最小の非正規化数
    ];
    for &angle in &tiny_negatives {
        let n = normalize_angle(angle);
        assert!(
            (0.0..360.0).contains(&n),
            "normalize_angle({angle:e}) = {n}"
        );
    }
}

#[test]
fn normalize_angle_stays_in_range_for_a_sweep() {
    let mut angle = -2000.0_f64;
    while angle < 2000.0 {
        let n = normalize_angle(angle);
        assert!((0.0..360.0).contains(&n), "normalize_angle({angle}) = {n}");
        angle += 0.37;
    }
    for &angle in &[
        -360.0,
        -720.0,
        360.0,
        720.0,
        -1e300,
        1e300,
        f64::MIN,
        f64::MAX,
    ] {
        let n = normalize_angle(angle);
        assert!((0.0..360.0).contains(&n), "normalize_angle({angle}) = {n}");
    }
}

#[test]
fn normalize_angle_keeps_ordinary_values() {
    assert_eq!(normalize_angle(0.0), 0.0);
    assert_eq!(normalize_angle(90.0), 90.0);
    assert_eq!(normalize_angle(360.0), 0.0);
    assert_eq!(normalize_angle(450.0), 90.0);
    assert_eq!(normalize_angle(-90.0), 270.0);
    assert_eq!(normalize_angle(-0.5), 359.5);
}

#[test]
fn normalize_angle_propagates_non_finite_values_as_nan() {
    assert!(normalize_angle(f64::NAN).is_nan());
    assert!(normalize_angle(f64::INFINITY).is_nan());
    assert!(normalize_angle(f64::NEG_INFINITY).is_nan());
}
