use crate::cli::args::Commands;
use crate::config::wtfm::WtfmConfig;
use std::error::Error;

// Analyze the current project and print the results
// TODO:
// - Dig deeper into .git area for fetching more informations, maybe via API, detacting if it's GitHub/Gitlab/Bitbucket/...
// - Add a way to edit the .wtfm.json file
pub fn execute(cmd: &Commands, debug: bool) -> Result<(), Box<dyn Error>> {
    if let Commands::Analyze { project_folder } = cmd {
        let config_path = project_folder.join(".wtfm.json");

        if let Some(config) = WtfmConfig::load(&config_path, debug) {
            println!("Project Analysis:");
            println!("================");
            println!("Project: {}", config.project_name);
            println!("Description: {}", config.description);
            println!("Version: {}", config.version);
            println!("Authors: {}", config.authors.len());
            println!("Features: {}", config.features.len());
            println!("Installation steps: {}", config.install_steps.len());
            println!("CI/CD configured: {}", config.setup_ci);
        } else {
            println!("No configuration found or invalid configuration file.");
        }
    }
    Ok(())
}
