use anyhow::Result;
use clap::Parser;
use std::fs::{self};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `ls`
pub struct Config {
    /// Paths to list
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,
    /// Long format
    #[arg(short, long)]
    long: bool,
    /// Show all files
    #[arg(short = 'a', long = "all")]
    show_hidden: bool,
}

pub fn run(config: Config) -> Result<()> {
    let paths = find_files(&config.paths, config.show_hidden)?;
    for path in paths {
        // if config.long {
        //     let metadata = fs::metadata(&path)?;
        //     println!("{:?} - {:?}", path, metadata);
        // } else {
            println!("{}", path.display());
        // }
    }
    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> Result<Vec<PathBuf>> {
    let mut files = vec![];

    for path in paths {
        match fs::metadata(path) {
            Err(e) => eprintln!("{path}: {e}"),
            Ok(meta) => {
                if meta.is_dir() {
                    let entries = fs::read_dir(path)?;
                    for entry in entries {
                        let entry = entry?;
                        let is_hidden = entry.file_name().to_string_lossy().starts_with(".");
                        if show_hidden || !is_hidden {
                            files.push(entry.path());
                        }
                    }
                } else {
                    files.push(PathBuf::from(path));
                }
            }
        }
    }

    Ok(files)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_files() {
        // Find all non-hidden entries in a directory
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );

        // Any existing file should be found even if hidden
        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        // Test multiple path arguments
        let res = find_files(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }

    #[test]
    fn test_find_files_hidden() {
        // Find all entries in a directory including hidden
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
    }
}