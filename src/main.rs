use std::process;

use api::*;
use clap::{Args, Parser, Subcommand};
fn main() {
    let cli = api::Cli::parse();
    // dbg!(cli);
    if let Err(e) = run(&cli) {
        println!("Run fun err: {e}");
        process::exit(1);
    }
}
