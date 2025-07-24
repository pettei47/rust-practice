use anyhow::Result;
use chrono::{DateTime, Local};
use clap::Parser;
use std::fs::{self};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

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
    if config.long {
        println!("{}", format_output(&paths)?);
    } else {
        for path in paths {
            println!("{}", path.display());
        }
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

fn format_mode(mode: u32) -> String {
    let mut permissions = String::new();
    permissions.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    permissions.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    permissions.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    permissions
}

fn format_output(paths: &[PathBuf]) -> Result<String> {
    //         1   2     3     4     5     6     7     8
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let metadata = fs::metadata(path)?;
        let file_type = if metadata.is_dir() {
            "d"
        } else {
            "-"
        };
        let permissions = metadata.mode();
        let permissions_str = format_mode(permissions);
        let num_links = metadata.nlink();
        let uid = metadata.uid();
        let owner = get_user_by_uid(uid)
            .map_or_else(|| uid.to_string(), |user| user.name().to_string_lossy().into_owned());
        let gid = metadata.gid();
        let group = get_group_by_gid(gid)
            .map_or_else(|| gid.to_string(), |group| group.name().to_string_lossy().into_owned());
        let size = metadata.len();
        let modified_time: DateTime<Local> = DateTime::from(metadata.modified()?);
        let modified_time_str = modified_time.format("%Y-%m-%d %H:%M:%S").to_string();

        table.add_row(Row::new()
            .with_cell(file_type) // 1 type
            .with_cell(permissions_str) // 2 permissions
            .with_cell(num_links) // 3 num of link
            .with_cell(owner) // 4 owner
            .with_cell(group) // 5 group
            .with_cell(size.to_string()) // 6 size
            .with_cell(modified_time_str) // 7 modified time
            .with_cell(path.display().to_string()) // 8 path
        );
    };
    Ok(format!("{}", table))
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

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }

    fn long_match(
        line: &str,
        expected_name: &str,
        expected_perms: &str,
        expected_size: Option<&str>,
    ) {
        let parts: Vec<_> = line.split_whitespace().collect();
        assert!(!parts.is_empty() && parts.len() <= 10);

        let perms = parts.first().unwrap();
        assert_eq!(perms, &expected_perms);

        if let Some(size) = expected_size {
            let file_size = parts.get(4).unwrap();
            assert_eq!(file_size, &size);
        }

        let display_name = parts.last().unwrap();
        assert_eq!(display_name, &expected_name);
    }

    #[test]
    fn test_format_output_one() {
        let bustle_path = "tests/inputs/bustle.txt";
        let bustle = PathBuf::from(bustle_path);

        let res = format_output(&[bustle]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let lines: Vec<&str> =
            out.split('\n').filter(|s| !s.is_empty()).collect();
        assert_eq!(lines.len(), 1);

        let line1 = lines.first().unwrap();
        long_match(line1, bustle_path, "-rw-r--r--", Some("193"));
    }

    #[test]
    fn test_format_output_two() {
        let res = format_output(&[
            PathBuf::from("tests/inputs/dir"),
            PathBuf::from("tests/inputs/empty.txt"),
        ]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let mut lines: Vec<&str> =
            out.split('\n').filter(|s| !s.is_empty()).collect();
        lines.sort();
        assert_eq!(lines.len(), 2);

        let empty_line = lines.remove(0);
        long_match(
            empty_line,
            "tests/inputs/empty.txt",
            "-rw-r--r--",
            Some("0"),
        );

        let dir_line = lines.remove(0);
        long_match(dir_line, "tests/inputs/dir", "drwxr-xr-x", None);
    }
}