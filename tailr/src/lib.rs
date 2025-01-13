use crate::TakeValue::*;
use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

impl FromStr for TakeValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "+0" {
            Ok(TakeValue::PlusZero)
        } else if let Ok(num) = s.parse::<i64>() {
            if s.starts_with('+') || num < 0 {
                Ok(TakeValue::TakeNum(num))
            } else {
                Ok(TakeValue::TakeNum(-num))
            }
        } else {
            Err(format!("illegal offset -- {}: Invalid argument", s))
        }
    }
}

/// tail command with Rust
#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct Config {
    /// Input file(s)
    #[arg(value_name = "FILE", required = true)]
    files: Vec<String>,
    /// Number of lines
    #[arg(long, short = 'n', value_name = "LINES", default_value = "-10")]
    lines: TakeValue,
    /// Number of bytes
    #[arg(long, short = 'c', value_name = "BYTES", conflicts_with = "lines")]
    bytes: Option<TakeValue>,
    /// Suppress headers
    #[arg(short, long)]
    quiet: bool,
}

pub fn run(config: Config) -> Result<()> {
    println!("{:?}", config);
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_parse_take_value() {
        assert_eq!("+0".parse::<TakeValue>(), Ok(PlusZero));
        assert_eq!("-0".parse::<TakeValue>(), Ok(TakeNum(0)));
        assert_eq!("-0".parse::<TakeValue>(), Ok(TakeNum(0)));
        assert_eq!("+1".parse::<TakeValue>(), Ok(TakeNum(1)));
        assert_eq!("-1".parse::<TakeValue>(), Ok(TakeNum(-1)));
        assert_eq!("1".parse::<TakeValue>(), Ok(TakeNum(-1)));
        assert_eq!(
            "a".parse::<TakeValue>(),
            Err("illegal offset -- a: Invalid argument".to_string())
        );
    }

    #[test]
    fn test_take_value_from_str() {
        assert_eq!(TakeValue::from_str("+0"), Ok(PlusZero));
        assert_eq!(TakeValue::from_str("-0"), Ok(TakeNum(0)));
        assert_eq!(TakeValue::from_str("-0"), Ok(TakeNum(0)));
        assert_eq!(TakeValue::from_str("+1"), Ok(TakeNum(1)));
        assert_eq!(TakeValue::from_str("-1"), Ok(TakeNum(-1)));
        assert_eq!(TakeValue::from_str("1"), Ok(TakeNum(-1)));
        assert_eq!(
            TakeValue::from_str("a"),
            Err("illegal offset -- a: Invalid argument".to_string())
        );
    }
}
