use crate::cli::args::Cli;
use crate::utils::git::Git;
use std::fs;

pub fn execute(cli: &Cli) {
    // look into current folder for :
    // - .wtfm.json -> it's a wtfm project
    // - .git folder -> it's a git repo
    // - package.json -> it's a node project
    // - Cargo.toml -> it's a rust project

    // if .wtfm.json exists we read the json and check if the git area in populated, if the project is a git repo

    let mut is_wtfm_project = false;
    let mut is_git_repo = false;
    let mut is_node_project = false;
    let mut is_rust_project = false;

    let config_path = &cli.project_folder.join(".wtfm.json");
    let config = crate::config::load_config(config_path);

    if config.is_some() {
        is_wtfm_project = true;
    }

    let git = Git::new(&cli.project_folder);
    if git.is_repo() {
        is_git_repo = true;
    }

    if fs::metadata("package.json").is_ok() {
        is_node_project = true;
    }

    if fs::metadata("Cargo.toml").is_ok() {
        is_rust_project = true;
    }

    println!("is_wtfm_project: {}", is_wtfm_project);

    println!("is_git_repo: {}", is_git_repo);
    if is_git_repo {
        let git_info = git.info();
        println!("git_info: {:?}", git_info);
    }

    println!("is_node_project: {}", is_node_project);

    println!("is_rust_project: {}", is_rust_project);
}
