use clap::{Args, Parser, Subcommand};
use std::{env, error::Error, fs};
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// The string that you are looking for
    #[arg(required = true)]
    query: String,

    /// File path for snatch to read
    #[arg(long, required = false)]
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
    } else if cli.file.is_empty() {
        let current_dir = env::current_dir()?;
        let entries = fs::read_dir(current_dir)?;

        entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_name = entry.file_name().to_string_lossy().into_owned();
                if file_name.contains(&cli.query) {
                    Some(file_name)
                } else {
                    None
                }
            })
            .for_each(|matching_file_name| {
                println!("{}", matching_file_name);
            });
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
