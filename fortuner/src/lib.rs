use anyhow::{anyhow, Result};
use clap::Parser;
use regex::RegexBuilder;

/// fortune command with Rust
#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct Config {
    /// Input file(s) or directory(s)
    #[arg(value_name = "FILE", required = true)]
    sources: Vec<String>,
    /// pattern to search for
    #[arg(short = 'm', long, value_name = "PATTERN")]
    pattern: Option<String>,
    /// seed for random number generator
    #[arg(short, long, value_name = "SEED")]
    seed: Option<u64>,
    /// Case insensitive pattern matching
    #[arg(short, long)]
    insensitive: bool,
}

pub fn run(config: Config) -> Result<()> {
    println!("{:?}", config);
    let pattern = config.pattern.map(|val: String| {
        RegexBuilder::new(val.as_str())
            .case_insensitive(config.insensitive)
            .build()
            .map_err(|_| anyhow!(r#"Invalid --pattern "{val}""#))
    })
    .transpose()?;
    println!("{:?}", pattern);
    Ok(())
}
