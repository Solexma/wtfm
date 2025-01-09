use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Output directory for the generated README
    #[arg(short, long, default_value = ".")]
    pub output: PathBuf,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Display author information
    Author,
    /// Generate README file
    Generate,
}
