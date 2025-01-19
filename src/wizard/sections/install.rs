use super::Section;
use crate::config::wtfm::{SectionStatus, WtfmConfig};
use crate::utils::helpers::DisplayHelper;
use crate::utils::messages::Messages;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, MultiSelect, Select};
use std::error::Error;

pub struct InstallSection {
    theme: ColorfulTheme,
}

impl InstallSection {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    fn prompt_prerequisites(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut prerequisites = Vec::new();
        println!("\nAdd prerequisites (empty line to finish):");
        loop {
            let prerequisite: String = Input::with_theme(&self.theme)
                .with_prompt("Prerequisite")
                .allow_empty(true)
                .interact_text()?;

            if prerequisite.trim().is_empty() {
                break;
            }

            prerequisites.push(prerequisite);
        }
        Ok(prerequisites)
    }

    fn prompt_install_steps(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut steps = Vec::new();
        println!("\nAdd installation steps (empty line to finish):");
        loop {
            let step: String = Input::with_theme(&self.theme)
                .with_prompt("Step")
                .allow_empty(true)
                .interact_text()?;

            if step.trim().is_empty() {
                break;
            }

            steps.push(step);
        }
        Ok(steps)
    }

    fn prompt_fields(&mut self, config: &mut WtfmConfig) -> Result<bool, Box<dyn Error>> {
        DisplayHelper::show_tips(Messages::INSTALL_TIPS);

        let is_edit = !config.prerequisites.is_empty() || !config.install_steps.is_empty();
        if is_edit {
            let edit_options = &[
                "Edit prerequisites",
                "Edit installation steps",
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
                    // Edit prerequisites
                    let prereq_options =
                        &["Add new", "Remove existing", "Edit existing", "Replace all"];
                    let action = Select::with_theme(&self.theme)
                        .with_prompt("Choose action")
                        .items(prereq_options)
                        .default(0)
                        .interact()?;

                    match action {
                        0 => {
                            let mut new_prereqs = self.prompt_prerequisites()?;
                            config.prerequisites.append(&mut new_prereqs);
                        }
                        1 => {
                            if !config.prerequisites.is_empty() {
                                let selections = MultiSelect::with_theme(&self.theme)
                                    .with_prompt("Select prerequisites to remove")
                                    .items(&config.prerequisites)
                                    .interact()?;

                                for idx in selections.into_iter().rev() {
                                    config.prerequisites.remove(idx);
                                }
                                DisplayHelper::show_success("Prerequisites removed successfully");
                            }
                        }
                        2 => {
                            if let Some(prereq_idx) = Select::with_theme(&self.theme)
                                .with_prompt("Select prerequisite to edit")
                                .items(&config.prerequisites)
                                .interact_opt()?
                            {
                                let new_prereq: String = Input::with_theme(&self.theme)
                                    .with_prompt("Prerequisite")
                                    .with_initial_text(&config.prerequisites[prereq_idx])
                                    .interact_text()?;

                                if !new_prereq.trim().is_empty() {
                                    config.prerequisites[prereq_idx] = new_prereq;
                                    DisplayHelper::show_success(
                                        "Prerequisite updated successfully",
                                    );
                                }
                            }
                        }
                        3 => {
                            config.prerequisites = self.prompt_prerequisites()?;
                        }
                        _ => {}
                    }
                }
                1 => {
                    // Edit installation steps
                    let step_options = &[
                        "Add new",
                        "Remove existing",
                        "Edit existing",
                        "Reorder steps",
                        "Replace all",
                    ];
                    let action = Select::with_theme(&self.theme)
                        .with_prompt("Choose action")
                        .items(step_options)
                        .default(0)
                        .interact()?;

                    match action {
                        0 => {
                            let mut new_steps = self.prompt_install_steps()?;
                            config.install_steps.append(&mut new_steps);
                        }
                        1 => {
                            if !config.install_steps.is_empty() {
                                let selections = MultiSelect::with_theme(&self.theme)
                                    .with_prompt("Select steps to remove")
                                    .items(&config.install_steps)
                                    .interact()?;

                                for idx in selections.into_iter().rev() {
                                    config.install_steps.remove(idx);
                                }
                            }
                        }
                        2 => {
                            if let Some(step_idx) = Select::with_theme(&self.theme)
                                .with_prompt("Select step to edit")
                                .items(&config.install_steps)
                                .interact_opt()?
                            {
                                let new_step: String = Input::with_theme(&self.theme)
                                    .with_prompt("Step")
                                    .with_initial_text(&config.install_steps[step_idx])
                                    .interact_text()?;

                                if !new_step.trim().is_empty() {
                                    config.install_steps[step_idx] = new_step;
                                    DisplayHelper::show_success(
                                        "Installation step updated successfully",
                                    );
                                }
                            }
                        }
                        3 => {
                            if config.install_steps.len() > 1 {
                                let mut remaining_steps = config.install_steps.clone();
                                let mut new_order = Vec::new();

                                while !remaining_steps.is_empty() {
                                    if let Some(idx) = Select::with_theme(&self.theme)
                                        .with_prompt("Select next step")
                                        .items(&remaining_steps)
                                        .interact_opt()?
                                    {
                                        new_order.push(remaining_steps.remove(idx));
                                    } else {
                                        break;
                                    }
                                }

                                if !new_order.is_empty() {
                                    config.install_steps = new_order;
                                }
                            }
                        }
                        4 => {
                            config.install_steps = self.prompt_install_steps()?;
                        }
                        _ => {}
                    }
                }
                2 => return Ok(true),
                _ => return Ok(false),
            }
        } else {
            // Add new prerequisites and installation steps
            config.prerequisites = self.prompt_prerequisites()?;
            config.install_steps = self.prompt_install_steps()?;
        }

