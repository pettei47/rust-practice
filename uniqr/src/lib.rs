use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(version, author, about)]
/// uniqr command with Rust
pub struct Config {
    /// Input file
    #[arg(value_name = "FILE", default_value = "-")]
    in_file: String,
    /// Output file
    #[arg(value_name = "FILE")]
    out_file: Option<String>,
    /// Count
    #[arg(short, long, value_name = "COUNT")]
    count: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
