use clap::Parser;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
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
        let line_trim_end = line.trim_end();
        if line_trim_end != prev_line.trim_end() && count > 0 {
            let output = make_output(&prev_line, count, &config);
            if let Some(out_file) = &config.out_file {
                let mut out = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(out_file)?;
                write!(out, "{}", output)?;
            } else {
                print!("{}", output);
            }
            count = 0;
        }
        count += 1;
        if bytes == 0 {
            break;
        }
        if prev_line.trim_end() != line_trim_end {
            prev_line.clear();
            prev_line = line.clone();
        }
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

fn make_output(prev_line: &str, count: usize, config: &Config) -> String {
    if config.count {
        format!("{:>4} {}", count, prev_line)
    } else {
        prev_line.to_string()
    }
}
