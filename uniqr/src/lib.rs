use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

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
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut prev_line = String::new();
    let mut count = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        // println!("{:?}", bytes);
        // println!("{:?}", line);
        // println!("{:?}", prev_line);
        if prev_line.is_empty() && count == 0 {
            prev_line = line.clone();
            line.clear();
            continue;
        }
        count += 1;
        if line != prev_line {
            if config.count {
                println!("{:>4} {}", count, prev_line.replace('\n', ""));
            } else {
                println!("{}", prev_line.replace('\n', ""));
            }
            count = 0;
        }
        if bytes == 0 {
            break;
        }
        prev_line.clear();
        prev_line = line.clone();
        line.clear();
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
