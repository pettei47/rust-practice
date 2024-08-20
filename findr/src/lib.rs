// use crate::EntryType::*;
use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

#[derive(Debug, Parser)]
#[command(version, author, about)]
/// findr command with Rust
pub struct Config {
    /// Search paths
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// Names
    #[arg(
        short('n'),
        long("name"),
        value_name = "NAME",
        value_parser(Regex::new),
        action(ArgAction::Append),
        num_args(0..)
    )]
    names: Vec<Regex>,

    /// Entry types
    #[arg(
        short('t'),
        long("type"),
        value_name = "TYPE",
        value_parser(clap::value_parser!(EntryType)),
        action(ArgAction::Append),
        num_args(0..)
    )]
    entry_types: Vec<EntryType>,
}

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &walkdir::DirEntry| {
        let file_type = if entry.file_type().is_dir() {
            EntryType::Dir
        } else if entry.file_type().is_file() {
            EntryType::File
        } else if entry.file_type().is_symlink() {
            EntryType::Link
        } else {
            return false;
        };
        config.entry_types.is_empty() || config.entry_types.contains(&file_type)
    };

    let name_filter = |entry: &walkdir::DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(entry.file_name().to_str().unwrap()))
    };

    for path in config.paths {
        let entry_names = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Ok(e) => Some(e),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|e| e.path().display().to_string())
            .collect::<Vec<String>>();
        println!("{}", entry_names.join("\n"));
    }
    Ok(())
}
