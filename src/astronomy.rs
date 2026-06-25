//! 春分・秋分の天文計算。
//!
//! Jean Meeus 『Astronomical Algorithms』(2nd ed.) に基づき、太陽の黄経を求めて
//! Newton 法で分点の瞬刻を計算します。標準ライブラリのみで完結し、本家 Python 版
//! (`jpholiday/checker/astronomy.py`) を演算順序まで忠実に移植したものです。
//!
//! - 精度: 1948〜3000 年で ±1 日
//! - 対応範囲: 1948 年以降（1948 年より前は 0 を返す）

use crate::date::Date;

/// J2000.0 元期（2000-01-01 12:00:00 UTC）のユリウス日。
const J2000: f64 = 2_451_545.0;
/// ユリウス世紀あたりの日数。
const DAYS_PER_JULIAN_CENTURY: f64 = 36_525.0;
/// 度からラジアンへの換算係数。
const DEGREES_TO_RADIANS: f64 = std::f64::consts::PI / 180.0;

/// ユリウス日 → 日時の分解結果。
///
/// `julian_day_to_datetime` の戻り値であり、本家の `datetime.datetime` に対応します。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JulianDateTime {
    /// 年。
    pub year: i32,
    /// 月（1〜12）。
    pub month: u32,
    /// 日。
    pub day: u32,
    /// 時（0〜23）。
    pub hour: i64,
    /// 分（0〜59）。
    pub minute: i64,
    /// 秒（0〜59）。
    pub second: i64,
    /// マイクロ秒。
    pub microsecond: i64,
}

/// 指定した日時のユリウス日を計算します（Meeus 第 7 章）。
pub fn julian_day(year: i64, month: i64, day: i64, hour: f64) -> f64 {
    let (mut y, mut m) = (year, month);
    if m <= 2 {
        y -= 1;
        m += 12;
    }

    // グレゴリオ暦補正。
    let a = y.div_euclid(100);
    let b = 2 - a + a.div_euclid(4);

    (365.25 * (y + 4716) as f64).floor()
        + (30.6001 * (m + 1) as f64).floor()
        + day as f64
        + hour / 24.0
        + b as f64
        - 1524.5
}

/// J2000.0 元期からのユリウス世紀を計算します。
#[inline]
pub fn julian_centuries_since_j2000(jd: f64) -> f64 {
    (jd - J2000) / DAYS_PER_JULIAN_CENTURY
}

/// 角度を `[0, 360)` 度の範囲へ正規化します。
pub fn normalize_angle(angle: f64) -> f64 {
    let a = angle % 360.0;
    if a < 0.0 { a + 360.0 } else { a }
}

/// 太陽の平均黄経（度）を計算します。
pub fn solar_mean_longitude(t: f64) -> f64 {
    let l0 = 280.4664567 + 36000.76982779 * t + 0.0003032028 * t * t + t * t * t / 49_931_000.0;
    normalize_angle(l0)
}

/// 地球の軌道離心率を計算します。
pub fn earth_orbit_eccentricity(t: f64) -> f64 {
    0.016708634 - 0.000042037 * t - 0.0000001267 * t * t
}

/// 太陽の平均近点角（度）を計算します。
pub fn solar_anomaly(t: f64) -> f64 {
    let m = 357.52910 + 35999.05030 * t - 0.0001559 * t * t - 0.00000048 * t * t * t;
    normalize_angle(m)
}

/// 中心差（真近点角と平均近点角の差、度）を計算します。
pub fn equation_of_center(t: f64) -> f64 {
    let m = solar_anomaly(t);
    let m_rad = m * DEGREES_TO_RADIANS;

    (1.914600 - 0.004817 * t - 0.000014 * t * t) * m_rad.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * m_rad).sin()
        + 0.000290 * (3.0 * m_rad).sin()
}

/// 太陽の視黄経（度、`[0, 360)`）を計算します。
///
/// - 0°: 春分（3 月）
/// - 90°: 夏至（6 月）
/// - 180°: 秋分（9 月）
/// - 270°: 冬至（12 月）
pub fn solar_ecliptic_longitude(jd: f64) -> f64 {
    let t = julian_centuries_since_j2000(jd);

    let l0 = solar_mean_longitude(t);
    let c = equation_of_center(t);
    let true_longitude = l0 + c;

    // 章動（簡略式）と光行差の補正。
    let omega = 125.04 - 1934.136 * t;
    let omega_rad = omega * DEGREES_TO_RADIANS;
    let nutation = -0.00478 * omega_rad.sin();
    let aberration = -0.00569;

    let apparent_longitude = true_longitude + nutation + aberration;
    normalize_angle(apparent_longitude)
}

/// 太陽黄経の時間変化率（度/日）を数値微分で計算します（Newton 法用）。
pub fn solar_ecliptic_longitude_rate(jd: f64, dt: f64) -> f64 {
    let lon1 = solar_ecliptic_longitude(jd - dt / 2.0);
    let lon2 = solar_ecliptic_longitude(jd + dt / 2.0);

    let mut diff = lon2 - lon1;
    if diff > 180.0 {
        diff -= 360.0;
    } else if diff < -180.0 {
        diff += 360.0;
    }

    diff / dt
}

