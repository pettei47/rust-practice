use crate::TakeValue::*;
use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

impl FromStr for TakeValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+0" => Ok(PlusZero),
            _ => match s.parse::<i64>() {
                Ok(num) if s.starts_with('+') || num < 0 => Ok(TakeNum(num)),
                Ok(num) => Ok(TakeNum(-num)),
                Err(_) => Err(format!("illegal offset -- {}: Invalid argument", s)),
            },
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
    for (file_num, filename) in config.files.iter().enumerate() {
        let num_files = config.files.len();
        match File::open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                let (lines, bytes) = count_lines_bytes(filename)?;
                if !config.quiet && num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(output_bytes) = &config.bytes {
                    print_bytes(file, output_bytes, bytes)?;
                } else {
                    print_lines(BufReader::new(file), &config.lines, lines)?;
                }
            }
        }
    }
    Ok(())
}

fn count_lines_bytes(filename: &str) -> Result<(u64, u64)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut lines = 0;
    let mut bytes = 0;
    let mut buffer = Vec::new();
    loop {
        let bytes_read = file.read_until(b'\n', &mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        lines += 1;
        bytes += bytes_read as u64;
        buffer.clear();
    }
    Ok((lines, bytes))
}

fn print_lines(mut file: impl BufRead, output_lines: &TakeValue, total_lines: u64) -> Result<()> {
    if let Some(start) = get_start_index(output_lines, total_lines) {
        let mut line_num = 0;
        let mut buffer = String::new();
        loop {
            let bytes_read = file.read_line(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            if line_num >= start {
                print!("{}", buffer);
            }
            line_num += 1;
            buffer.clear();
        }
    }
    Ok(())
}

fn print_bytes<T: Read + Seek>(
    mut file: T,
    output_bytes: &TakeValue,
    total_bytes: u64,
) -> Result<()> {
    if let Some(start) = get_start_index(output_bytes, total_bytes) {
        file.seek(io::SeekFrom::Start(start))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        if !buffer.is_empty() {
            print!("{}", String::from_utf8_lossy(&buffer));
        }
    }
    Ok(())
}

fn get_start_index(take_val: &TakeValue, total: u64) -> Option<u64> {
    match take_val {
        PlusZero => match total {
            0 => None,
            _ => Some(0),
        },
        TakeNum(n) => {
            let abs_n = n.unsigned_abs();
            if *n == 0 || total == 0 || (*n > 0 && abs_n > total) {
                None
            } else {
                let start = if *n < 0 {
                    total as i64 - abs_n as i64
                } else {
                    abs_n as i64 - 1
                };
                Some(if start < 0 { 0 } else { start as u64 })
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::{
        count_lines_bytes, get_start_index,
        TakeValue::{self, *},
    };
    use std::str::FromStr;

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

    #[test]
    fn test_count_lines_bytes() {
        assert_eq!(count_lines_bytes("tests/inputs/empty.txt").unwrap(), (0, 0));
        assert_eq!(count_lines_bytes("tests/inputs/one.txt").unwrap(), (1, 24));
        assert_eq!(count_lines_bytes("tests/inputs/two.txt").unwrap(), (2, 23));
        assert_eq!(
            count_lines_bytes("tests/inputs/three.txt").unwrap(),
            (3, 27)
        );
        assert_eq!(
            count_lines_bytes("tests/inputs/twelve.txt").unwrap(),
            (12, 63)
        );
    }

    #[test]
    fn test_get_start_index() {
        // +0 from an empty file (0 lines/bytes) returns None
        assert_eq!(get_start_index(&PlusZero, 0), None);

        // +0 from a nonempty file returns an index that
        // is one less than the number of lines/bytes
        assert_eq!(get_start_index(&PlusZero, 1), Some(0));

        // Taking 0 lines/bytes returns None
        assert_eq!(get_start_index(&TakeNum(0), 1), None);

        // Taking any lines/bytes from an empty file returns None
        assert_eq!(get_start_index(&TakeNum(1), 0), None);

        // Taking more lines/bytes than is available returns None
        assert_eq!(get_start_index(&TakeNum(2), 1), None);

        // When starting line/byte is less than total lines/bytes,
        // return one less than starting number
        assert_eq!(get_start_index(&TakeNum(1), 10), Some(0));
        assert_eq!(get_start_index(&TakeNum(2), 10), Some(1));
        assert_eq!(get_start_index(&TakeNum(3), 10), Some(2));

        // When starting line/byte is negative and less than total,
        // return total - start
        assert_eq!(get_start_index(&TakeNum(-1), 10), Some(9));
        assert_eq!(get_start_index(&TakeNum(-2), 10), Some(8));
        assert_eq!(get_start_index(&TakeNum(-3), 10), Some(7));

        // When the starting line/byte is negative and more than the total,
        // return 0 to print the whole file
        assert_eq!(get_start_index(&TakeNum(-20), 10), Some(0));
    }
}
