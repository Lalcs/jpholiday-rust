//! 差分検証用: 指定年範囲の全祝日を `YYYY-MM-DD\t名称` 形式で出力する。
//!
//! 使用例: `cargo run --example dump -- 1948 2099`
//! 本家 Python 版の同等出力と diff することで網羅的な挙動一致を確認する。

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let start: i32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1948);
    let end: i32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2099);

    let mut out = String::new();
    for year in start..=end {
        for (date, name) in jpholiday::year_holidays(year) {
            out.push_str(&format!(
                "{:04}-{:02}-{:02}\t{}\n",
                date.year(),
                date.month(),
                date.day(),
                name
            ));
        }
    }
    print!("{out}");
}
