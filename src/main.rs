mod cli;
mod commands;
mod config;
mod utils;

use clap::Parser;
use cli::args::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Author) => commands::author::execute(&cli),
        Some(Commands::Generate) => commands::generate::execute(&cli),
        Some(Commands::Analyze) => commands::analyze::execute(&cli),
        None => commands::analyze::execute(&cli),
    }
}
