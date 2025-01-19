use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WtfmConfig {
    // Project Section
    pub project_name: String,
    pub description: String,
    pub version: String,

    // Authors Section
    pub authors: Vec<Author>,

    // License Section
    pub license: Option<License>,

    // CI/CD Section
    pub setup_ci: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci_platform: Option<String>,
    #[serde(default)]
    pub ci_features: Vec<String>,
    #[serde(default)]
    pub ci_branches: Vec<String>,

    // Features Section
    pub features: Vec<String>,

    // Installation Section
    pub prerequisites: Vec<String>,
    pub install_steps: Vec<String>,

    // Section Status
    pub sections_status: HashMap<String, SectionStatus>,

    // Optional Git Info (auto-detected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_info: Option<GitInfo>,

    // Optional Cargo Info (auto-detected for Rust projects)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_info: Option<CargoInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct License {
    pub name: String,
    pub spdx_id: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SectionStatus {
    NotConfigured,
    Configured,
    Skipped,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitInfo {
    pub repository: String,
    pub branch: String,
    pub remote_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CargoInfo {
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
}

impl WtfmConfig {
    pub fn load(path: &Path, debug: bool) -> Option<Self> {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(config) => Some(config),
                Err(e) => {
                    if debug {
                        eprintln!("Error parsing config file: {}", e);
                    }
                    None
                }
            },
            Err(e) => {
                if debug {
                    eprintln!("Error reading config file: {}", e);
                }
                None
            }
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_save_load() {
        let config = WtfmConfig {
            project_name: "Test".to_string(),
            description: "Test Project".to_string(),
            ..Default::default()
        };

        let temp_file = NamedTempFile::new().unwrap();
        config.save(temp_file.path()).unwrap();

        let loaded_config = WtfmConfig::load(temp_file.path(), false).unwrap();
        assert_eq!(loaded_config.project_name, config.project_name);
        assert_eq!(loaded_config.description, config.description);
    }
}
