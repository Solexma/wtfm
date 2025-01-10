use crate::cli::args::Cli;
use crate::config::wizard::WizardAnswers;
use crate::config::Author;
use crate::utils::git::Git;
use std::fs;

pub fn execute(cli: &Cli) {
    let config_path = ".wtfm.json";
    let config = crate::config::load_config(config_path);

    let git = Git::new(&cli.project_folder);

    let answers = if config.is_none() {
        let answers = WizardAnswers::from_interactive();
        let config = crate::config::WtfmConfig {
            project_name: answers.project_name.clone(),
            description: answers.description.clone(),
            version: answers.version.clone(),
            license: answers.license.clone(),
            setup_ci: answers.setup_ci,
            author_quantity: answers.author_quantity,
            authors: answers
                .authors
                .iter()
                .map(|author| {
                    let parts: Vec<&str> = author.split('<').collect();
                    let name = parts[0].trim().to_string();
                    let email = parts[1]
                        .trim_matches(|c| c == '<' || c == '>' || c == ' ')
                        .to_string();
                    Author { name, email }
                })
                .collect(),
            git_info: if git.is_repo() {
                Some(crate::config::GitInfo {
                    is_git_repo: true,
                    current_branch: git.info().and_then(|i| i.current_branch.clone()),
                    remote_url: git.info().and_then(|i| i.remote_url.clone()),
                    tags: git.info().map(|i| i.tags.clone()).unwrap_or_default(),
                })
            } else {
                None
            },
        };
        crate::config::save_config(config_path, &config).expect("Failed to save config");
        answers
    } else {
        let config = config.unwrap();
        WizardAnswers {
            project_name: config.project_name,
            description: config.description,
            version: config.version,
            license: config.license,
            setup_ci: config.setup_ci,
            author_quantity: config.author_quantity,
            authors: config
                .authors
                .iter()
                .map(|a| format!("{} <{}>", a.name, a.email))
                .collect(),
        }
    };

    if !git.is_repo() {
        println!("Note: Current directory is not a git repository");
    }

    fs::create_dir_all(&cli.output).expect("Failed to create output directory");
    let readme_path = cli.output.join("README.md");

    let readme_content = crate::utils::template::generate_readme_with_template(&answers);
    fs::write(readme_path.clone(), &readme_content).expect("Failed to write generated README");

    println!("README successfully generated at {:?}", readme_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_execute_generate() {
        // Create a temporary directory for our test
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create a CLI instance pointing to our temp directory
        let cli = Cli {
            command: None,
            output: temp_path.to_path_buf(),
            project_folder: temp_path.to_path_buf(),
        };

        // Execute the generate command
        execute(&cli);

        // Check if README.md was created
        let readme_path = temp_path.join("README.md");
        assert!(readme_path.exists());

        // Verify that README content is valid
        let content = fs::read_to_string(readme_path).unwrap();
        assert!(content.contains("# "));
        assert!(content.contains("## License"));
    }
}
