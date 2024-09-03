use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
use regex::Regex;
use std::error::Error;
use std::ops::Range;

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug, Parser)]
#[command(version, author, about)]
/// cutr command with Rust
pub struct Config {
    /// Files to process
    #[arg(value_name = "FILE", required = true, default_value = "-")]
    files: Vec<String>,
    /// Fields and ranges
    #[arg(short('d'), value_name = "DELIM", default_value = "\t")]
    delimiter: u8,
    /// Extract fields
    #[arg(short('f'), long("fields"), value_name = "FIELDS")]
    extract: Extract,
}
