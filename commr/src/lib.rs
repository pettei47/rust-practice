use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// comm clone written in Rust
pub struct Config {
    /// Input file 1
    #[arg(value_name = "FILE1")]
    file1: String,

    /// Input file 2
    #[arg(value_name = "FILE2")]
    file2: String,

    /// Suppress printing of column 1 (lines unique to FILE1)
    #[arg(short = '1', long)]
    suppress_unique1: bool,

    /// Suppress printing of column 2 (lines unique to FILE2)
    #[arg(short = '2', long)]
    suppress_unique2: bool,

    /// Suppress printing of column 3 (lines common to both files)
    #[arg(short = '3', long)]
    suppress_common: bool,

    /// Case insensitive comparison of lines
    #[arg(short, long)]
    insensitive: bool,

    /// Output delimiter
    #[arg(short, long = "output-delimiter", default_value = "\t")]
    delimiter: String,
}

pub fn run(config: Config) -> Result<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(anyhow!("Both input files cannot be STDIN (\"-\")"));
    }

    let _file1 = match open(file1) {
        Err(e) => return Err(anyhow!("{file1}: {e}")),
        Ok(file) => file,
    };
    let _file2 = match open(file2) {
        Err(e) => return Err(anyhow!("{file2}: {e}")),
        Ok(file) => file,
    };

    println!("Opened {} and {}", file1, file2);

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
