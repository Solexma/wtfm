use crate::debug;
use crate::licenses::License;
use crate::utils::cargo::cargo::CargoInfo;
use crate::utils::git::GitInfo;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct WtfmConfig {
    pub project_name: String,
    pub description: String,
    pub version: String,
    pub license: License,
    pub setup_ci: bool,
    pub author_quantity: u32,
    pub authors: Vec<Author>,
    pub git_info: Option<GitInfo>,
    pub cargo_info: Option<CargoInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl WtfmConfig {
    pub fn load<P: AsRef<Path>>(path: P, _debug: bool) -> Option<Self> {
        let config_content = match fs::read_to_string(path) {
            Ok(content) => {
                debug!("File content: {}", content);
                content
            }
            Err(e) => {
                debug!("Error reading config file: {}", e);
                return None;
            }
        };

        match serde_json::from_str(&config_content) {
            Ok(config) => Some(config),
            Err(e) => {
                debug!("Error parsing config file: {}", e);
                None
            }
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let config_content = serde_json::to_string_pretty(self)?;
        fs::write(path, config_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::licenses::{License, LicenseCategory};
    use tempfile::NamedTempFile;

    fn create_test_config() -> WtfmConfig {
        WtfmConfig {
            project_name: "Test".to_string(),
            description: "Test Project".to_string(),
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
            author_quantity: 1,
            authors: vec![Author {
                name: "Test Author".to_string(),
                email: "test@example.com".to_string(),
            }],
            git_info: None,
            cargo_info: None,
        }
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = create_test_config();

        config.save(temp_file.path()).unwrap();
        let loaded_config = WtfmConfig::load(temp_file.path(), false).unwrap();

        assert_eq!(loaded_config.project_name, config.project_name);
        assert_eq!(loaded_config.license.spdx_id, config.license.spdx_id);
    }
}
