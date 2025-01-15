use crate::cli::args::Commands;
use crate::config::wizard::WizardAnswers;
use crate::config::wtfm::{Author, WtfmConfig};
use crate::utils::git::Git;
use std::fs;

pub fn execute(cmd: &Commands, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    if let Commands::Generate {
        output,
        project_folder,
    } = cmd
    {
        let config_path = project_folder.join(".wtfm.json");
        let config = WtfmConfig::load(&config_path, debug);

        let git = Git::new(project_folder);

        let answers = match config {
            None => {
                let answers = WizardAnswers::from_interactive();
                let config = WtfmConfig {
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
                    git_info: None,
                    cargo_info: None,
                };
                config.save(&config_path)?;
                answers
            }
            Some(config) => WizardAnswers {
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
            },
        };

        if !git.is_repo() {
            println!("Note: Current directory is not a git repository");
        }

        fs::create_dir_all(output)?;
        let readme_path = output.join("README.md");

        let readme_content = crate::utils::template::generate_readme_with_template(&answers);
        fs::write(&readme_path, &readme_content)?;

        println!("README successfully generated at {:?}", readme_path);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::licenses::{License, LicenseCategory};
    use tempfile::TempDir;

    #[test]
    fn test_generate_readme() {
        let temp_dir = TempDir::new().unwrap();
        let config = WtfmConfig {
            project_name: "Test Project".to_string(),
            description: "Test Description".to_string(),
            version: "0.1.0".to_string(),
            license: License::new(
                "MIT",
                "MIT License",
                LicenseCategory::Permissive,
                true,
                true,
                false,
            ),
            setup_ci: false,
            author_quantity: 1,
            authors: vec![Author {
                name: "Test Author".to_string(),
                email: "test@example.com".to_string(),
            }],
            git_info: None,
            cargo_info: None,
        };

        let config_path = temp_dir.path().join(".wtfm.json");
        config.save(&config_path).unwrap();

        let cmd = Commands::Generate {
            output: temp_dir.path().to_path_buf(),
            project_folder: temp_dir.path().to_path_buf(),
        };

        execute(&cmd, false).unwrap();

        assert!(temp_dir.path().join("README.md").exists());
    }

    #[test]
    fn test_generate_readme_with_ci() {
        let temp_dir = TempDir::new().unwrap();
        let cmd = Commands::Generate {
            output: temp_dir.path().to_path_buf(),
            project_folder: temp_dir.path().to_path_buf(),
        };

        let config = WtfmConfig {
            project_name: "Test Project".to_string(),
            description: "Test Description".to_string(),
            version: "0.1.0".to_string(),
            license: License::new(
                "MIT",
                "MIT License",
                LicenseCategory::Permissive,
                true,
                true,
                false,
            ),
            setup_ci: true,
            author_quantity: 1,
            authors: vec![Author {
                name: "Test Author".to_string(),
                email: "test@example.com".to_string(),
            }],
            git_info: None,
            cargo_info: None,
        };

        let config_path = temp_dir.path().join(".wtfm.json");
        config.save(&config_path).unwrap();

        execute(&cmd, false).unwrap();

        let readme_content = std::fs::read_to_string(temp_dir.path().join("README.md")).unwrap();
        assert!(readme_content.contains("Continuous Integration"));
    }
}
