use std::fs;

use clap::Parser;
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(value_name = "PATTERN", help = "Search pattern")]
    pattern: Regex,
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,
    #[arg(short, long, help = "Recursive search")]
    recursive: bool,
    #[arg(short, long, help = "Count occurrences")]
    count: bool,
    #[arg(short = 'v', long, help = "Invert match")]
    invert_match: bool,
    #[arg(short, long, help = "Case insensitive")]
    insensitive: bool,
}

pub fn get_cli() -> MyResult<Cli> {
    let mut cli = Cli::parse();
    cli.pattern = RegexBuilder::new(&cli.pattern.to_string())
        .case_insensitive(cli.insensitive)
        .build()?;

    Ok(cli)
}

fn find_files(paths: &[String], recursive: bool) -> Vec<MyResult<String>> {
    let mut results: Vec<MyResult<String>> = vec![];

    for path in paths {
        match path.as_str() {
            "-" => results.push(Ok(path.to_string())),
            _ => match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        results.push(Ok(path.to_string()));
                        continue;
                    }

                    if !recursive && metadata.is_dir() {
                        results.push(Err(format!("{} is a directory", path).into()));
                        break;
                    }

                    for entry in WalkDir::new(path)
                        .into_iter()
                        .flatten()
                        .filter(|e| e.file_type().is_file())
                    {
                        results.push(Ok(entry.path().display().to_string()))
                    }
                }
                Err(e) => {
                    results.push(Err(format!("{}: {}", path, e).into()));
                    break;
                }
            }
        }
    }

    results
}

pub fn run(cli: Cli) -> MyResult<()> {
    println!("{:#?}", cli);
    Ok(())
}

#[cfg(test)]
mod tests {
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    use super::find_files;

    #[test]
    fn test_find_files() {
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory")
        }

        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        let bad: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}
