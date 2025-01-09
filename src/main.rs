mod cli;
mod commands;
mod config;
mod utils;

use cli::args::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Author) => commands::author::execute(&cli),
        Some(Commands::Generate) => commands::generate::execute(&cli),
        None => commands::generate::execute(&cli), // Default to generate command
    }
}