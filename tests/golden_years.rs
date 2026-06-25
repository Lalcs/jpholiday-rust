//! 年別ゴールデンテスト（1971-2027）。
//!
//! 各年の全祝日を 3 種類の規則的な検査（祝日名・月別件数・年別件数）で網羅的に固定した
//! ゴールデンデータであり、手で編集しないこと。

use jpholiday::Date;

#[inline]
fn d(y: i32, m: u32, day: u32) -> Date {
    Date::new(y, m, day).unwrap()
}

#[test]
fn golden_year_1971() {
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 1, 15)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 4, 29)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 9, 24)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1971, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(1971, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(1971, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(1971, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(1971, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(1971, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(1971, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(1971, 7).len(), 0);
    assert_eq!(jpholiday::month_holidays(1971, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(1971, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(1971, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(1971, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(1971, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(1971).len(), 13);
}

#[test]
fn golden_year_1988() {
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 1, 15)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 3, 21)).as_deref(),
        Some("春分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 4, 29)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1988, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(1988, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(1988, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(1988, 3).len(), 2);
    assert_eq!(jpholiday::month_holidays(1988, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(1988, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(1988, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(1988, 7).len(), 0);
    assert_eq!(jpholiday::month_holidays(1988, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(1988, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(1988, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(1988, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(1988, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(1988).len(), 14);
}

#[test]
fn golden_year_1989() {
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 1, 2)).as_deref(),
        Some("元日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 1, 15)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 1, 16)).as_deref(),
        Some("成人の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 2, 24)).as_deref(),
        Some("昭和天皇の大喪の礼")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1989, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(1989, 1).len(), 4);
    assert_eq!(jpholiday::month_holidays(1989, 2).len(), 2);
    assert_eq!(jpholiday::month_holidays(1989, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(1989, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(1989, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(1989, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(1989, 7).len(), 0);
    assert_eq!(jpholiday::month_holidays(1989, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(1989, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(1989, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(1989, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(1989, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(1989).len(), 17);
}

#[test]
fn golden_year_1992() {
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 1, 15)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 5, 4)).as_deref(),
        Some("憲法記念日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(jpholiday::is_holiday_name(d(1992, 5, 6)).as_deref(), None);
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1992, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(1992, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(1992, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(1992, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(1992, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(1992, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(1992, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(1992, 7).len(), 0);
    assert_eq!(jpholiday::month_holidays(1992, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(1992, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(1992, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(1992, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(1992, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(1992).len(), 14);
}

#[test]
fn golden_year_1997() {
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 1, 15)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(jpholiday::is_holiday_name(d(1997, 5, 4)).as_deref(), None);
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 7, 20)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 7, 21)).as_deref(),
        Some("海の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 11, 24)).as_deref(),
        Some("勤労感謝の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1997, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(1997, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(1997, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(1997, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(1997, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(1997, 5).len(), 2);
    assert_eq!(jpholiday::month_holidays(1997, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(1997, 7).len(), 2);
    assert_eq!(jpholiday::month_holidays(1997, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(1997, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(1997, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(1997, 11).len(), 3);
    assert_eq!(jpholiday::month_holidays(1997, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(1997).len(), 16);
}

#[test]
fn golden_year_1998() {
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 1, 15)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 5, 4)).as_deref(),
        Some("憲法記念日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(jpholiday::is_holiday_name(d(1998, 5, 6)).as_deref(), None);
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 7, 20)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1998, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(1998, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(1998, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(1998, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(1998, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(1998, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(1998, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(1998, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(1998, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(1998, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(1998, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(1998, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(1998, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(1998).len(), 15);
}

#[test]
fn golden_year_1999() {
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 1, 15)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 3, 22)).as_deref(),
        Some("春分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 7, 20)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 10, 11)).as_deref(),
        Some("体育の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(1999, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(1999, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(1999, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(1999, 3).len(), 2);
    assert_eq!(jpholiday::month_holidays(1999, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(1999, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(1999, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(1999, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(1999, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(1999, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(1999, 10).len(), 2);
    assert_eq!(jpholiday::month_holidays(1999, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(1999, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(1999).len(), 17);
}

#[test]
fn golden_year_2000() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 1, 10)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 7, 20)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 10, 9)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2000, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2000, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2000, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2000, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2000, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2000, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2000, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2000, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2000, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2000, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2000, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2000, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2000, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2000).len(), 15);
}

#[test]
fn golden_year_2001() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 1, 8)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 2, 12)).as_deref(),
        Some("建国記念の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 4, 30)).as_deref(),
        Some("みどりの日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 7, 20)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 9, 24)).as_deref(),
        Some("秋分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 10, 8)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2001, 12, 24)).as_deref(),
        Some("天皇誕生日 振替休日")
    );
    assert_eq!(jpholiday::month_holidays(2001, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2001, 2).len(), 2);
    assert_eq!(jpholiday::month_holidays(2001, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2001, 4).len(), 2);
    assert_eq!(jpholiday::month_holidays(2001, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2001, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2001, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2001, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2001, 9).len(), 3);
    assert_eq!(jpholiday::month_holidays(2001, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2001, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2001, 12).len(), 2);
    assert_eq!(jpholiday::year_holidays(2001).len(), 19);
}

#[test]
fn golden_year_2002() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 1, 14)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 5, 6)).as_deref(),
        Some("こどもの日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 7, 20)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 9, 16)).as_deref(),
        Some("敬老の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 10, 14)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 11, 4)).as_deref(),
        Some("文化の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2002, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2002, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2002, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2002, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2002, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2002, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2002, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2002, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2002, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2002, 9).len(), 3);
    assert_eq!(jpholiday::month_holidays(2002, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2002, 11).len(), 3);
    assert_eq!(jpholiday::month_holidays(2002, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2002).len(), 18);
}

#[test]
fn golden_year_2003() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 1, 13)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 7, 21)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 10, 13)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 11, 24)).as_deref(),
        Some("勤労感謝の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2003, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2003, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2003, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2003, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2003, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2003, 5).len(), 2);
    assert_eq!(jpholiday::month_holidays(2003, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2003, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2003, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2003, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2003, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2003, 11).len(), 3);
    assert_eq!(jpholiday::month_holidays(2003, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2003).len(), 15);
}

#[test]
fn golden_year_2004() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 1, 12)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 7, 19)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 9, 20)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 10, 11)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2004, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2004, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2004, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2004, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2004, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2004, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2004, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2004, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2004, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2004, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2004, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2004, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2004, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2004).len(), 15);
}

#[test]
fn golden_year_2005() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 1, 10)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 3, 21)).as_deref(),
        Some("春分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 7, 18)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 9, 19)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2005, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2005, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2005, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2005, 3).len(), 2);
    assert_eq!(jpholiday::month_holidays(2005, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2005, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2005, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2005, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2005, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2005, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2005, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2005, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2005, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2005).len(), 16);
}

#[test]
fn golden_year_2006() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 1, 2)).as_deref(),
        Some("元日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 1, 9)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 4, 29)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 5, 4)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 7, 17)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 9, 18)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 10, 9)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2006, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2006, 1).len(), 3);
    assert_eq!(jpholiday::month_holidays(2006, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2006, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2006, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2006, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2006, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2006, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2006, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2006, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2006, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2006, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2006, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2006).len(), 16);
}

#[test]
fn golden_year_2007() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 1, 8)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 2, 12)).as_deref(),
        Some("建国記念の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 4, 30)).as_deref(),
        Some("昭和の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 7, 16)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 9, 17)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 9, 24)).as_deref(),
        Some("秋分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 10, 8)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2007, 12, 24)).as_deref(),
        Some("天皇誕生日 振替休日")
    );
    assert_eq!(jpholiday::month_holidays(2007, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2007, 2).len(), 2);
    assert_eq!(jpholiday::month_holidays(2007, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2007, 4).len(), 2);
    assert_eq!(jpholiday::month_holidays(2007, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2007, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2007, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2007, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2007, 9).len(), 3);
    assert_eq!(jpholiday::month_holidays(2007, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2007, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2007, 12).len(), 2);
    assert_eq!(jpholiday::year_holidays(2007).len(), 19);
}

#[test]
fn golden_year_2008() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 1, 14)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 5, 6)).as_deref(),
        Some("みどりの日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 7, 21)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 10, 13)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 11, 24)).as_deref(),
        Some("勤労感謝の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2008, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2008, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2008, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2008, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2008, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2008, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2008, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2008, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2008, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2008, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2008, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2008, 11).len(), 3);
    assert_eq!(jpholiday::month_holidays(2008, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2008).len(), 17);
}

#[test]
fn golden_year_2009() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 1, 12)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 5, 6)).as_deref(),
        Some("憲法記念日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 7, 20)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 9, 21)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 9, 22)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 10, 12)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2009, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2009, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2009, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2009, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2009, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2009, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2009, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2009, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2009, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2009, 9).len(), 3);
    assert_eq!(jpholiday::month_holidays(2009, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2009, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2009, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2009).len(), 17);
}

#[test]
fn golden_year_2010() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 1, 11)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 3, 22)).as_deref(),
        Some("春分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 7, 19)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 9, 20)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 10, 11)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2010, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2010, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2010, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2010, 3).len(), 2);
    assert_eq!(jpholiday::month_holidays(2010, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2010, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2010, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2010, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2010, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2010, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2010, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2010, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2010, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2010).len(), 16);
}

#[test]
fn golden_year_2011() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 1, 10)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 7, 18)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 9, 19)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2011, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2011, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2011, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2011, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2011, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2011, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2011, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2011, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2011, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2011, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2011, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2011, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2011, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2011).len(), 15);
}

#[test]
fn golden_year_2012() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 1, 2)).as_deref(),
        Some("元日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 1, 9)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 4, 30)).as_deref(),
        Some("昭和の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 7, 16)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 9, 17)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 9, 22)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 10, 8)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2012, 12, 24)).as_deref(),
        Some("天皇誕生日 振替休日")
    );
    assert_eq!(jpholiday::month_holidays(2012, 1).len(), 3);
    assert_eq!(jpholiday::month_holidays(2012, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2012, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2012, 4).len(), 2);
    assert_eq!(jpholiday::month_holidays(2012, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2012, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2012, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2012, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2012, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2012, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2012, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2012, 12).len(), 2);
    assert_eq!(jpholiday::year_holidays(2012).len(), 18);
}

#[test]
fn golden_year_2013() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 1, 14)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 5, 6)).as_deref(),
        Some("こどもの日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 7, 15)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 9, 16)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 10, 14)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 11, 4)).as_deref(),
        Some("文化の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2013, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2013, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2013, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2013, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2013, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2013, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2013, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2013, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2013, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2013, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2013, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2013, 11).len(), 3);
    assert_eq!(jpholiday::month_holidays(2013, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2013).len(), 17);
}

#[test]
fn golden_year_2014() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 1, 13)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 5, 6)).as_deref(),
        Some("みどりの日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 7, 21)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 10, 13)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 11, 24)).as_deref(),
        Some("勤労感謝の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2014, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2014, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2014, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2014, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2014, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2014, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2014, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2014, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2014, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2014, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2014, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2014, 11).len(), 3);
    assert_eq!(jpholiday::month_holidays(2014, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2014).len(), 17);
}

#[test]
fn golden_year_2015() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 1, 12)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 5, 6)).as_deref(),
        Some("憲法記念日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 7, 20)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 9, 21)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 9, 22)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 10, 12)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2015, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2015, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2015, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2015, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2015, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2015, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2015, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2015, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2015, 8).len(), 0);
    assert_eq!(jpholiday::month_holidays(2015, 9).len(), 3);
    assert_eq!(jpholiday::month_holidays(2015, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2015, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2015, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2015).len(), 17);
}

