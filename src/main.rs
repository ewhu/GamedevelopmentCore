// src/main.rs
/*
 * Main executable for GamedevelopmentCore
 */

use clap::Parser;
use gamedevelopmentcore::{Result, run};

#[derive(Parser)]
#[command(version, about = GamedevelopmentCore - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
