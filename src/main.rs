mod cli;
mod error;
mod models;
mod runner;
mod task;
mod tools;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = runner::run(cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
