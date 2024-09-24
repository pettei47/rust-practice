use anyhow::{anyhow, bail, Result};
use clap::Parser;
use regex::Regex;
use std::num::NonZeroUsize;
use std::ops::Range;

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct ArgsExtract {
    /// Selected fields
    #[arg(short, long, value_name = "FIELDS")]
    fields: Option<String>,

    /// Selected bytes
    #[arg(short, long, value_name = "BYTES")]
    bytes: Option<String>,

    /// Selected chars
    #[arg(short, long, value_name = "CHARS")]
    chars: Option<String>,
}

#[derive(Debug, Parser)]
#[command(version, author, about)]
/// cutr command with Rust
pub struct Config {
    /// Files to process
    #[arg(value_name = "FILE", required = true, default_value = "-")]
    files: Vec<String>,
    /// Fields and ranges
    #[arg(short, long, value_name = "DELIM", default_value = "\t")]
    delim: String,

    #[command(flatten)]
    extract: ArgsExtract,
}

type PositionList = Vec<Range<usize>>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

fn parse_index(input: &str) -> Result<usize> {
    let value_error = || anyhow!(r#"illegal list value: "{input}""#);
    input
        .starts_with('+')
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|n| usize::from(n) - 1)
                .map_err(|_| value_error())
        })
}

fn parse_pos(range: String) -> Result<PositionList> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    range
        .split(',')
        .map(|val| {
            parse_index(val).map(|n| n..n + 1).or_else(|e| {
                range_re.captures(val).ok_or(e).and_then(|captures| {
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;
                    if n1 >= n2 {
                        bail!(
                            "First number in range ({}) \
                            must be lower than second number ({})",
                            n1 + 1,
                            n2 + 1
                        );
                    }
                    Ok(n1..n2 + 1)
                })
            })
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)
}

pub fn run(config: Config) -> Result<()> {
    println!("{:?}", config);
    let delim_bytes = config.delim.as_bytes();
    if delim_bytes.len() != 1 {
        bail!(r#"--delim "{}" must be a single byte"#, config.delim);
    }
    let delim: u8 = *delim_bytes.first().unwrap();

    let extract = if let Some(fields) = config.extract.fields.map(parse_pos).transpose()? {
        Extract::Fields(fields)
    } else if let Some(bytes) = config.extract.bytes.map(parse_pos).transpose()? {
        Extract::Bytes(bytes)
    } else if let Some(chars) = config.extract.chars.map(parse_pos).transpose()? {
        Extract::Chars(chars)
    } else {
        unreachable!("Must have --fields, --bytes, or --chars");
    };
    println!("{:?} {:?}", delim, extract);
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    // use super::{extract_bytes, extract_chars, extract_fields, parse_pos};
    use super::parse_pos;
    // use csv::StringRecord;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("".to_string()).is_err());

        // Zero is an error
        let res = parse_pos("0".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);

        let res = parse_pos("0-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);

        // A leading "+" is an error
        let res = parse_pos("+1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "+1""#,);

        let res = parse_pos("+1-2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "+1-2""#,
        );

        let res = parse_pos("1-+2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "1-+2""#,
        );

        // Any non-number is an error
        let res = parse_pos("a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_pos("1,a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_pos("1-a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "1-a""#,);

        let res = parse_pos("a-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a-1""#,);

        // Wonky ranges
        let res = parse_pos("-".to_string());
        assert!(res.is_err());

        let res = parse_pos(",".to_string());
        assert!(res.is_err());

        let res = parse_pos("1,".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-1".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-a".to_string());
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }

    // #[test]
    // fn test_extract_fields() {
    //     let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
    //     assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
    //     assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
    //     assert_eq!(extract_fields(&rec, &[0..1, 2..3]), &["Captain", "12345"]);
    //     assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
    //     assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
    // }

    // #[test]
    // fn test_extract_chars() {
    //     assert_eq!(extract_chars("", &[0..1]), "".to_string());
    //     assert_eq!(extract_chars("ábc", &[0..1]), "á".to_string());
    //     assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác".to_string());
    //     assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
    //     assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "cb".to_string());
    //     assert_eq!(extract_chars("ábc", &[0..1, 1..2, 4..5]), "áb".to_string());
    // }

    // #[test]
    // fn test_extract_bytes() {
    //     assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
    //     assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
    //     assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
    //     assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
    //     assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
    //     assert_eq!(extract_bytes("ábc", &[0..2, 5..6]), "á".to_string());
    // }
}
