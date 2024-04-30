use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

pub fn run(args: Args) -> MyResult<()> {
  for filename in args.files {
    match open(&filename) {
        Err(e) => eprintln!("Failed to open {}: {}", filename, e),
        Ok(file) => {
          let mut non_blank_line_number = 1;
          for (line_number, line) in file.lines().enumerate() {
            let line = line?;
            if args.number_lines {
              print!("{:6}\t", line_number + 1);
            } else if args.number_non_blank_lines && !line.trim().is_empty() {
              print!("{:6}\t", non_blank_line_number);
              non_blank_line_number += 1;
            }
            println!("{}", line);
          }
        },
    }
  }
  Ok(())
}

/// cat command with Rust
#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct Args {
  /// Input file(s)
  #[arg(value_name = "FILE", default_value = "-")]
  files: Vec<String>,
  /// Number all output lines
  #[arg(long = "number", short, conflicts_with = "number_non_blank_lines")]
  number_lines: bool,
  /// Number non-blank output lines
  #[arg(long = "number-nonblank", short = 'b', conflicts_with = "number_lines")]
  number_non_blank_lines: bool,
}
