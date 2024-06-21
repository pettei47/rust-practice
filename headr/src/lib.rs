use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

/// head command with Rust
#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct Config {
  /// Input file(s)
  #[arg(value_name = "FILE", default_value = "-")]
  files: Vec<String>,
  /// Number of lines
  #[arg(
    long,
    short = 'n',
    value_name = "LINES",
    default_value = "10",
    value_parser = clap::value_parser!(u64).range(1..),
  )]
  lines: u64,
  /// Number of bytes
  #[arg(long, short = 'c', value_name = "BYTES", conflicts_with = "lines")]
  bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
  let num_files = config.files.len();

  for (file_num, filename) in config.files.iter().enumerate() {
    match open(filename) {
      Err(e) => eprintln!("{}: {}", filename, e),
      Ok(mut file) => {
        if num_files > 1 {
          println!(
            "{}==> {} <==",
            if file_num > 0 { "\n" } else { "" },
            filename
          );
        }
        if let Some(num_bytes) = config.bytes {
          let bytes: Result<Vec<_>, _> = file.bytes().take(num_bytes).collect();
          print!("{}", String::from_utf8_lossy(&bytes?));
        } else {
          let mut line = String::new();
          for _ in 0..config.lines {
            if file.read_line(&mut line)? == 0 {
              break;
            }
            print!("{}", line);
            line.clear();
          }
        }
      }
    }
  }
  Ok(())
}

// fn parse_positive_int(val: &str) -> MyResult<usize> {
//   match val.parse() {
//     Ok(n) if n > 0 => Ok(n),
//     _ => Err(val.into()),
//   }
// }

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

// #[test]
// fn test_parse_positive_int() {
//   let res = parse_positive_int("3");
//   assert!(res.is_ok());
//   assert_eq!(res.unwrap(), 3);

//   let res = parse_positive_int("foo");
//   assert!(res.is_err());
//   assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

//   let res = parse_positive_int("0");
//   assert!(res.is_err());
//   assert_eq!(res.unwrap_err().to_string(), "0".to_string());
// }