#[test]
fn golden_year_2016() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 1, 11)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 3, 21)).as_deref(),
        Some("春分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 7, 18)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 9, 19)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 9, 22)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 10, 10)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2016, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2016, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2016, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2016, 3).len(), 2);
    assert_eq!(jpholiday::month_holidays(2016, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2016, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2016, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2016, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2016, 8).len(), 1);
    assert_eq!(jpholiday::month_holidays(2016, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2016, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2016, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2016, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2016).len(), 17);
}

#[test]
fn golden_year_2017() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 1, 2)).as_deref(),
        Some("元日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 1, 9)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 7, 17)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 9, 18)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 10, 9)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2017, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(jpholiday::month_holidays(2017, 1).len(), 3);
    assert_eq!(jpholiday::month_holidays(2017, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2017, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2017, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2017, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2017, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2017, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2017, 8).len(), 1);
    assert_eq!(jpholiday::month_holidays(2017, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2017, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2017, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2017, 12).len(), 1);
    assert_eq!(jpholiday::year_holidays(2017).len(), 17);
}

#[test]
fn golden_year_2018() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 1, 8)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 2, 12)).as_deref(),
        Some("建国記念の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 4, 30)).as_deref(),
        Some("昭和の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 7, 16)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 9, 17)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 9, 24)).as_deref(),
        Some("秋分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 10, 8)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 12, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2018, 12, 24)).as_deref(),
        Some("天皇誕生日 振替休日")
    );
    assert_eq!(jpholiday::month_holidays(2018, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2018, 2).len(), 2);
    assert_eq!(jpholiday::month_holidays(2018, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2018, 4).len(), 2);
    assert_eq!(jpholiday::month_holidays(2018, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2018, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2018, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2018, 8).len(), 1);
    assert_eq!(jpholiday::month_holidays(2018, 9).len(), 3);
    assert_eq!(jpholiday::month_holidays(2018, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2018, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2018, 12).len(), 2);
    assert_eq!(jpholiday::year_holidays(2018).len(), 20);
}

#[test]
fn golden_year_2019() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 1, 14)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 4, 30)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 5, 1)).as_deref(),
        Some("天皇の即位の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 5, 2)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 5, 6)).as_deref(),
        Some("こどもの日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 7, 15)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 8, 12)).as_deref(),
        Some("山の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 9, 16)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 10, 14)).as_deref(),
        Some("体育の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 10, 22)).as_deref(),
        Some("即位礼正殿の儀")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 11, 4)).as_deref(),
        Some("文化の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2019, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(2019, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2019, 2).len(), 1);
    assert_eq!(jpholiday::month_holidays(2019, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2019, 4).len(), 2);
    assert_eq!(jpholiday::month_holidays(2019, 5).len(), 6);
    assert_eq!(jpholiday::month_holidays(2019, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2019, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2019, 8).len(), 2);
    assert_eq!(jpholiday::month_holidays(2019, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2019, 10).len(), 2);
    assert_eq!(jpholiday::month_holidays(2019, 11).len(), 3);
    assert_eq!(jpholiday::month_holidays(2019, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(2019).len(), 22);
}

#[test]
fn golden_year_2020() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 1, 13)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 2, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 2, 24)).as_deref(),
        Some("天皇誕生日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 5, 6)).as_deref(),
        Some("憲法記念日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 7, 23)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 7, 24)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 8, 10)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 9, 21)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 9, 22)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2020, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(2020, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2020, 2).len(), 3);
    assert_eq!(jpholiday::month_holidays(2020, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2020, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2020, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2020, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2020, 7).len(), 2);
    assert_eq!(jpholiday::month_holidays(2020, 8).len(), 1);
    assert_eq!(jpholiday::month_holidays(2020, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2020, 10).len(), 0);
    assert_eq!(jpholiday::month_holidays(2020, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2020, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(2020).len(), 18);
}

#[test]
fn golden_year_2021() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 1, 11)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 2, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 7, 22)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 7, 23)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 8, 8)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 8, 9)).as_deref(),
        Some("山の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 9, 20)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2021, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(2021, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2021, 2).len(), 2);
    assert_eq!(jpholiday::month_holidays(2021, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2021, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2021, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2021, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2021, 7).len(), 2);
    assert_eq!(jpholiday::month_holidays(2021, 8).len(), 2);
    assert_eq!(jpholiday::month_holidays(2021, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2021, 10).len(), 0);
    assert_eq!(jpholiday::month_holidays(2021, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2021, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(2021).len(), 17);
}

#[test]
fn golden_year_2022() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 1, 10)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 2, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 7, 18)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 9, 19)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 10, 10)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2022, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(2022, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2022, 2).len(), 2);
    assert_eq!(jpholiday::month_holidays(2022, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2022, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2022, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2022, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2022, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2022, 8).len(), 1);
    assert_eq!(jpholiday::month_holidays(2022, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2022, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2022, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2022, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(2022).len(), 16);
}

