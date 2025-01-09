use dialoguer::{Input, Select};

pub struct WizardAnswers {
    pub project_name: String,
    pub description: String,
    pub version: String,
    pub license: String,
    pub setup_ci: bool,
    pub author_quantity: u32,
    pub authors: Vec<String>,
}

impl WizardAnswers {
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

        let licenses = vec!["MIT", "Apache-2.0", "GPL-3.0", "AGPL-3.0"];
        let license_idx = Select::new()
            .with_prompt("Choose a license")
            .items(&licenses)
            .default(0)
            .interact()
            .expect("Failed to get license choice");

        let setup_ci = Select::new()
            .with_prompt("Setup CI?")
            .items(&["Yes", "No"])
            .default(0)
            .interact()
            .expect("Failed to get CI choice") == 0;

        let author_quantity: u32 = Input::new()
            .with_prompt("How many authors?")
            .default(1)
            .interact()
            .expect("Failed to get authors quantity");

        let authors = (0..author_quantity)
            .map(|_| Input::new().with_prompt("Author name").interact().expect("Failed to get author name"))
            .collect();

        Self {
            project_name,
            description,
            version,
            license: licenses[license_idx].to_string(),
            setup_ci,
            author_quantity,
            authors,
        }
    }
}