use anyhow::{bail, Result};
use clap::Parser;
use chrono::{Datelike, Local, NaiveDate};


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cal`
pub struct Config {
    /// Year (1-9999)
    #[arg(value_parser(clap::value_parser!(i32).range(1..=9999)))]
    year: i32,
    /// Month name or number (1-12)
    #[arg(short)]
    month: Option<String>,
    /// Show the whole current year
    #[arg(short('y'), long("year"), conflicts_with_all(["month", "year"]))]
    show_current_year: bool,
}

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
        month = None;
        year = today.year();
    } else if month.is_none() && year == 0 {
        month = Some(today.month());
        year = today.year();
    }

    println!("Year: {year}");
    if let Some(month) = month {
        println!("Month: {month}");
    }
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
                    Some((i))
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

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{parse_month};

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
}
