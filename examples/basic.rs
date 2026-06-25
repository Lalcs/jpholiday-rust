//! jpholiday の基本的な使い方を示すサンプル。
//!
//! 実行: `cargo run --example basic`

use jpholiday::{Date, JPHoliday, OriginalHolidayChecker};

fn main() {
    // --- 関数 API ---
    let new_year = Date::new(2017, 1, 1).unwrap();
    println!("{new_year} は祝日? {}", jpholiday::is_holiday(new_year));
    println!(
        "{} の祝日名: {:?}",
        Date::new(2017, 1, 2).unwrap(),
        jpholiday::is_holiday_name(Date::new(2017, 1, 2).unwrap())
    );

    println!("\n2017 年の祝日一覧:");
    for (date, name) in jpholiday::year_holidays(2017) {
        println!("  {date}  {name}");
    }

    println!("\n2017 年 5 月の祝日:");
    for (date, name) in jpholiday::month_holidays(2017, 5) {
        println!("  {date}  {name}");
    }

    println!("\nゴールデンウィーク (2017-05-01 〜 2017-05-05):");
    for (date, name) in jpholiday::between(
        Date::new(2017, 5, 1).unwrap(),
        Date::new(2017, 5, 5).unwrap(),
    ) {
        println!("  {date}  {name}");
    }

    // --- クラス API ---
    let jp = JPHoliday::new();
    let holidays = jp.holidays(Date::new(2020, 1, 1).unwrap());
    println!("\nクラス API: {:?}", holidays);

    // --- 独自祝日 ---
    struct CompanyHoliday;
    impl OriginalHolidayChecker for CompanyHoliday {
        fn is_holiday(&self, date: Date) -> bool {
            date == Date::new(2020, 2, 9).unwrap()
        }
        fn holiday_name(&self, _date: Date) -> String {
            "特別休暇".to_string()
        }
    }

    let mut jp = JPHoliday::new();
    jp.register(CompanyHoliday);
    let special = Date::new(2020, 2, 9).unwrap();
    println!(
        "\n独自祝日を登録: {} -> {:?}",
        special,
        jp.is_holiday_name(special)
    );
    jp.unregister::<CompanyHoliday>();
    println!(
        "登録解除後: {} -> {:?}",
        special,
        jp.is_holiday_name(special)
    );
}
