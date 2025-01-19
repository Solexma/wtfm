use crate::cli::args::Commands;
use crate::config::wtfm::WtfmConfig;
use std::error::Error;

pub fn execute(cmd: &Commands, debug: bool) -> Result<(), Box<dyn Error>> {
    if let Commands::Info { project_folder } = cmd {
        let config_path = project_folder.join(".wtfm.json");

        if let Some(_config) = WtfmConfig::load(&config_path, debug) {
            println!("Project Info:");
            println!("Project: {}", env!("CARGO_PKG_NAME"));
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
            println!("Brought to you by Solexma LLC.");
            println!("Website: https://www.solexma.com");
            println!("GitHub: https://github.com/solexma");
            println!("--------------------------------");
        }
    }
    Ok(())
}
