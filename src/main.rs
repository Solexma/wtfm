use clap::Parser;
use wtfm::cli::args::{Cli, Commands};
use wtfm::commands;
use wtfm::utils::logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    logger::init(cli.debug);

    match &cli.command {
        Some(Commands::Author) => commands::author::execute(&Commands::Author, cli.debug),
        Some(cmd @ Commands::Generate { .. }) => commands::generate::execute(cmd, cli.debug),
        Some(cmd @ Commands::Analyze { .. }) => commands::analyze::execute(cmd, cli.debug),
        None => commands::analyze::execute(
            &Commands::Analyze {
                project_folder: std::path::PathBuf::from("."),
            },
            cli.debug,
        ),
    }
}
