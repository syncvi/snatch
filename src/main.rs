use api::*;
use clap::{Args, Parser, Subcommand};
fn main() {
    let cli = api::Cli::parse();
    dbg!(cli);
}