#[test]
fn golden_year_2023() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 1, 2)).as_deref(),
        Some("元日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 1, 9)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 2, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 7, 17)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 9, 18)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 10, 9)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2023, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(2023, 1).len(), 3);
    assert_eq!(jpholiday::month_holidays(2023, 2).len(), 2);
    assert_eq!(jpholiday::month_holidays(2023, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2023, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2023, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2023, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2023, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2023, 8).len(), 1);
    assert_eq!(jpholiday::month_holidays(2023, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2023, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2023, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2023, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(2023).len(), 17);
}

#[test]
fn golden_year_2024() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 1, 8)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 2, 12)).as_deref(),
        Some("建国記念の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 2, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 5, 6)).as_deref(),
        Some("こどもの日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 7, 15)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 8, 12)).as_deref(),
        Some("山の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 9, 16)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 9, 22)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 9, 23)).as_deref(),
        Some("秋分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 10, 14)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 11, 4)).as_deref(),
        Some("文化の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2024, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(2024, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2024, 2).len(), 3);
    assert_eq!(jpholiday::month_holidays(2024, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2024, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2024, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2024, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2024, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2024, 8).len(), 2);
    assert_eq!(jpholiday::month_holidays(2024, 9).len(), 3);
    assert_eq!(jpholiday::month_holidays(2024, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2024, 11).len(), 3);
    assert_eq!(jpholiday::month_holidays(2024, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(2024).len(), 21);
}

#[test]
fn golden_year_2025() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 1, 13)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 2, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 2, 24)).as_deref(),
        Some("天皇誕生日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 5, 6)).as_deref(),
        Some("みどりの日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 7, 21)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 9, 15)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 10, 13)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2025, 11, 24)).as_deref(),
        Some("勤労感謝の日 振替休日")
    );
    assert_eq!(jpholiday::month_holidays(2025, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2025, 2).len(), 3);
    assert_eq!(jpholiday::month_holidays(2025, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2025, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2025, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2025, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2025, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2025, 8).len(), 1);
    assert_eq!(jpholiday::month_holidays(2025, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2025, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2025, 11).len(), 3);
    assert_eq!(jpholiday::month_holidays(2025, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(2025).len(), 19);
}

#[test]
fn golden_year_2026() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 1, 12)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 2, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 3, 20)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 5, 6)).as_deref(),
        Some("憲法記念日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 7, 20)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 9, 21)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 9, 22)).as_deref(),
        Some("国民の休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 10, 12)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2026, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(2026, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2026, 2).len(), 2);
    assert_eq!(jpholiday::month_holidays(2026, 3).len(), 1);
    assert_eq!(jpholiday::month_holidays(2026, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2026, 5).len(), 4);
    assert_eq!(jpholiday::month_holidays(2026, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2026, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2026, 8).len(), 1);
    assert_eq!(jpholiday::month_holidays(2026, 9).len(), 3);
    assert_eq!(jpholiday::month_holidays(2026, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2026, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2026, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(2026).len(), 18);
}

#[test]
fn golden_year_2027() {
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 1, 1)).as_deref(),
        Some("元日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 1, 11)).as_deref(),
        Some("成人の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 2, 11)).as_deref(),
        Some("建国記念の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 2, 23)).as_deref(),
        Some("天皇誕生日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 3, 21)).as_deref(),
        Some("春分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 3, 22)).as_deref(),
        Some("春分の日 振替休日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 4, 29)).as_deref(),
        Some("昭和の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 5, 3)).as_deref(),
        Some("憲法記念日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 5, 4)).as_deref(),
        Some("みどりの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 5, 5)).as_deref(),
        Some("こどもの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 7, 19)).as_deref(),
        Some("海の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 8, 11)).as_deref(),
        Some("山の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 9, 20)).as_deref(),
        Some("敬老の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 9, 23)).as_deref(),
        Some("秋分の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 10, 11)).as_deref(),
        Some("スポーツの日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 11, 3)).as_deref(),
        Some("文化の日")
    );
    assert_eq!(
        jpholiday::is_holiday_name(d(2027, 11, 23)).as_deref(),
        Some("勤労感謝の日")
    );
    assert_eq!(jpholiday::month_holidays(2027, 1).len(), 2);
    assert_eq!(jpholiday::month_holidays(2027, 2).len(), 2);
    assert_eq!(jpholiday::month_holidays(2027, 3).len(), 2);
    assert_eq!(jpholiday::month_holidays(2027, 4).len(), 1);
    assert_eq!(jpholiday::month_holidays(2027, 5).len(), 3);
    assert_eq!(jpholiday::month_holidays(2027, 6).len(), 0);
    assert_eq!(jpholiday::month_holidays(2027, 7).len(), 1);
    assert_eq!(jpholiday::month_holidays(2027, 8).len(), 1);
    assert_eq!(jpholiday::month_holidays(2027, 9).len(), 2);
    assert_eq!(jpholiday::month_holidays(2027, 10).len(), 1);
    assert_eq!(jpholiday::month_holidays(2027, 11).len(), 2);
    assert_eq!(jpholiday::month_holidays(2027, 12).len(), 0);
    assert_eq!(jpholiday::year_holidays(2027).len(), 17);
}