        if !config.prerequisites.is_empty() || !config.install_steps.is_empty() {
            config
                .sections_status
                .insert(self.name().to_string(), SectionStatus::Configured);
            DisplayHelper::show_success(Messages::SUCCESS_SECTION);
        }
        Ok(true)
    }

    fn edit_section(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>> {
        if config.prerequisites.is_empty() && config.install_steps.is_empty() {
            return self.prompt_fields(config).map(|_| ());
        }

        let edit_options = &["Edit prerequisites", "Edit installation steps", "Done"];
        loop {
            DisplayHelper::show_section_header("Installation Configuration");

            // Display current configuration
            if !config.prerequisites.is_empty() {
                println!("\nCurrent prerequisites:");
                for (i, prereq) in config.prerequisites.iter().enumerate() {
                    println!("{}. {}", i + 1, prereq);
                }
            }

            if !config.install_steps.is_empty() {
                println!("\nCurrent installation steps:");
                for (i, step) in config.install_steps.iter().enumerate() {
                    println!("{}. {}", i + 1, step);
                }
            }

            let selection = Select::with_theme(&self.theme)
                .with_prompt("\nWhat would you like to edit?")
                .items(edit_options)
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    // Edit prerequisites
                    let prereq_options = &[
                        "Add new",
                        "Remove existing",
                        "Edit existing",
                        "Replace all",
                        "Back",
                    ];
                    let action = Select::with_theme(&self.theme)
                        .with_prompt("Choose action")
                        .items(prereq_options)
                        .default(0)
                        .interact()?;

                    match action {
                        0 => {
                            let new_prereqs = self.prompt_prerequisites()?;
                            config.prerequisites.extend(new_prereqs);
                        }
                        1 => {
                            if !config.prerequisites.is_empty() {
                                let selections = MultiSelect::with_theme(&self.theme)
                                    .with_prompt("Select prerequisites to remove")
                                    .items(&config.prerequisites)
                                    .interact()?;

                                if !selections.is_empty()
                                    && Select::with_theme(&self.theme)
                                        .with_prompt(Messages::CONFIRM_DELETE)
                                        .items(&["Yes", "No"])
                                        .default(1)
                                        .interact()?
                                        == 0
                                {
                                    for index in selections.into_iter().rev() {
                                        config.prerequisites.remove(index);
                                    }
                                    DisplayHelper::show_success(
                                        "Prerequisites removed successfully",
                                    );
                                }
                            } else {
                                DisplayHelper::show_warning("No prerequisites to remove");
                            }
                        }
                        2 => {
                            if !config.prerequisites.is_empty() {
                                let prereq_idx = Select::with_theme(&self.theme)
                                    .with_prompt("Select prerequisite to edit")
                                    .items(&config.prerequisites)
                                    .interact()?;

                                let new_prereq: String = Input::with_theme(&self.theme)
                                    .with_prompt("New prerequisite")
                                    .default(config.prerequisites[prereq_idx].clone())
                                    .interact_text()?;

                                if !new_prereq.trim().is_empty() {
                                    config.prerequisites[prereq_idx] = new_prereq;
                                    DisplayHelper::show_success(
                                        "Prerequisite updated successfully",
                                    );
                                }
                            } else {
                                DisplayHelper::show_warning("No prerequisites to edit");
                            }
                        }
                        3 => {
                            config.prerequisites = self.prompt_prerequisites()?;
                        }
                        _ => {}
                    }
                }
                1 => {
                    // Edit installation steps
                    let step_options = &[
                        "Add new",
                        "Remove existing",
                        "Edit existing",
                        "Reorder steps",
                        "Replace all",
                        "Back",
                    ];
                    let action = Select::with_theme(&self.theme)
                        .with_prompt("Choose action")
                        .items(step_options)
                        .default(0)
                        .interact()?;

                    match action {
                        0 => {
                            let new_steps = self.prompt_install_steps()?;
                            config.install_steps.extend(new_steps);
                        }
                        1 => {
                            if !config.install_steps.is_empty() {
                                let selections = MultiSelect::with_theme(&self.theme)
                                    .with_prompt("Select steps to remove")
                                    .items(&config.install_steps)
                                    .interact()?;

                                if !selections.is_empty()
                                    && Select::with_theme(&self.theme)
                                        .with_prompt(Messages::CONFIRM_DELETE)
                                        .items(&["Yes", "No"])
                                        .default(1)
                                        .interact()?
                                        == 0
                                {
                                    for index in selections.into_iter().rev() {
                                        config.install_steps.remove(index);
                                    }
                                    DisplayHelper::show_success("Steps removed successfully");
                                }
                            } else {
                                DisplayHelper::show_warning("No steps to remove");
                            }
                        }
                        2 => {
                            if !config.install_steps.is_empty() {
                                let step_idx = Select::with_theme(&self.theme)
                                    .with_prompt("Select step to edit")
                                    .items(&config.install_steps)
                                    .interact()?;

                                let new_step: String = Input::with_theme(&self.theme)
                                    .with_prompt("Step")
                                    .with_initial_text(&config.install_steps[step_idx])
                                    .interact_text()?;

                                if !new_step.trim().is_empty() {
                                    config.install_steps[step_idx] = new_step;
                                    DisplayHelper::show_success(
                                        "Installation step updated successfully",
                                    );
                                }
                            } else {
                                DisplayHelper::show_warning("No steps to edit");
                            }
                        }
                        3 => {
                            if config.install_steps.len() > 1 {
                                println!("\nSelect steps in the desired order:");
                                let mut remaining_steps = config.install_steps.clone();
                                let mut new_order = Vec::new();

                                while !remaining_steps.is_empty() {
                                    let idx = Select::with_theme(&self.theme)
                                        .with_prompt("Select next step")
                                        .items(&remaining_steps)
                                        .interact()?;

                                    new_order.push(remaining_steps.remove(idx));
                                }

                                config.install_steps = new_order;
                                DisplayHelper::show_success("Steps reordered successfully");
                            } else {
                                DisplayHelper::show_warning("Not enough steps to reorder");
                            }
                        }
                        4 => {
                            config.install_steps = self.prompt_install_steps()?;
                        }
                        _ => {}
                    }
                }
                _ => break,
            }
        }

        if !config.install_steps.is_empty() || !config.prerequisites.is_empty() {
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

impl Section for InstallSection {
    fn name(&self) -> &'static str {
        "Installation"
    }

    fn description(&self) -> &'static str {
        "Add installation instructions and prerequisites"
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
        self.edit_section(config)
    }
}
