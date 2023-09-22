use clap::{Args, Parser, Subcommand};
use std::{error::Error, fs};
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// The string that you are looking for
    #[arg(required = true)]
    query: String,

    /// File path for snatch to read
    #[arg(long)]
    file: String,

    /// Makes snatch case insensitive
    #[arg(short, long)]
    insensitive: bool,

    /// Prints the entire file with query highlighted
    #[arg(short, long)]
    verbose: bool,
}

pub fn run(cli: &Cli) -> Result<(), Box<dyn Error>> {
    if !&cli.file.is_empty() {
        let contents = fs::read_to_string(&cli.file)?;
        for line in search(&cli.query, &contents) {
            println!("{line}");
        }
    }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
