// use crate::EntryType::*;
use clap::Parser;
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug, Parser)]
#[command(version, author, about)]
/// findr command with Rust
pub struct Config {
    /// Search paths (default: ".")
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// Name
    #[arg(short, long = "name", value_name = "NAME")]
    names: Vec<Regex>,

    /// Entry type [possible values: f, d, l]
    #[arg(
      short = 't',
      long = "type",
      value_name = "TYPE",
      value_parser = parse_entry_type,
    )]
    entry_types: Vec<EntryType>,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_entry_type(s: &str) -> Result<EntryType, String> {
    match s {
        "f" => Ok(EntryType::File),
        "d" => Ok(EntryType::Dir),
        "l" => Ok(EntryType::Link),
        _ => Err("Invalid entry type".to_string()),
    }
}
