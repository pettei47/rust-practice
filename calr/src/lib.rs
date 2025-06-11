use anyhow::{bail, Result};
use clap::Parser;
use chrono::{Datelike, Local, NaiveDate};
use itertools::izip;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cal`
pub struct Config {
    /// Year (1-9999)
    #[arg(value_parser(clap::value_parser!(i32).range(1..=9999)))]
    year: Option<i32>,
    /// Month name or number (1-12)
    #[arg(short)]
    month: Option<String>,
    /// Show the whole current year
    #[arg(short('y'), long("year"), conflicts_with_all(["month", "year"]))]
    show_current_year: bool,
}

const LINE_WIDTH: usize = 22;
const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub fn run(config: Config) -> Result<()>{
    let today = Local::now().date_naive();
    let mut year = config.year;
    let mut month = config.month.map(parse_month).transpose()?;

    if config.show_current_year {
        let year = today.year();
        let months: Vec<_> = (1..=12)
            .map(|month| format_month(year, month, false, today))
            .collect();
        println!("{year:>32}");
        for (i, chunk) in months.chunks(3).enumerate() {
                if let [m1, m2, m3] = chunk {
                    for lines in izip!(m1, m2, m3) {
                        println!("{}{}{}", lines.0, lines.1, lines.2);
                    }
                    if i < 3 {
                        println!();
                    }
                }
            }
        return Ok(());
    }

    if month.is_none() && year.is_some() {
        let year = year.unwrap();
        let months: Vec<_> = (1..=12)
            .map(|month| format_month(year, month, false, today))
            .collect();
        println!("{year:>32}");
        for (i, chunk) in months.chunks(3).enumerate() {
                if let [m1, m2, m3] = chunk {
                    for lines in izip!(m1, m2, m3) {
                        println!("{}{}{}", lines.0, lines.1, lines.2);
                    }
                    if i < 3 {
                        println!();
                    }
                }
            }
        return Ok(());
    }
    let print_year = year.is_some();
    if month.is_none() && year.is_none() {
        month = Some(today.month());
    }
    if year.is_none() {
        year = Some(today.year());
    }

    let formatted_month = format_month(year.unwrap(), month.unwrap(), print_year, today);
    println!("{}", formatted_month.join("\n"));
    Ok(())
}

fn parse_month(month: String) -> Result<u32> {
    match month.parse() {
        Ok(num) => {
            if num >= 1 && num <= 12 {
                Ok(num)
            } else {
                bail!(r#"month "{num}" not in the range 1 through 12"#)
            }
        }
        _ => {
            let month_lowered = &month.to_lowercase();
            let matches = MONTH_NAMES
                .iter()
                .enumerate()
                .filter_map(|(i, &m)| if m.to_lowercase().starts_with(month_lowered) {
                    Some(i)
                } else {
                    None
                })
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                bail!(r#"Invalid month "{month}""#)
            }
            Ok((matches[0] + 1) as u32)
        }
    }
}

fn format_month (year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last_day = last_day_in_month(year, month);

    // 先頭の空白を埋める
    let mut days: Vec<String> = (1..first_day.weekday().number_from_sunday())
        .map(|_| "  ".to_string())
        .collect();

    // 日付を埋める
    days.extend((first_day.day()..=last_day.day())
        .map(|d| {
            let date = NaiveDate::from_ymd_opt(year, month, d).unwrap();
            if date == today {
                format!("\u{1b}[7m{:2}\u{1b}[0m", d) // ハイライト
            } else {
                format!("{:2}", d)
            }
        }));
    
    let month_name = MONTH_NAMES[(month - 1) as usize];
    let header = format!(
        "{:^width$}  ",
        if print_year {
            format!("{} {}", month_name, year)
        } else {
            month_name.to_string()
        },
        width = LINE_WIDTH - 2,
    );

    let mut calendar = vec![header, "Su Mo Tu We Th Fr Sa  ".to_string()];

    // 7日ごとに行を分ける
    for day in days.chunks(7) {
        let week = format!(
            "{:width$}  ",
            day.join(" "),
            width = LINE_WIDTH - 2,
        );
        calendar.push(week);
    }

    while calendar.len() < 8 {
        calendar.push(" ".repeat(LINE_WIDTH));
    }

    calendar
}

fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    let leap_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        true
    } else {
        false
    };
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => NaiveDate::from_ymd_opt(year, month, 31).unwrap(),
        4 | 6 | 9 | 11 => NaiveDate::from_ymd_opt(year, month, 30).unwrap(),
        2 if leap_year => NaiveDate::from_ymd_opt(year, month, 29).unwrap(),
        _ => NaiveDate::from_ymd_opt(year, month, 28).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::{format_month, last_day_in_month, parse_month};
    use chrono::NaiveDate;

    #[test]
    fn test_parse_month() {
        let res = parse_month("1".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"month "0" not in the range 1 through 12"#
        );

        let res = parse_month("13".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"month "13" not in the range 1 through 12"#
        );

        let res = parse_month("foo".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"Invalid month "foo""#);

        let res = parse_month("ju".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"Invalid month "ju""#);
    }

    #[test]
    fn test_format_month() {
        let today = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
        let leap_february = vec![
            "   February 2020      ",
            "Su Mo Tu We Th Fr Sa  ",
            "                   1  ",
            " 2  3  4  5  6  7  8  ",
            " 9 10 11 12 13 14 15  ",
            "16 17 18 19 20 21 22  ",
            "23 24 25 26 27 28 29  ",
            "                      ",
        ];
        assert_eq!(format_month(2020, 2, true, today), leap_february);

        let may = vec![
            "        May           ",
            "Su Mo Tu We Th Fr Sa  ",
            "                1  2  ",
            " 3  4  5  6  7  8  9  ",
            "10 11 12 13 14 15 16  ",
            "17 18 19 20 21 22 23  ",
            "24 25 26 27 28 29 30  ",
            "31                    ",
        ];
        assert_eq!(format_month(2020, 5, false, today), may);

        let april_hl = vec![
            "     April 2021       ",
            "Su Mo Tu We Th Fr Sa  ",
            "             1  2  3  ",
            " 4  5  6 \u{1b}[7m 7\u{1b}[0m  8  9 10  ",
            "11 12 13 14 15 16 17  ",
            "18 19 20 21 22 23 24  ",
            "25 26 27 28 29 30     ",
            "                      ",
        ];
        let today = NaiveDate::from_ymd_opt(2021, 4, 7).unwrap();
        assert_eq!(format_month(2021, 4, true, today), april_hl);
    }

    #[test]
    fn test_last_day_in_month() {
        assert_eq!(
            last_day_in_month(2020, 1),
            NaiveDate::from_ymd_opt(2020, 1, 31).unwrap()
        );
        assert_eq!(
            last_day_in_month(2020, 2),
            NaiveDate::from_ymd_opt(2020, 2, 29).unwrap()
        );
        assert_eq!(
            last_day_in_month(2020, 4),
            NaiveDate::from_ymd_opt(2020, 4, 30).unwrap()
        );
    }

}
