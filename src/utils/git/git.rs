use super::parser;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitInfo {
    pub is_git_repo: bool,
    pub current_branch: Option<String>,
    pub remote_url: Option<String>,
    pub tags: Vec<String>,
}

#[allow(dead_code)]
pub struct Git {
    path: std::path::PathBuf,
    git_path: std::path::PathBuf,
    info: Option<GitInfo>,
}

impl Git {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let git_path = path.join(".git");
        let info = if git_path.exists() {
            Some(parser::parse_git_info(&git_path))
        } else {
            None
        };

        Self {
            path,
            git_path,
            info,
        }
    }

    pub fn is_repo(&self) -> bool {
        self.info.is_some()
    }

    pub fn info(&self) -> Option<&GitInfo> {
        self.info.as_ref()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
