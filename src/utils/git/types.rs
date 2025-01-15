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

pub struct Git {
    path: std::path::PathBuf,
}

impl Git {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn is_repo(&self) -> bool {
        self.path.join(".git").is_dir()
    }

    pub fn info(&self) -> Option<GitInfo> {
        if !self.is_repo() {
            return None;
        }

        let git_path = self.path.join(".git");
        Some(GitInfo {
            is_git_repo: true,
            current_branch: parser::read_current_branch(&git_path),
            remote_url: parser::read_remote_url(&git_path),
            tags: parser::read_tags(&git_path),
        })
    }
}