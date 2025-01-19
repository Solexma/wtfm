pub mod analyze;
pub mod author;
pub mod edit;
pub mod generate;
pub mod info;

use std::path::PathBuf;

#[derive(Debug)]
pub enum Commands {
    Generate {
        output: PathBuf,
        project_folder: PathBuf,
    },
}
