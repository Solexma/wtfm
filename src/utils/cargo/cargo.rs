use super::parser;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoInfo {
    pub package_name: Option<String>,
    pub version: Option<String>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
}

pub struct Cargo {
    path: std::path::PathBuf,
    info: Option<CargoInfo>,
}

impl Cargo {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let cargo_path = path.join("Cargo.toml");
        let info = if cargo_path.exists() {
            Some(parser::parse_cargo_info(&cargo_path))
        } else {
            None
        };

        Self { path, info }
    }

    pub fn info(&self) -> Option<&CargoInfo> {
        self.info.as_ref()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
