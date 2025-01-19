use crate::config::wtfm::{SectionStatus, WtfmConfig};
use crate::utils::helpers::DisplayHelper;
use crate::wizard::sections::Section;
use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::HashMap;
use std::error::Error;

pub struct Menu {
    sections: Vec<Box<dyn Section>>,
    theme: ColorfulTheme,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            theme: ColorfulTheme::default(),
        }
    }

    pub fn add_section(&mut self, section: Box<dyn Section>) {
        self.sections.push(section);
    }

    pub fn run(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>> {
        loop {
            let mut options: Vec<String> = self
                .sections
                .iter()
                .map(|s| {
                    let status = match s.status(config) {
                        SectionStatus::NotConfigured => "[ ]",
                        SectionStatus::Configured => "[✓]",
                        SectionStatus::Skipped => "[↷]",
                    };
                    format!("{} {} - {}", status, s.name(), s.description())
                })
                .collect();

            options.push("Done".to_string());

            DisplayHelper::show_section_header("WTFM Configuration Menu");
            let selection = Select::with_theme(&self.theme)
                .with_prompt("Select a section to configure")
                .items(&options)
                .default(0)
                .interact()?;

            if selection == options.len() - 1 {
                let mut missing_required = false;
                for section in &self.sections {
                    if !section.can_skip() && section.status(config) == SectionStatus::NotConfigured
                    {
                        DisplayHelper::show_error(&format!(
                            "{} must be configured before proceeding.",
                            section.name()
                        ));
                        missing_required = true;
                    }
                }

                if missing_required {
                    if Select::with_theme(&self.theme)
                        .with_prompt(
                            "Required sections are not configured. Do you want to exit anyway?",
                        )
                        .items(&["No, continue configuring", "Yes, exit"])
                        .default(0)
                        .interact()?
                        == 1
                    {
                        break;
                    }
                    continue;
                } else {
                    if Select::with_theme(&self.theme)
                        .with_prompt("Are you sure you want to finish?")
                        .items(&["No, continue configuring", "Yes, I'm done"])
                        .default(0)
                        .interact()?
                        == 1
                    {
                        break;
                    }
                }
            } else {
                self.sections[selection].run(config)?;
            }
        }

        Ok(())
    }

    pub fn get_sections_status(&self, config: &WtfmConfig) -> HashMap<String, SectionStatus> {
        self.sections
            .iter()
            .map(|s| (s.name().to_string(), s.status(config)))
            .collect()
    }

    pub fn find_section(&mut self, name: &str) -> Option<&mut Box<dyn Section>> {
        self.sections
            .iter_mut()
            .find(|s| s.name().to_lowercase() == name.to_lowercase())
    }
}
