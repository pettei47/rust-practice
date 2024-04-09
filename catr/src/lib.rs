use clap::{App, Arg};
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

pub fn run(config: Config) -> MyResult<()> {
  for filename in config.files {
    match open(&filename) {
        Err(e) => eprintln!("Failed to open {}: {}", filename, e),
        Ok(file) => {
          let mut line_number = 1;
          let mut non_blank_line_number = 1;
          for line in file.lines() {
            let line = line?;
            if config.number_lines {
              print!("{:6}\t", line_number);
              line_number += 1;
            } else if config.number_non_blank_lines && !line.trim().is_empty() {
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

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool,
  number_non_blank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("catr")
    .version("0.1.0")
    .author("teppei")
    .about("cat command with Rust")
    .arg(
      Arg::with_name("files")
        .value_name("FILE")
        .help("Input file(s)")
        .multiple(true)
        .default_value("-")
    )
    .arg(
      Arg::with_name("number_lines")
        .long("number")
        .short("n")
        .help("Number all output lines")
        .takes_value(false)
        .conflicts_with("number_nonblank"),
    )
    .arg(
      Arg::with_name("number_nonblank")
        .long("number-nonblank")
        .short("b")
        .help("Number non-blank output lines")
        .takes_value(false),
    )
    .get_matches();

  Ok(Config {
    files: matches.values_of_lossy("files").unwrap(),
    number_lines: matches.is_present("number_lines"),
    number_non_blank_lines: matches.is_present("number_nonblank"),
  })
}
