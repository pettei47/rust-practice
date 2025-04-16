use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use anyhow::{anyhow, Result, bail};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use std::ffi::OsStr;
use std::fs::{self, File};
use walkdir::WalkDir;
use clap::Parser;
use regex::RegexBuilder;

/// fortune command with Rust
#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct Config {
    /// Input file(s) or directory(s)
    #[arg(value_name = "FILE", required = true)]
    sources: Vec<String>,
    /// pattern to search for
    #[arg(short = 'm', long, value_name = "PATTERN")]
    pattern: Option<String>,
    /// seed for random number generator
    #[arg(short, long, value_name = "SEED")]
    seed: Option<u64>,
    /// Case insensitive pattern matching
    #[arg(short, long)]
    insensitive: bool,
}

#[derive(Debug)]
struct Fortune {
    source: String,
    text: String,
}

pub fn run(config: Config) -> Result<()> {
    let pattern = config.pattern.map(|val: String| {
        RegexBuilder::new(val.as_str())
            .case_insensitive(config.insensitive)
            .build()
            .map_err(|_| anyhow!(r#"Invalid --pattern "{val}""#))
    })
    .transpose()?;
    let paths = find_files(&config.sources)?;
    let fortunes = read_fortunes(&paths)?;
    
    if pattern.is_some() {
        let pattern = pattern.unwrap();
        let filtered_fortunes: Vec<Fortune> = fortunes
            .into_iter()
            .filter(|f| pattern.is_match(&f.text))
            .collect();
        if filtered_fortunes.is_empty() {
            bail!("No fortunes found matching the pattern");
        }
        println!("{}", pick_fortune(&filtered_fortunes, config.seed).unwrap());
    } else {
        println!("{}", pick_fortune(&fortunes, config.seed).unwrap());
    }

    Ok(())
}

fn find_files(paths: &[String]) -> Result<Vec<PathBuf>> {
    let dat = OsStr::new("dat");
    let mut files = vec![];

    for path in paths {
        match fs::metadata(path) {
            Err(e) => bail!("{path}: {e}"),
            Ok(_) => files.extend(
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| {
                        e.file_type().is_file()
                            && e.path().extension() != Some(dat)
                    })
                    .map(|e| e.path().into()),
            ),
        }
    }

    files.sort();
    files.dedup();
    Ok(files)
}

fn read_fortunes(paths: &[PathBuf]) -> Result<Vec<Fortune>> {
    let mut fortunes = vec![];
    let mut buffer = vec![];

    for path in paths {
        let basename = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let file = File::open(path).map_err(|e| anyhow!("{}: {e}", path.to_string_lossy()))?;

        for line in BufReader::new(file).lines().map_while(Result::ok) {
            if line == "%" {
                if !buffer.is_empty() {
                    fortunes.push(Fortune {
                        source: basename.clone(),
                        text: buffer.join("\n"),
                    });
                    buffer.clear();
                }
            } else {
                buffer.push(line.to_string());
            }
        }
    }

    Ok(fortunes)
}

fn pick_fortune(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    let mut rng: Box<dyn RngCore> = match seed {
        Some(seed) => Box::new(StdRng::seed_from_u64(seed)),
        None => Box::new(rand::thread_rng()),
    };
    fortunes.choose(&mut rng).map(|f| f.text.to_string())
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let res = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/jokes"
        );

        // Fails to find a bad file
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        // Finds all the input files, excludes ".dat"
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());

        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));

        // Test for multiple sources, path must be unique and sorted
        let res = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string())
        }
    }

    #[test]
    fn test_read_fortunes() {
        // Parses all the fortunes without a filter
        let res = read_fortunes(&[PathBuf::from("./tests/inputs/jokes")]);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            println!("{:?}", fortunes);
            // Correct number and sorting
            assert_eq!(fortunes.len(), 6);
            assert_eq!(fortunes.first().unwrap().source, "jokes");
            assert_eq!(
                fortunes.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\n\
                A. Collared greens."
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "Q: What do you call a deer wearing an eye patch?\n\
                A: A bad idea (bad-eye deer)."
            );
        }

        // Filters for matching text
        let res = read_fortunes(&[
            PathBuf::from("./tests/inputs/jokes"),
            PathBuf::from("./tests/inputs/quotes"),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 11);
    }

    #[test]
    fn test_pick_fortune() {
        // Create a slice of fortunes
        let fortunes = &[
            Fortune {
                source: "fortunes".to_string(),
                text: "You cannot achieve the impossible without \
                        attempting the absurd."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Assumption is the mother of all screw-ups."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ];

        // Pick a fortune with a seed
        assert_eq!(
            pick_fortune(fortunes, Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}