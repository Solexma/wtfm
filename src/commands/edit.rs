use crate::cli::args::Commands;
use crate::config::wtfm::WtfmConfig;
use crate::utils::helpers::DisplayHelper;
use crate::utils::template::generate_readme_with_template;
use crate::wizard::Wizard;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::error::Error;
use std::fs;

pub fn execute(cmd: &Commands, debug: bool) -> Result<(), Box<dyn Error>> {
    if let Commands::Edit {
        section,
        project_folder,
    } = cmd
    {
        let config_path = project_folder.join(".wtfm.json");
        let readme_path = project_folder.join("README.md");

        DisplayHelper::show_section_header("WTFM Editor");

        if !config_path.exists() {
            let should_generate = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("No configuration file found. Would you like to create one?")
                .default(true)
                .interact()?;

            if !should_generate {
                return Ok(());
            }

            return crate::commands::generate::execute(
                &Commands::Generate {
                    output: project_folder.clone(),
                    project_folder: project_folder.clone(),
                },
                debug,
            );
        }

        let config =
            WtfmConfig::load(&config_path, debug).ok_or("Failed to load configuration file")?;

        let mut wizard = Wizard::new_with_config(config);

        if let Some(section_name) = &section {
            DisplayHelper::show_section_header(&format!("Editing section: {}", section_name));
        }

        wizard.edit_section(section.clone())?;

        // Save updated configuration
        wizard.save_config(&config_path)?;
        DisplayHelper::show_success(&format!("Configuration saved at {}", config_path.display()));

        // Regenerate README.md
        let readme_content = generate_readme_with_template(wizard.get_config())?;
        fs::write(&readme_path, readme_content)?;
        DisplayHelper::show_success(&format!("README.md updated at {}", readme_path.display()));
    }

    Ok(())
}
