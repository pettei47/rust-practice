use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// grep clone written in Rust
pub struct Config {
    /// Search pattern
    #[arg()]
    pattern: String,

    /// Input file(s)
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    /// Case insensitive search
    #[arg(short, long("ignore-case"))]
    insensitive: bool,

    /// Recursive search
    #[arg(short, long)]
    recursive: bool,

    /// Count occurrences
    #[arg(short, long)]
    count: bool,

    /// Invert match
    #[arg(short('v'), long("invert-match"))]
    invert: bool,
}

pub fn run(config: Config) -> Result<()> {
    let pattern = regex::RegexBuilder::new(&config.pattern)
        .case_insensitive(config.insensitive)
        .build()
        .map_err(|_| anyhow!(r#"Invalid pattern "{}""#, config.pattern))?;
    println!("{:?}", config);
    println!("{:?}", pattern);
    Ok(())
}
