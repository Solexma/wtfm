mod authors;
mod cicd;
mod features;
mod install;
mod license;
mod project;

pub use authors::AuthorsSection;
pub use cicd::CiCdSection;
pub use features::FeaturesSection;
pub use install::InstallSection;
pub use license::LicenseSection;
pub use project::ProjectSection;

use crate::config::wtfm::{SectionStatus, WtfmConfig};
use std::error::Error;

pub trait Section {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>>;
    fn can_skip(&self) -> bool;
    fn status(&self, config: &WtfmConfig) -> SectionStatus;
    fn edit(&mut self, config: &mut WtfmConfig) -> Result<(), Box<dyn Error>>;
}
