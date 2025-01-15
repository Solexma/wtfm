use crate::licenses::{License, LicenseCategory};
use dialoguer::{Input, Select};
use std::fmt;

#[derive(Debug)]
pub struct WizardAnswers {
    pub project_name: String,
    pub description: String,
    pub version: String,
    pub license: License,
    pub setup_ci: bool,
    pub author_quantity: u32,
    pub authors: Vec<String>,
}

impl fmt::Display for WizardAnswers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Project: {}\nDescription: {}\nVersion: {}\nLicense: {} ({})\nSetup CI: {}\nAuthors: {}",
            self.project_name,
            self.description,
            self.version,
            self.license.name,
            self.license.spdx_id,
            self.setup_ci,
            self.authors.join(", ")
        )
    }
}

impl WizardAnswers {
    #[doc(hidden)]
    pub fn new_test() -> Self {
        Self {
            project_name: "Test Project".to_string(),
            description: "A test project".to_string(),
            version: "0.1.0".to_string(),
            license: License::new(
                "MIT",
                "MIT License",
                LicenseCategory::Permissive,
                true,
                true,
                false,
            ),
            setup_ci: true,
            author_quantity: 2,
            authors: vec!["Author 1".to_string(), "Author 2".to_string()],
        }
    }

    pub fn from_interactive() -> Self {
        let project_name: String = Input::new()
            .with_prompt("Project name")
            .interact()
            .expect("Failed to get project name");

        let description: String = Input::new()
            .with_prompt("Description")
            .interact()
            .expect("Failed to get description");

        let version: String = Input::new()
            .with_prompt("Version")
            .default("0.1.0".into())
            .interact()
            .expect("Failed to get version");

        let available_licenses = License::get_licenses();
        let license_idx = Select::new()
            .with_prompt("Choose a license")
            .items(&available_licenses)
            .default(0)
            .interact()
            .expect("Failed to get license choice");

        let license = available_licenses[license_idx].clone();

        let setup_ci = Select::new()
            .with_prompt("Setup CI?")
            .items(&["Yes", "No"])
            .default(0)
            .interact()
            .expect("Failed to get CI choice")
            == 0;

        let author_quantity: u32 = Input::new()
            .with_prompt("How many authors?")
            .default(1)
            .interact()
            .expect("Failed to get authors quantity");

        let authors = (0..author_quantity)
            .map(|_| {
                Input::new()
                    .with_prompt("Author name")
                    .interact()
                    .expect("Failed to get author name")
            })
            .collect();

        Self {
            project_name,
            description,
            version,
            license,
            setup_ci,
            author_quantity,
            authors,
        }
    }
}
