use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  dbg!(config);
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
