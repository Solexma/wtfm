use super::Section;
use crate::config::wtfm::{SectionStatus, WtfmConfig};
use crate::utils::helpers::{DisplayHelper, ValidationHelper};
use crate::utils::messages::Messages;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::error::Error;

pub struct ProjectSection {
    theme: ColorfulTheme,
}

impl ProjectSection {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    fn prompt_fields(&mut self, config: &mut WtfmConfig) -> Result<bool, Box<dyn Error>> {
        DisplayHelper::show_tips(Messages::PROJECT_TIPS);

        let is_edit = !config.project_name.is_empty();
        if is_edit {
            let edit_options = &["Edit project details", "Skip", "Back"];
            let selection = Select::with_theme(&self.theme)
                .with_prompt("What would you like to do?")
                .items(edit_options)
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    // Continue with editing
                }
                1 => return Ok(true),
                _ => return Ok(false),
            }
        }

        let name = Input::with_theme(&self.theme)
            .with_prompt("Project name")
            .with_initial_text(&config.project_name)
            .validate_with(|input: &String| ValidationHelper::validate_project_name(input))
            .interact_text()?;

        let description = Input::with_theme(&self.theme)
            .with_prompt("Project description")
            .with_initial_text(&config.description)
            .allow_empty(true)
            .interact_text()?;

        let version: String = Input::with_theme(&self.theme)
            .with_prompt("Version (X.Y.Z)")
            .with_initial_text(&config.version)
            .default("0.1.0".to_string())
            .validate_with(|input: &String| ValidationHelper::validate_version(input))
            .interact_text()?;

        let changed = name != config.project_name
            || config.description != description
            || version != config.version;

        config.project_name = name;
        config.description = description;
        config.version = version;

        config
            .sections_status
            .insert(self.name().to_string(), SectionStatus::Configured);

        if changed {
            DisplayHelper::show_success("Project details updated successfully");
        }
        DisplayHelper::show_success(Messages::SUCCESS_SECTION);
        Ok(true)
    }
}

impl Section for ProjectSection {
    fn name(&self) -> &'static str {
        "Project"
    }

    fn description(&self) -> &'static str {
        "Configure basic project information (name, description, version)"
    }

    fn run(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>> {
        self.prompt_fields(config).map(|_| ())
    }

    fn can_skip(&self) -> bool {
        false
    }

    fn status(&self, config: &WtfmConfig) -> SectionStatus {
        config
            .sections_status
            .get(self.name())
            .cloned()
            .unwrap_or(SectionStatus::NotConfigured)
    }

    fn edit(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>> {
        self.prompt_fields(config).map(|_| ())
    }
}
