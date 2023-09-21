use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "xyz", version = "0.1", about = "Idk you tell me")]
pub struct Cli {
    /// The string that you are looking for
    #[arg(required = true)]
    query: String,

    /// File path for the snatch to read
    #[arg(long)]
    file: String,

    /// Makes snatch case insensitive
    #[arg(short, long)]
    insensitive: bool,
}
