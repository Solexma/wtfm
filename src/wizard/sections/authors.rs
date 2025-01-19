use super::Section;
use crate::config::wtfm::{Author, SectionStatus, WtfmConfig};
use crate::utils::helpers::{DisplayHelper, ValidationHelper};
use crate::utils::messages::Messages;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::error::Error;

pub struct AuthorsSection {
    theme: ColorfulTheme,
}

impl AuthorsSection {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    fn prompt_fields(&mut self, config: &mut WtfmConfig) -> Result<bool, Box<dyn Error>> {
        DisplayHelper::show_tips(Messages::AUTHORS_TIPS);

        let is_edit = !config.authors.is_empty();
        if is_edit {
            let edit_options = &[
                "Edit authors",
                "Add author",
                "Remove author",
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
                    // Edit existing authors
                    if let Some(author_idx) = Select::with_theme(&self.theme)
                        .with_prompt("Select author to edit")
                        .items(
                            &config
                                .authors
                                .iter()
                                .map(|a| format!("{} <{}>", a.name, a.email))
                                .collect::<Vec<_>>(),
                        )
                        .interact_opt()?
                    {
                        let author = &config.authors[author_idx];
                        let name = Input::with_theme(&self.theme)
                            .with_prompt("Author name")
                            .with_initial_text(&author.name)
                            .interact_text()?;

                        let email = Input::with_theme(&self.theme)
                            .with_prompt("Author email")
                            .with_initial_text(&author.email)
                            .validate_with(|input: &String| ValidationHelper::validate_email(input))
                            .interact_text()?;

                        config.authors[author_idx] = Author { name, email };
                    }
                }
                1 => {
                    // Add new author
                    let name = Input::with_theme(&self.theme)
                        .with_prompt("Author name")
                        .interact_text()?;

                    let email = loop {
                        let input: String = Input::with_theme(&self.theme)
                            .with_prompt("Author email")
                            .interact_text()?;

                        match ValidationHelper::validate_email(&input) {
                            Ok(_) => break input,
                            Err(_) => {
                                DisplayHelper::show_error(Messages::ERR_INVALID_EMAIL);
                                continue;
                            }
                        }
                    };

                    config.authors.push(Author { name, email });
                }
                2 => {
                    // Remove author
                    if let Some(author_idx) = Select::with_theme(&self.theme)
                        .with_prompt("Select author to remove")
                        .items(
                            &config
                                .authors
                                .iter()
                                .map(|a| format!("{} <{}>", a.name, a.email))
                                .collect::<Vec<_>>(),
                        )
                        .interact_opt()?
                    {
                        config.authors.remove(author_idx);
                    }
                }
                3 => return Ok(true),
                _ => return Ok(false),
            }
        } else {
            // Add first author
            let name = Input::with_theme(&self.theme)
                .with_prompt("Author name")
                .interact_text()?;

            let email = loop {
                let input: String = Input::with_theme(&self.theme)
                    .with_prompt("Author email")
                    .interact_text()?;

                match ValidationHelper::validate_email(&input) {
                    Ok(_) => break input,
                    Err(_) => {
                        DisplayHelper::show_error(Messages::ERR_INVALID_EMAIL);
                        continue;
                    }
                }
            };

            config.authors.push(Author { name, email });
        }

        if !config.authors.is_empty() {
            config
                .sections_status
                .insert(self.name().to_string(), SectionStatus::Configured);
            DisplayHelper::show_success(Messages::SUCCESS_SECTION);
        }
        Ok(true)
    }
}

impl Section for AuthorsSection {
    fn name(&self) -> &'static str {
        "Authors"
    }

    fn description(&self) -> &'static str {
        "Add project authors"
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