/// 太陽が指定黄経に達する瞬刻のユリウス日を Newton 法で求めます。
fn find_equinox_solstice_jd(
    year: i64,
    target_longitude: f64,
    initial_month: i64,
    initial_day: i64,
) -> f64 {
    let mut jd = julian_day(year, initial_month, initial_day, 12.0);

    let max_iterations = 10;
    let tolerance = 0.00001; // 約 0.86 秒

    for _ in 0..max_iterations {
        let current_lon = solar_ecliptic_longitude(jd);

        let mut diff = current_lon - target_longitude;
        if diff > 180.0 {
            diff -= 360.0;
        } else if diff < -180.0 {
            diff += 360.0;
        }

        if diff.abs() < tolerance {
            break;
        }

        let rate = solar_ecliptic_longitude_rate(jd, 0.0001);
        jd -= diff / rate;
    }

    jd
}

/// ユリウス日を日時へ変換します（Meeus 第 7 章）。返す日時は UTC。
pub fn julian_day_to_datetime(jd: f64) -> JulianDateTime {
    let jd = jd + 0.5;
    let z = jd.floor();
    let f = jd - z;

    let a = if z < 2_299_161.0 {
        z
    } else {
        let alpha = ((z - 1_867_216.25) / 36_524.25).floor();
        z + 1.0 + alpha - (alpha / 4.0).floor()
    };

    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let d = (365.25 * c).floor();
    let e = ((b - d) / 30.6001).floor();

    let day = b - d - (30.6001 * e).floor() + f;

    let month = if e < 14.0 { e - 1.0 } else { e - 13.0 };
    let year = if month > 2.0 { c - 4716.0 } else { c - 4715.0 };

    // 日・時・分・秒・マイクロ秒へ分解（Python の int() = 0 方向への切り捨て）。
    let day_int = day.trunc() as i64;
    let hour_frac = (day - day_int as f64) * 24.0;
    let hour = hour_frac.trunc() as i64;
    let minute_frac = (hour_frac - hour as f64) * 60.0;
    let minute = minute_frac.trunc() as i64;
    let second_frac = (minute_frac - minute as f64) * 60.0;
    let second = second_frac.trunc() as i64;
    let microsecond = ((second_frac - second as f64) * 1_000_000.0).trunc() as i64;

    JulianDateTime {
        year: year as i32,
        month: month as u32,
        day: day_int as u32,
        hour,
        minute,
        second,
        microsecond,
    }
}

/// UTC の分点日時を日本標準時 (UTC+9) に直したときの「日」を返します。
///
/// 本家は `datetime` に対し `timedelta(hours=9)` を加算してから日付を読み出すため、
/// 9 時間加算で時が 24 を超えた場合のみ翌日へ繰り上げます（分・秒は繰り上げに影響しない）。
fn jst_day(utc: JulianDateTime, expected_month: u32) -> u32 {
    let base = Date::new(utc.year, utc.month, utc.day)
        .expect("equinox calculation produced an invalid calendar date");
    let jst = if utc.hour + 9 >= 24 {
        base.succ()
    } else {
        base
    };
    debug_assert_eq!(
        jst.month(),
        expected_month,
        "equinox resolved to an unexpected month"
    );
    jst.day()
}

/// 春分の日（3 月）の日を返します。1948 年より前は 0。
pub fn calculate_vernal_equinox(year: i32) -> u32 {
    if year < 1948 {
        return 0;
    }
    let jd = find_equinox_solstice_jd(year as i64, 0.0, 3, 20);
    jst_day(julian_day_to_datetime(jd), 3)
}

/// 秋分の日（9 月）の日を返します。1948 年より前は 0。
pub fn calculate_autumn_equinox(year: i32) -> u32 {
    if year < 1948 {
        return 0;
    }
    let jd = find_equinox_solstice_jd(year as i64, 180.0, 9, 23);
    jst_day(julian_day_to_datetime(jd), 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn julian_day_epoch() {
        assert!((julian_day(2000, 1, 1, 12.0) - 2_451_545.0).abs() < 1e-5);
        assert!((julian_day(1999, 1, 1, 0.0) - 2_451_179.5).abs() < 1e-4);
        assert!((julian_day(1987, 1, 27, 0.0) - 2_446_822.5).abs() < 1e-4);
        assert!((julian_day(1988, 1, 27, 0.0) - 2_447_187.5).abs() < 1e-4);
    }

    #[test]
    fn jd_to_datetime_epoch() {
        let dt = julian_day_to_datetime(2_451_545.0);
        assert_eq!((dt.year, dt.month, dt.day, dt.hour), (2000, 1, 1, 12));
    }

    #[test]
    fn normalize() {
        assert!((normalize_angle(360.0) - 0.0).abs() < 1e-9);
        assert!((normalize_angle(-90.0) - 270.0).abs() < 1e-9);
        assert!((normalize_angle(450.0) - 90.0).abs() < 1e-9);
    }

    #[test]
    fn equinox_known_values() {
        assert_eq!(calculate_vernal_equinox(2000), 20);
        assert_eq!(calculate_vernal_equinox(2024), 20);
        assert_eq!(calculate_autumn_equinox(2012), 22);
        assert_eq!(calculate_autumn_equinox(2020), 22);
        assert_eq!(calculate_vernal_equinox(1947), 0);
        assert_eq!(calculate_autumn_equinox(1947), 0);
    }
}
