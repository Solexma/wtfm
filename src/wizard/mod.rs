pub mod menu;
pub mod sections;

use crate::config::wtfm::WtfmConfig;
use menu::Menu;
use sections::{
    AuthorsSection, CiCdSection, FeaturesSection, InstallSection, LicenseSection, ProjectSection,
};
use std::error::Error;
use std::path::Path;

pub struct Wizard {
    menu: Menu,
    pub config: WtfmConfig,
}

impl Wizard {
    pub fn new() -> Self {
        Self::new_with_config(WtfmConfig::default())
    }

    pub fn new_with_config(config: WtfmConfig) -> Self {
        let mut menu = Menu::new();
        menu.add_section(Box::new(ProjectSection::new()));
        menu.add_section(Box::new(LicenseSection::new()));
        menu.add_section(Box::new(AuthorsSection::new()));
        menu.add_section(Box::new(CiCdSection::new()));
        menu.add_section(Box::new(FeaturesSection::new()));
        menu.add_section(Box::new(InstallSection::new()));

        Self { menu, config }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Welcome to WTFM Interactive Wizard!");
        println!("Let's configure your documentation step by step.");
        println!();

        self.menu.run(&mut self.config)
    }

    pub fn save_config(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        self.config.save(path)?;
        Ok(())
    }

    pub fn get_config(&self) -> &WtfmConfig {
        &self.config
    }

    pub fn edit_section(&mut self, section_name: Option<String>) -> Result<(), Box<dyn Error>> {
        match section_name {
            Some(name) => {
                if let Some(section) = self.menu.find_section(&name) {
                    section.edit(&mut self.config)?;
                }
                Ok(())
            }
            None => self.run(),
        }
    }
}

pub use sections::Section;
