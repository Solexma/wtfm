use crate::cli::args::Commands;
use crate::config::wtfm::WtfmConfig;
use crate::debug;
use crate::utils::cargo::cargo::Cargo;
use crate::utils::git::Git;
use colored::*;
use std::fs;

// Analyze the current project and print the results
// TODO:
// - Dig deeper into .git area for fetching more informations, maybe via API, detacting if it's GitHub/Gitlab/Bitbucket/...
// - Add a way to edit the .wtfm.json file
pub fn execute(cmd: &Commands, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    if let Commands::Analyze { project_folder } = cmd {
        // look into current folder for :
        // - .wtfm.json -> it's a wtfm project (if there's a file, user can edit it)
        // - .git folder -> it's a git repo
        // - package.json -> it's a node project (we can get project information and scripts from here)
        // - Cargo.toml -> it's a rust project (we can get package information from here)
        // - LICENSE -> it's a license file (we can get license information from here ?)

        let mut is_wtfm_project = false;
        let mut is_git_repo = false;
        let mut is_node_project = false;
        let mut is_rust_project = false;
        let mut is_license_file = false;

        // Usa project_folder per tutti i percorsi
        let config_path = project_folder.join(".wtfm.json");
        let package_json_path = project_folder.join("package.json");
        let license_path = project_folder.join("LICENSE");

        debug!("Looking for config at: {:?}", config_path);
        debug!("Config file exists: {}", config_path.exists());

        let config = WtfmConfig::load(&config_path, debug);
        debug!("Config loaded: {}", config.is_some());

        if config.is_some() {
            is_wtfm_project = true;
            debug!("Found .wtfm.json configuration file");
        }

        let git = Git::new(project_folder);
        if git.is_repo() {
            is_git_repo = true;
            debug!("Found .git folder");
        }

        if fs::metadata(&package_json_path).is_ok() {
            is_node_project = true;
            debug!("Found package.json file");
        }

        let cargo = Cargo::new(project_folder);
        if cargo.info().is_some() {
            is_rust_project = true;
            debug!("Found Cargo.toml file");
        }

        if fs::metadata(&license_path).is_ok() {
            is_license_file = true;
            debug!("Found LICENSE file");
        }

        println!("{}: {}", "Project Status".bold(), "-".repeat(40));
        println!(
            "WTFM Project: {}",
            if is_wtfm_project {
                "Yes".green()
            } else {
                "No".red()
            }
        );
        println!(
            "Git Repository: {}",
            if is_git_repo {
                "Yes".green()
            } else {
                "No".red()
            }
        );
        println!(
            "Rust Project: {}",
            if is_rust_project {
                "Yes".green()
            } else {
                "No".red()
            }
        );
        println!(
            "Node.js Project: {}",
            if is_node_project {
                "Yes".green()
            } else {
                "No".red()
            }
        );
        println!(
            "License File: {}",
            if is_license_file {
                "Yes".green()
            } else {
                "No".red()
            }
        );
    }
    Ok(())
}
