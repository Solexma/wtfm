use super::Section;
use crate::config::wtfm::{SectionStatus, WtfmConfig};
use crate::utils::helpers::DisplayHelper;
use crate::utils::messages::Messages;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, MultiSelect, Select};
use std::error::Error;

pub struct FeaturesSection {
    theme: ColorfulTheme,
}

impl FeaturesSection {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    fn prompt_fields(&mut self, config: &mut WtfmConfig) -> Result<bool, Box<dyn Error>> {
        DisplayHelper::show_tips(Messages::FEATURES_TIPS);

        let is_edit = !config.features.is_empty();
        if is_edit {
            let edit_options = &[
                "Edit features",
                "Add features",
                "Remove features",
                "Reorder features",
                "Skip",
                "Back",
            ];
            let selection = Select::with_theme(&self.theme)
                .with_prompt("What would you like to do?")
                .items(edit_options)
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    if let Some(feature_idx) = Select::with_theme(&self.theme)
                        .with_prompt("Select feature to edit")
                        .items(&config.features)
                        .interact_opt()?
                    {
                        let new_feature: String = Input::with_theme(&self.theme)
                            .with_prompt("Feature")
                            .with_initial_text(&config.features[feature_idx])
                            .interact_text()?;

                        if !new_feature.trim().is_empty() {
                            config.features[feature_idx] = new_feature;
                            DisplayHelper::show_success("Feature updated successfully");
                        }
                    }
                }
                1 => {
                    println!("\nAdd new features (empty line to finish):");
                    loop {
                        let feature: String = Input::with_theme(&self.theme)
                            .with_prompt("Feature")
                            .allow_empty(true)
                            .interact_text()?;

                        if feature.trim().is_empty() {
                            break;
                        }

                        config.features.push(feature);
                    }
                }
                2 => {
                    if !config.features.is_empty() {
                        let selections = MultiSelect::with_theme(&self.theme)
                            .with_prompt("Select features to remove")
                            .items(&config.features)
                            .interact()?;

                        for idx in selections.into_iter().rev() {
                            config.features.remove(idx);
                        }
                    } else {
                        DisplayHelper::show_warning("No features to remove");
                    }
                }
                3 => {
                    if config.features.len() > 1 {
                        println!("\nSelect features in the desired order:");
                        let mut remaining_features = config.features.clone();
                        let mut new_order = Vec::new();

                        while !remaining_features.is_empty() {
                            if let Some(idx) = Select::with_theme(&self.theme)
                                .with_prompt("Select next feature")
                                .items(&remaining_features)
                                .interact_opt()?
                            {
                                new_order.push(remaining_features.remove(idx));
                            } else {
                                break;
                            }
                        }

                        if !new_order.is_empty() {
                            config.features = new_order;
                        }
                    }
                }
                4 => return Ok(true),
                _ => return Ok(false),
            }
        } else {
            println!("\nAdd features (empty line to finish):");
            loop {
                let feature: String = Input::with_theme(&self.theme)
                    .with_prompt("Feature")
                    .allow_empty(true)
                    .interact_text()?;

                if feature.trim().is_empty() {
                    break;
                }

                config.features.push(feature);
            }
        }

        if !config.features.is_empty() {
            config
                .sections_status
                .insert(self.name().to_string(), SectionStatus::Configured);
            DisplayHelper::show_success(Messages::SUCCESS_SECTION);
        }
        Ok(true)
    }

    fn edit_features(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>> {
        if config.features.is_empty() {
            return self.prompt_fields(config).map(|_| ());
        }

        DisplayHelper::show_section_header("Current Features");
        for (i, feature) in config.features.iter().enumerate() {
            println!("{}. {}", i + 1, feature);
        }

        let edit_options = &[
            "Add features",
            "Remove features",
            "Edit features",
            "Reorder features",
            "Done",
        ];
        loop {
            let selection = Select::with_theme(&self.theme)
                .with_prompt("What would you like to do?")
                .items(edit_options)
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    DisplayHelper::show_tips(Messages::FEATURES_TIPS);
                    println!("\nAdd new features (empty line to finish):");
                    loop {
                        let feature: String = Input::with_theme(&self.theme)
                            .with_prompt("Feature")
                            .allow_empty(true)
                            .interact_text()?;

                        if feature.trim().is_empty() {
                            break;
                        }

                        config.features.push(feature);
                        DisplayHelper::show_success("Feature added");
                    }
                }
                1 => {
                    if config.features.is_empty() {
                        DisplayHelper::show_warning("No features to remove");
                        continue;
                    }

                    let selections = MultiSelect::with_theme(&self.theme)
                        .with_prompt("Select features to remove")
                        .items(&config.features)
                        .interact()?;

                    if !selections.is_empty() {
                        if Select::with_theme(&self.theme)
                            .with_prompt(Messages::CONFIRM_DELETE)
                            .items(&["Yes", "No"])
                            .default(1)
                            .interact()?
                            == 0
                        {
                            // Remove in reverse order to maintain correct indices
                            for index in selections.into_iter().rev() {
                                config.features.remove(index);
                            }
                            DisplayHelper::show_success("Features removed successfully");
                        }
                    }
                }
                2 => {
                    if config.features.is_empty() {
                        DisplayHelper::show_warning("No features to edit");
                        continue;
                    }

                    let feature_idx = Select::with_theme(&self.theme)
                        .with_prompt("Select feature to edit")
                        .items(&config.features)
                        .interact()?;

                    let new_feature = Input::with_theme(&self.theme)
                        .with_prompt("New feature text")
                        .default(config.features[feature_idx].clone())
                        .interact_text()?;

                    if !new_feature.trim().is_empty() {
                        config.features[feature_idx] = new_feature;
                        DisplayHelper::show_success("Feature updated successfully");
                    }
                }
                3 => {
                    if config.features.len() <= 1 {
                        DisplayHelper::show_warning("Not enough features to reorder");
                        continue;
                    }

                    DisplayHelper::show_section_header("Reorder Features");
                    println!("Select features in the desired order:");
                    let mut remaining_features = config.features.clone();
                    let mut new_order = Vec::new();

                    while !remaining_features.is_empty() {
                        let idx = Select::with_theme(&self.theme)
                            .with_prompt("Select next feature")
                            .items(&remaining_features)
                            .interact()?;

                        new_order.push(remaining_features.remove(idx));
                    }

                    config.features = new_order;
                    DisplayHelper::show_success("Features reordered successfully");
                }
                _ => break,
            }
        }

        if !config.features.is_empty() {
            config
                .sections_status
                .insert(self.name().to_string(), SectionStatus::Configured);
        } else {
            config
                .sections_status
                .insert(self.name().to_string(), SectionStatus::Skipped);
        }

        Ok(())
    }
}

impl Section for FeaturesSection {
    fn name(&self) -> &'static str {
        "Features"
    }

    fn description(&self) -> &'static str {
        "Add features and capabilities of your project"
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
        self.edit_features(config)
    }
}
