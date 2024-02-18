use clap::Parser;
use regex::{Regex, RegexBuilder};

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

pub fn run(cli: Cli) -> MyResult<()> {
    println!("{:#?}", cli);
    Ok(())
}
