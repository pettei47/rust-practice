use clap:: {App, Arg};
use std::{default, error::Error};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
 }

 pub fn get_args() -> MyResult<Config> {
  let matches = App::new("wcr")
    .version("0.1.0")
    .author("teppei")
    .about("Rust wc")
    .arg(
      Arg::with_name("files")
        .value_name("FILE")
        .help("Input file(s)")
        .multiple(true)
        .default_value("-"),
    )
    .arg(
      Arg::with_name("lines")
        .value_name("LINE")
        .help("Count lines")
        .short("l")
        .long("lines")
        .takes_value(false),
    )
    .arg(
      Arg::with_name("words")
        .value_name("WORD")
        .help("Count words")
        .short("w")
        .long("words")
        .takes_value(false),
    )
    .arg(
      Arg::with_name("bytes")
        .value_name("BYTE")
        .help("Count bytes")
        .short("c")
        .long("bytes")
        .takes_value(false),
    )
    .arg(
      Arg::with_name("chars")
        .value_name("CHAR")
        .help("Count characters")
        .short("m")
        .long("chars")
        .conflicts_with("bytes")
        .takes_value(false),
    )
    .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|&v| !v) {
      lines = true;
      words = true;
      bytes = true;
    }

    Ok(Config {
      files: matches.values_of_lossy("files").unwrap(),
      lines,
      words,
      bytes,
      chars,
    })
 }

pub fn run(config: Config) -> MyResult<()> {
  println!("{:?}", config);
  Ok(())
}
