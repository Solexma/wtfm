use clap::Parser;
use std::error::Error;
use wtfm::cli::args::{Cli, Commands};
use wtfm::commands::{analyze, author, edit, generate, info};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let debug = std::env::var("WTFM_DEBUG").is_ok();

    match &cli.command {
        Commands::Generate { .. } => generate::execute(&cli.command, debug)?,
        Commands::Analyze { .. } => analyze::execute(&cli.command, debug)?,
        Commands::Info { .. } => info::execute(&cli.command, debug)?,
        Commands::Author { .. } => author::execute(&cli.command, debug)?,
        Commands::Edit { .. } => edit::execute(&cli.command, debug)?,
    }

    Ok(())
}
