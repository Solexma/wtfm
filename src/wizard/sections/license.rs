use super::Section;
use crate::config::wtfm::{License, SectionStatus, WtfmConfig};
use crate::utils::helpers::DisplayHelper;
use crate::utils::messages::Messages;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::error::Error;

pub struct LicenseSection {
    theme: ColorfulTheme,
}

impl LicenseSection {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    fn get_available_licenses() -> Vec<License> {
        vec![
            License {
                name: "MIT".to_string(),
                spdx_id: "MIT".to_string(),
                url: "https://opensource.org/licenses/MIT".to_string(),
            },
            License {
                name: "Apache 2.0".to_string(),
                spdx_id: "Apache-2.0".to_string(),
                url: "https://opensource.org/licenses/Apache-2.0".to_string(),
            },
            License {
                name: "GNU GPL v3".to_string(),
                spdx_id: "GPL-3.0".to_string(),
                url: "https://www.gnu.org/licenses/gpl-3.0.en.html".to_string(),
            },
            License {
                name: "BSD 3-Clause".to_string(),
                spdx_id: "BSD-3-Clause".to_string(),
                url: "https://opensource.org/licenses/BSD-3-Clause".to_string(),
            },
            License {
                name: "Custom".to_string(),
                spdx_id: "Custom".to_string(),
                url: String::new(),
            },
        ]
    }

    fn prompt_fields(&mut self, config: &mut WtfmConfig) -> Result<bool, Box<dyn Error>> {
        DisplayHelper::show_tips(Messages::LICENSE_TIPS);

        let is_edit = config.license.is_some();
        if is_edit {
            let edit_options = &["Change license", "Remove license", "Skip", "Back"];
            let selection = Select::with_theme(&self.theme)
                .with_prompt("What would you like to do?")
                .items(edit_options)
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    // Continue with license selection
                }
                1 => {
                    config.license = None;
                    config
                        .sections_status
                        .insert(self.name().to_string(), SectionStatus::Skipped);
                    DisplayHelper::show_success("License removed successfully");
                    return Ok(true);
                }
                2 => return Ok(true),
                _ => return Ok(false),
            }
        }

        let licenses = Self::get_available_licenses();
        let license_names: Vec<&str> = licenses.iter().map(|l| l.name.as_str()).collect();

        let selected = Select::with_theme(&self.theme)
            .with_prompt("Choose a license")
            .items(&license_names)
            .default(0)
            .interact()?;

        let mut selected_license = licenses[selected].clone();

        if selected_license.name == "Custom" {
            selected_license.name = Input::with_theme(&self.theme)
                .with_prompt("License name")
                .interact_text()?;

            selected_license.spdx_id = Input::with_theme(&self.theme)
                .with_prompt("SPDX identifier")
                .interact_text()?;

            selected_license.url = Input::with_theme(&self.theme)
                .with_prompt("License URL")
                .interact_text()?;
        }

        let changed = match &config.license {
            Some(current) => current != &selected_license,
            None => true,
        };

        config.license = Some(selected_license);
        config
            .sections_status
            .insert(self.name().to_string(), SectionStatus::Configured);

        if changed {
            DisplayHelper::show_success("License updated successfully");
        }
        DisplayHelper::show_success(Messages::SUCCESS_SECTION);
        Ok(true)
    }

    fn edit_license(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>> {
        if config.sections_status.get(self.name()) == Some(&SectionStatus::NotConfigured) {
            return self.prompt_fields(config).map(|_| ());
        }

        DisplayHelper::show_section_header("Current License");
        println!(
            "Name: {}",
            config
                .license
                .as_ref()
                .map(|l| l.name.clone())
                .unwrap_or_default()
        );
        println!(
            "SPDX ID: {}",
            config
                .license
                .as_ref()
                .map(|l| l.spdx_id.clone())
                .unwrap_or_default()
        );
        if let Some(license) = &config.license {
            if !license.url.is_empty() {
                println!("URL: {}", license.url);
            }
        }

        let edit_options = &["Change license", "Remove license", "Back"];
        let selection = Select::with_theme(&self.theme)
            .with_prompt("What would you like to do?")
            .items(edit_options)
            .default(0)
            .interact()?;

        match selection {
            0 => self.prompt_fields(config).map(|_| ()),
            1 => {
                if Select::with_theme(&self.theme)
                    .with_prompt(Messages::CONFIRM_DELETE)
                    .items(&["Yes", "No"])
                    .default(1)
                    .interact()?
                    == 0
                {
                    config
                        .sections_status
                        .insert(self.name().to_string(), SectionStatus::Skipped);
                    DisplayHelper::show_success("License removed successfully");
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl Section for LicenseSection {
    fn name(&self) -> &'static str {
        "License"
    }

    fn description(&self) -> &'static str {
        "Choose a license for your project"
    }

    fn run(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>> {
        self.prompt_fields(config).map(|_| ())
    }

    fn can_skip(&self) -> bool {
        true
    }

    fn status(&self, config: &WtfmConfig) -> SectionStatus {
        config
            .sections_status
            .get(self.name())
            .cloned()
            .unwrap_or(SectionStatus::NotConfigured)
    }

    fn edit(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>> {
        self.edit_license(config)
    }
}
