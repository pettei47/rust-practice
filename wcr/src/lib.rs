use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let file_info = count(file)?;
                total_lines += file_info.num_lines;
                total_words += file_info.num_words;
                total_bytes += file_info.num_bytes;
                total_chars += file_info.num_chars;
                print_count(&config, file_info, filename);
            }
        }
    }
    if config.files.len() > 1 {
        let total_info = FileInfo {
            num_lines: total_lines,
            num_words: total_words,
            num_bytes: total_bytes,
            num_chars: total_chars,
        };
        print_count(&config, total_info, "total");
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += line_bytes;
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn print_count(config: &Config, file_info: FileInfo, filename: &str) {
    if config.lines {
        print!("{:8}", file_info.num_lines);
    }
    if config.words {
        print!("{:8}", file_info.num_words);
    }
    if config.bytes {
        print!("{:8}", file_info.num_bytes);
    }
    if config.chars {
        print!("{:8}", file_info.num_chars);
    }
    if filename != "-" {
        print!(" {}", filename);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world, I just want your half\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 47,
            num_chars: 47,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
