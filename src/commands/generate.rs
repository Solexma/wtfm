use crate::cli::args::Cli;
use crate::config::wizard::WizardAnswers;
use crate::config::Author;
use std::fs;

pub fn execute(cli: &Cli) {
    let config_path = ".wtfm.json";
    let config = crate::config::load_config(config_path);

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

    fs::create_dir_all(&cli.output).expect("Failed to create output directory");
    let readme_path = cli.output.join("README.md");

    let readme_content = crate::utils::template::generate_readme_with_template(&answers);
    fs::write(readme_path.clone(), &readme_content).expect("Failed to write generated README");

    println!("README successfully generated at {:?}", readme_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::args::Cli;
    use tempfile::TempDir;

    #[test]
    fn test_execute_generate() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().to_path_buf();

        // Create a mock CLI instance
        let cli = Cli {
            command: None,
            output: output_path.clone(),
        };

        // Execute the generate command
        execute(&cli);

        // Verify that README was created
        let readme_path = output_path.join("README.md");
        assert!(readme_path.exists());

        // Verify that README content is valid
        let content = fs::read_to_string(readme_path).unwrap();
        assert!(content.contains("# "));
        assert!(content.contains("## License"));
    }
}
