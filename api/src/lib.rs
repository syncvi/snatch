use clap::Parser;
use owo_colors::OwoColorize;
use std::{
    env,
    error::Error,
    fs, process,
    sync::{Arc, Mutex},
    thread,
};
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

    /// Enables mutli-threading
    #[arg(short, long)]
    threading: bool,

    /// Prints the entire file with query highlighted
    #[arg(short, long)]
    verbose: bool,
}

pub fn run(cli: &Cli) -> Result<(), Box<dyn Error>> {
    if let Some(file) = &cli.file {
        let contents = fs::read_to_string(file)?;
        match cli.threading {
            false => {
                for line in search(&cli.query, &contents, &cli.insensitive) {
                    let highlighted_line = highlighter(&line, &cli.query);
                    println!("{}", highlighted_line);
                }
            }
            true => {
                for line in searchp(&cli.query, &contents, &cli.insensitive) {
                    let highlighted_line = highlighter(&line, &cli.query);
                    println!("{}", highlighted_line);
                }
            }
        }
    } else if let None = &cli.file {
        if let Err(e) = dir_search(&cli.query, &cli.insensitive) {
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
            if line.contains(&query) {
                results.push(line);
            }
        }
    }
    results
}

fn searchp(query: &str, contents: &str, insensitive: &bool) -> Vec<String> {
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let chunk_size = (lines.len() + 3) / 4; // Divide into four roughly equal parts

    let query_arc = Arc::new(query.to_string());

    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    if *insensitive {
        // Spawn threads for case-insensitive search
        for i in 0..4 {
            let query_arc_clone = Arc::clone(&query_arc);
            let lines_chunk = lines
                .iter()
                .skip(i * chunk_size)
                .take(chunk_size)
                .cloned()
                .collect::<Vec<String>>();
            let results_clone = Arc::clone(&results);

            let handle = thread::spawn(move || {
                let query = query_arc_clone.as_str();
                let mut matching_lines = Vec::new();

                for line in &lines_chunk {
                    if line.to_lowercase().contains(&query.to_lowercase()) {
                        matching_lines.push(line.clone());
                    }
                }

                let mut results_lock = results_clone.lock().unwrap();
                results_lock.extend(matching_lines);
            });

            handles.push(handle);
        }
    } else {
        // Spawn threads for case-sensitive search
        for i in 0..4 {
            let query_arc_clone = Arc::clone(&query_arc);
            let lines_chunk = lines
                .iter()
                .skip(i * chunk_size)
                .take(chunk_size)
                .cloned()
                .collect::<Vec<String>>();
            let results_clone = Arc::clone(&results);

            let handle = thread::spawn(move || {
                let query = query_arc_clone.as_str();
                let mut matching_lines = Vec::new();

                for line in &lines_chunk {
                    if line.contains(query) {
                        matching_lines.push(line.clone());
                    }
                }

                let mut results_lock = results_clone.lock().unwrap();
                results_lock.extend(matching_lines);
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results_lock = results.lock().unwrap();
    results_lock.clone()
}

pub fn dir_search(query: &str, insensitive: &bool) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let entries = fs::read_dir(current_dir)?;

    entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if *insensitive {
                if file_name.to_lowercase().contains(&query.to_lowercase()) {
                    Some(file_name)
                } else {
                    None
                }
            } else {
                if file_name.contains(&query) {
                    Some(file_name)
                } else {
                    None
                }
            }
        })
        .for_each(|matching_file_name| {
            println!("{}", matching_file_name.cyan());
        });
    Ok(())
}
pub fn highlighter(line: &str, query: &str) -> String {
    let mut highlighted_line = String::new();
    let mut line = &line[..];

    while let Some(index) = line.to_lowercase().find(&query.to_lowercase()) {
        // append the text all the way to match (non inclusive)
        highlighted_line.push_str(&line[..index]);

        // append the matched substring in its original case but highlighted
        let matched_query = &line[index..index + query.len()];
        highlighted_line.push_str(&matched_query.bright_green().to_string());

        // move the line cursor past the matched query
        line = &line[index + query.len()..];
    }

    // append any remaining text
    highlighted_line.push_str(line);

    highlighted_line
}
