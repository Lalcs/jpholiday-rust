//! 本家 `test_class.py` の移植。`JPHoliday` インスタンス API を検証する。

use jpholiday::{Date, Holiday, JPHoliday};

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

#[test]
fn holidays_returns_models() {
    let jp = JPHoliday::new();

    assert_eq!(
        jp.holidays(d(2020, 1, 1)),
        vec![Holiday::new(d(2020, 1, 1), "元日")]
    );
    assert_eq!(jp.holidays(d(2020, 2, 3)), Vec::<Holiday>::new());
    assert!(!jp.is_holiday(d(2020, 2, 3)));
}

#[test]
fn instance_year_and_month_holidays() {
    let jp = JPHoliday::new();

    // 2017 年 5 月: 憲法記念日・みどりの日・こどもの日。
    let may = jp.month_holidays(2017, 5);
    assert_eq!(may.len(), 3);
    assert_eq!(may[0], Holiday::new(d(2017, 5, 3), "憲法記念日"));
    assert_eq!(may[1], Holiday::new(d(2017, 5, 4), "みどりの日"));
    assert_eq!(may[2], Holiday::new(d(2017, 5, 5), "こどもの日"));

    // 2017 年の祝日数。
    assert_eq!(jp.year_holidays(2017).len(), 17);

    // 範囲取得（両端含む）。
    let between = jp.between(d(2017, 1, 1), d(2017, 1, 9));
    assert_eq!(
        between.first().unwrap(),
        &Holiday::new(d(2017, 1, 1), "元日")
    );
    assert_eq!(
        between
            .iter()
            .find(|h| h.date == d(2017, 1, 2))
            .unwrap()
            .name,
        "元日 振替休日"
    );
}
