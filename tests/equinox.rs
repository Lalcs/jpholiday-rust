//! 春分・秋分テーブルと天文関数の検証。

use jpholiday::astronomy;

/// 国立天文台の値に基づく春分の日（2000-2030）。
const VERNAL_2000_2030: [(i32, u32); 31] = [
    (2000, 20),
    (2001, 20),
    (2002, 21),
    (2003, 21),
    (2004, 20),
    (2005, 20),
    (2006, 21),
    (2007, 21),
    (2008, 20),
    (2009, 20),
    (2010, 21),
    (2011, 21),
    (2012, 20),
    (2013, 20),
    (2014, 21),
    (2015, 21),
    (2016, 20),
    (2017, 20),
    (2018, 21),
    (2019, 21),
    (2020, 20),
    (2021, 20),
    (2022, 21),
    (2023, 21),
    (2024, 20),
    (2025, 20),
    (2026, 20),
    (2027, 21),
    (2028, 20),
    (2029, 20),
    (2030, 20),
];

/// 国立天文台の値に基づく秋分の日（2000-2030）。
const AUTUMN_2000_2030: [(i32, u32); 31] = [
    (2000, 23),
    (2001, 23),
    (2002, 23),
    (2003, 23),
    (2004, 23),
    (2005, 23),
    (2006, 23),
    (2007, 23),
    (2008, 23),
    (2009, 23),
    (2010, 23),
    (2011, 23),
    (2012, 22),
    (2013, 23),
    (2014, 23),
    (2015, 23),
    (2016, 22),
    (2017, 23),
    (2018, 23),
    (2019, 23),
    (2020, 22),
    (2021, 23),
    (2022, 23),
    (2023, 23),
    (2024, 22),
    (2025, 23),
    (2026, 23),
    (2027, 23),
    (2028, 22),
    (2029, 23),
    (2030, 23),
];

#[test]
fn vernal_equinox_complete_range() {
    for (year, expected) in VERNAL_2000_2030 {
        assert_eq!(
            astronomy::calculate_vernal_equinox(year),
            expected,
            "vernal equinox for {year}"
        );
    }
}

#[test]
fn autumn_equinox_complete_range() {
    for (year, expected) in AUTUMN_2000_2030 {
        assert_eq!(
            astronomy::calculate_autumn_equinox(year),
            expected,
            "autumn equinox for {year}"
        );
    }
}

#[test]
fn equinox_historical_dates() {
    let vernal = [
        (1980, 20),
        (1990, 21),
        (2010, 21),
        (2020, 20),
        (2024, 20),
        (2025, 20),
    ];
    for (year, expected) in vernal {
        assert_eq!(astronomy::calculate_vernal_equinox(year), expected);
    }
    let autumn = [
        (1980, 23),
        (1990, 23),
        (2010, 23),
        (2020, 22),
        (2024, 22),
        (2025, 23),
    ];
    for (year, expected) in autumn {
        assert_eq!(astronomy::calculate_autumn_equinox(year), expected);
    }
}

#[test]
fn equinox_before_1948_is_zero() {
    for year in [1900, 1920, 1947] {
        assert_eq!(astronomy::calculate_vernal_equinox(year), 0);
        assert_eq!(astronomy::calculate_autumn_equinox(year), 0);
    }
}

#[test]
fn equinox_extended_range_bounds() {
    for year in [1948, 1950, 1980, 2000, 2050, 2100, 2200, 2500, 3000] {
        let v = astronomy::calculate_vernal_equinox(year);
        assert!(
            (19..=21).contains(&v),
            "vernal {v} out of bounds for {year}"
        );
        let a = astronomy::calculate_autumn_equinox(year);
        assert!(
            (21..=24).contains(&a),
            "autumn {a} out of bounds for {year}"
        );
    }
}

#[test]
fn consecutive_years_consistency() {
    for year in 2000..2030 {
        let d1 = astronomy::calculate_vernal_equinox(year) as i64;
        let d2 = astronomy::calculate_vernal_equinox(year + 1) as i64;
        assert!(
            (d1 - d2).abs() <= 2,
            "vernal jump between {year} and {}",
            year + 1
        );
    }
}

#[test]
fn julian_day_known_values() {
    let cases = [
        (2000, 1, 1, 12.0, 2_451_545.0),
        (1999, 1, 1, 0.0, 2_451_179.5),
        (1987, 1, 27, 0.0, 2_446_822.5),
        (1988, 1, 27, 0.0, 2_447_187.5),
    ];
    for (y, m, d, h, expected) in cases {
        assert!((astronomy::julian_day(y, m, d, h) - expected).abs() < 1e-4);
    }
}

