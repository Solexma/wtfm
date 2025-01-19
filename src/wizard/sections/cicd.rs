use super::Section;
use crate::config::wtfm::{SectionStatus, WtfmConfig};
use crate::utils::helpers::DisplayHelper;
use crate::utils::messages::Messages;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, MultiSelect, Select};
use std::error::Error;

pub struct CiCdSection {
    theme: ColorfulTheme,
}

impl CiCdSection {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    fn get_available_features() -> Vec<&'static str> {
        vec![
            "Build",
            "Test",
            "Lint",
            "Coverage",
            "Documentation",
            "Release",
            "Docker",
            "Deploy",
        ]
    }

    fn prompt_fields(&mut self, config: &mut WtfmConfig) -> Result<bool, Box<dyn Error>> {
        DisplayHelper::show_tips(Messages::CICD_TIPS);

        let is_edit = config.setup_ci;
        if is_edit {
            let edit_options = &["Edit CI/CD settings", "Disable CI/CD", "Skip", "Back"];
            let selection = Select::with_theme(&self.theme)
                .with_prompt("What would you like to do?")
                .items(edit_options)
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    let ci_options = &[
                        "Edit platform",
                        "Edit features",
                        "Edit branch configuration",
                    ];
                    let action = Select::with_theme(&self.theme)
                        .with_prompt("What would you like to edit?")
                        .items(ci_options)
                        .default(0)
                        .interact()?;

                    match action {
                        0 => {
                            config.ci_platform = Some(
                                Input::with_theme(&self.theme)
                                    .with_prompt("CI/CD platform")
                                    .with_initial_text(config.ci_platform.as_deref().unwrap_or(""))
                                    .interact_text()?,
                            );
                        }
                        1 => {
                            let features = Self::get_available_features();
                            let defaults: Vec<bool> = features
                                .iter()
                                .map(|&f| config.ci_features.contains(&f.to_string()))
                                .collect();

                            let selections = MultiSelect::with_theme(&self.theme)
                                .with_prompt("Select CI/CD features")
                                .items(&features)
                                .defaults(&defaults)
                                .interact()?;

                            config.ci_features = selections
                                .iter()
                                .map(|&i| features[i].to_string())
                                .collect();
                        }
                        2 => {
                            let mut branches = config.ci_branches.clone();
                            loop {
                                let branch_options = &["Add branch", "Remove branch", "Done"];
                                let branch_action = Select::with_theme(&self.theme)
                                    .with_prompt("Branch configuration")
                                    .items(branch_options)
                                    .default(0)
                                    .interact()?;

                                match branch_action {
                                    0 => {
                                        let branch = Input::with_theme(&self.theme)
                                            .with_prompt("Branch name")
                                            .interact_text()?;
                                        branches.push(branch);
                                    }
                                    1 => {
                                        if let Some(branch_idx) = Select::with_theme(&self.theme)
                                            .with_prompt("Select branch to remove")
                                            .items(&branches)
                                            .interact_opt()?
                                        {
                                            branches.remove(branch_idx);
                                        }
                                    }
                                    _ => break,
                                }
                            }
                            config.ci_branches = branches;
                        }
                        _ => {}
                    }
                }
                1 => {
                    config.setup_ci = false;
                    config.ci_platform = None;
                    config.ci_features.clear();
                    config.ci_branches.clear();
                    return Ok(true);
                }
                2 => return Ok(true),
                _ => return Ok(false),
            }
        } else {
            let setup = Select::with_theme(&self.theme)
                .with_prompt("Would you like to setup CI/CD?")
                .items(&["Yes", "No"])
                .default(0)
                .interact()?;

            if setup == 0 {
                config.setup_ci = true;
                config.ci_platform = Some(
                    Input::with_theme(&self.theme)
                        .with_prompt("CI/CD platform")
                        .interact_text()?,
                );

                let features = Self::get_available_features();
                let selections = MultiSelect::with_theme(&self.theme)
                    .with_prompt("Select CI/CD features")
                    .items(&features)
                    .interact()?;

                config.ci_features = selections
                    .iter()
                    .map(|&i| features[i].to_string())
                    .collect();

                println!("\nAdd branches (empty line to finish):");
                loop {
                    let branch: String = Input::with_theme(&self.theme)
                        .with_prompt("Branch")
                        .allow_empty(true)
                        .interact_text()?;

                    if branch.trim().is_empty() {
                        break;
                    }

                    config.ci_branches.push(branch);
                }
            }
        }

        config.sections_status.insert(
            self.name().to_string(),
            if config.setup_ci {
                SectionStatus::Configured
            } else {
                SectionStatus::Skipped
            },
        );

        DisplayHelper::show_success(Messages::SUCCESS_SECTION);
        Ok(true)
    }
}

impl Section for CiCdSection {
    fn name(&self) -> &'static str {
        "CI/CD"
    }

    fn description(&self) -> &'static str {
        "Configure Continuous Integration and Deployment settings"
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
        self.prompt_fields(config).map(|_| ())
    }
}
