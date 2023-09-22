use clap::{Args, Parser, Subcommand};
use owo_colors::{colors::*, OwoColorize};
use std::{env, error::Error, fs, process};
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// The string that you are looking for
    #[arg(required = true)]
    query: String,

    /// File path for snatch to read
    #[arg(long, required = false)]
    file: Option<String>,

    /// Makes snatch case insensitive
    #[arg(short, long)]
    insensitive: bool,

    /// Prints the entire file with query highlighted
    #[arg(short, long)]
    verbose: bool,
}

pub fn run(cli: &Cli) -> Result<(), Box<dyn Error>> {
    if let Some(file) = &cli.file {
        let contents = fs::read_to_string(file)?;
        for line in search(&cli.query, &contents, &cli.insensitive) {
            println!(
                "{}",
                line.replace(&cli.query, &cli.query.bright_green().to_string())
            );
        }
    } else if let None = &cli.file {
        if let Err(e) = dir_search(&cli.query) {
            println!("Dir search fun error {e}");
            process::exit(1);
        }
    }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str, insensitive: &bool) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if *insensitive {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        } else {
            if line.contains(query) {
                results.push(line);
            }
        }
    }
    results
}
pub fn dir_search(query: &str) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let entries = fs::read_dir(current_dir)?;

    entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if file_name.contains(query) {
                Some(file_name)
            } else {
                None
            }
        })
        .for_each(|matching_file_name| {
            println!("{}", matching_file_name.cyan());
        });
    Ok(())
}
