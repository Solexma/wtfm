use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "wtfm")]
#[command(about = "Write The F*cking Manual")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate documentation
    Generate {
        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,

        /// Project folder
        #[arg(short, long, default_value = ".")]
        project_folder: PathBuf,
    },

    /// Analyze project
    Analyze {
        /// Project folder
        #[arg(short, long, default_value = ".")]
        project_folder: PathBuf,
    },

    /// Show project info
    Info {
        /// Project folder
        #[arg(short, long, default_value = ".")]
        project_folder: PathBuf,
    },

    /// Manage authors
    Author {
        /// Project folder
        #[arg(short, long, default_value = ".")]
        project_folder: PathBuf,
    },

    /// Edit configuration
    Edit {
        /// Project folder
        #[arg(short, long, default_value = ".")]
        project_folder: PathBuf,

        /// Section to edit (optional)
        #[arg(short, long)]
        section: Option<String>,
    },
}
