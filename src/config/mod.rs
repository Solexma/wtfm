pub mod wizard;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct WtfmConfig {
    pub project_name: String,
    pub description: String,
    pub version: String,
    pub license: String,
    pub setup_ci: bool,
    pub author_quantity: u32,
    pub authors: Vec<Author>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Option<WtfmConfig> {
    let config_content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&config_content).ok()
}

pub fn save_config<P: AsRef<Path>>(path: P, config: &WtfmConfig) -> std::io::Result<()> {
    let config_content = serde_json::to_string_pretty(config)?;
    fs::write(path, config_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_save_and_load_config() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = WtfmConfig {
            project_name: "Test Project".to_string(),
            description: "A test project".to_string(),
            version: "0.1.0".to_string(),
            license: "MIT".to_string(),
            setup_ci: true,
            author_quantity: 1,
            authors: vec![Author {
                name: "Test Author".to_string(),
                email: "test@example.com".to_string(),
            }],
        };

        // Test saving configuration
        save_config(&temp_file, &config).unwrap();
        assert!(temp_file.as_file().metadata().unwrap().len() > 0);

        // Test loading configuration
        let loaded_config = load_config(&temp_file).unwrap();
        assert_eq!(loaded_config.project_name, config.project_name);
        assert_eq!(loaded_config.description, config.description);
        assert_eq!(loaded_config.version, config.version);
        assert_eq!(loaded_config.license, config.license);
        assert_eq!(loaded_config.setup_ci, config.setup_ci);
        assert_eq!(loaded_config.author_quantity, config.author_quantity);
        assert_eq!(loaded_config.authors.len(), config.authors.len());
        assert_eq!(loaded_config.authors[0].name, config.authors[0].name);
        assert_eq!(loaded_config.authors[0].email, config.authors[0].email);
    }

    #[test]
    fn test_load_config_nonexistent_file() {
        let result = load_config("nonexistent_file.json");
        assert!(result.is_none());
    }
}