#[test]
fn julian_day_to_datetime_epoch() {
    let dt = astronomy::julian_day_to_datetime(2_451_545.0);
    assert_eq!((dt.year, dt.month, dt.day, dt.hour), (2000, 1, 1, 12));
}

#[test]
fn solar_function_values() {
    // J2000.0 における平均近点角は約 357.5°、離心率は約 0.0167。
    assert!((astronomy::solar_anomaly(0.0) - 357.52910).abs() < 1e-3);
    assert!((astronomy::earth_orbit_eccentricity(0.0) - 0.0167).abs() < 1e-4);
}

#[test]
fn normalize_angle_cases() {
    let cases = [
        (0.0, 0.0),
        (360.0, 0.0),
        (720.0, 0.0),
        (-90.0, 270.0),
        (450.0, 90.0),
        (359.9, 359.9),
    ];
    for (input, expected) in cases {
        assert!((astronomy::normalize_angle(input) - expected).abs() < 1e-5);
    }
}

#[test]
fn solar_ecliptic_longitude_in_range() {
    // 太陽黄経は常に [0, 360) に収まる。
    for (y, m, d) in [
        (2000, 1, 1),
        (2000, 3, 20),
        (2000, 6, 21),
        (2000, 9, 22),
        (2000, 12, 21),
    ] {
        let jd = astronomy::julian_day(y, m, d, 12.0);
        let lon = astronomy::solar_ecliptic_longitude(jd);
        assert!((0.0..360.0).contains(&lon), "lon {lon} out of range");
    }
}

#[test]
fn solar_ecliptic_longitude_approximate_values() {
    // 分点・至点で太陽黄経が概ね 0/90/180/270° になる。
    let cases = [
        (2000, 3, 20, 0.0),
        (2000, 6, 21, 90.0),
        (2000, 9, 22, 180.0),
        (2000, 12, 21, 270.0),
    ];
    for (y, m, d, expected) in cases {
        let jd = astronomy::julian_day(y, m, d, 12.0);
        let lon = astronomy::solar_ecliptic_longitude(jd);
        let mut diff = (lon - expected).abs();
        if diff > 180.0 {
            diff = 360.0 - diff;
        }
        assert!(diff < 5.0, "longitude {lon} not within 5° of {expected}");
    }
}

#[test]
fn extreme_years_do_not_panic() {
    // 以前は debug ビルドで 22856 年以降（debug_assert）、release ビルドでも約 15 億年以降の一部の年
    // （expect）で panic し、3 月・9 月の日付を `is_holiday` に渡すだけで判定処理が落ちていた。
    // 計算が破綻する年は 0（該当日なし）を返し、それ以外は実在する日（1〜31）を返す。
    let mut years: Vec<i32> = vec![
        22_856,
        50_000,
        100_000,
        1_510_998_641,
        i32::MAX - 1,
        i32::MAX,
    ];
    years.extend((3_000..=1_000_000).step_by(997));
    years.extend((1_000_000..=i32::MAX).step_by(99_999_989));
    for year in years {
        for day in [
            astronomy::calculate_vernal_equinox(year),
            astronomy::calculate_autumn_equinox(year),
        ] {
            assert!(day <= 31, "year {year}: day {day}");
        }
    }

    // i32::MAX 年では計算が破綻する（以前は expect で panic していた）。
    assert_eq!(astronomy::calculate_vernal_equinox(i32::MAX), 0);
    assert_eq!(astronomy::calculate_autumn_equinox(i32::MAX), 0);
}

#[test]
fn holiday_queries_on_extreme_years_do_not_panic() {
    for year in [22_856, 50_000, 100_000, 1_510_998_641, i32::MAX] {
        // 分点の計算は 3 月・9 月の日付で行われ、振替休日・国民の休日の判定で前後の日も参照される。
        let _ = jpholiday::month_holidays(year, 3);
        let _ = jpholiday::month_holidays(year, 9);
        let _ = jpholiday::is_holiday(jpholiday::Date::new(year, 3, 20).unwrap());
        let _ = jpholiday::JPHoliday::new().month_holidays(year, 9);
    }
    // 年全体の問い合わせも panic せずに終わる（最大の年は末尾で巻き戻らずに止まる）。
    let _ = jpholiday::year_holidays(i32::MAX);
    let _ = jpholiday::JPHoliday::new().year_holidays(i32::MAX);
}
