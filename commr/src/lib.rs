use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
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
    #[arg(short, long, default_value = "\t")]
    delimiter: String,
}

pub fn run(config: Config) -> Result<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(anyhow!("Both input files cannot be STDIN (\"-\")"));
    }

    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let mut lines1 = match open(file1) {
        Err(e) => return Err(anyhow!("{file1}: {e}")),
        Ok(file) => file.lines().map_while(Result::ok).map(case),
    };
    let mut lines2 = match open(file2) {
        Err(e) => return Err(anyhow!("{file2}: {e}")),
        Ok(file) => file.lines().map_while(Result::ok).map(case),
    };

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Less => {
                    if !config.suppress_unique1 {
                        println!("{}", val1);
                    }
                    line1 = lines1.next();
                }
                Greater => {
                    if !config.suppress_unique2 {
                        if config.suppress_unique1 {
                            println!("{}", val2);
                        } else {
                            println!("{}{}", config.delimiter, val2);
                        }
                    }
                    line2 = lines2.next();
                }
                Equal => {
                    if !config.suppress_common {
                        if config.suppress_unique1 {
                            if config.suppress_unique2 {
                                println!("{}", val1);
                            } else {
                                println!("{}{}", config.delimiter, val1);
                            }
                        } else if config.suppress_unique2 {
                            println!("{}{}", config.delimiter, val1);
                        } else {
                            println!("{}{}{}", config.delimiter, config.delimiter, val1);
                        }
                    }
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                if !config.suppress_unique1 {
                    println!("{}", val1);
                }
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                if !config.suppress_unique2 {
                    if config.suppress_unique1 {
                        println!("{}", val2);
                    } else {
                        println!("{}{}", config.delimiter, val2);
                    }
                }
                line2 = lines2.next();
            }
            _ => (),
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
