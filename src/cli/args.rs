use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Analyze the project
    Analyze {
        /// Project directory to analyze
        #[arg(short, long, default_value = ".")]
        project_folder: PathBuf,
    },
    /// Display info about the project
    Info,
    /// Display author information
    Author,
    /// Generate README file
    Generate {
        /// Output directory for the generated README
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
        /// Project directory
        #[arg(short, long, default_value = ".")]
        project_folder: PathBuf,
    },
}
