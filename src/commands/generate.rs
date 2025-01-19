use crate::cli::args::Commands;
use crate::config::wtfm::WtfmConfig;
use crate::utils::helpers::DisplayHelper;
use crate::utils::template::generate_readme_with_template;
use crate::wizard::Wizard;
use std::error::Error;
use std::fs;

pub fn execute(cmd: &Commands, debug: bool) -> Result<(), Box<dyn Error>> {
    if let Commands::Generate {
        output,
        project_folder,
    } = cmd
    {
        let config_path = project_folder.join("wtfm.json");

        DisplayHelper::show_section_header("WTFM Generator");

        let config = if config_path.exists() {
            WtfmConfig::load(&config_path, debug).ok_or("Failed to load configuration")?
        } else {
            let mut wizard = Wizard::new();
            wizard.run()?;
            wizard.get_config().to_owned()
        };

        let readme_path = output.join("README.md");
        let readme_content = generate_readme_with_template(&config)?;
        fs::write(&readme_path, readme_content)?;

        DisplayHelper::show_success(&format!(
            "README.md generated successfully at {}",
            readme_path.display()
        ));

        if !config_path.exists() {
            config.save(&config_path)?;
            DisplayHelper::show_success(&format!(
                "Configuration saved at {}",
                config_path.display()
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_generate_readme() {
        let temp_dir = TempDir::new().unwrap();
        let config = WtfmConfig {
            project_name: "Test Project".to_string(),
            description: "Test Description".to_string(),
            ..Default::default()
        };

        let config_path = temp_dir.path().join("wtfm.json");
        config.save(&config_path).unwrap();

        let cmd = Commands::Generate {
            output: temp_dir.path().to_path_buf(),
            project_folder: temp_dir.path().to_path_buf(),
        };

        execute(&cmd, false).unwrap();
        assert!(temp_dir.path().join("README.md").exists());
    }
}
