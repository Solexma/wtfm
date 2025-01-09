mod parser;

use std::path::Path;

#[derive(Debug)]
pub struct GitInfo {
    pub is_git_repo: bool,
    pub current_branch: Option<String>,
    pub remote_url: Option<String>,
    pub tags: Vec<String>,
}

pub struct Git {
    path: std::path::PathBuf,
    info: Option<GitInfo>,
}

impl Git {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let info = if path.join(".git").exists() {
            Some(parser::parse_git_info(&path))
        } else {
            None
        };

        Self { path, info }
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