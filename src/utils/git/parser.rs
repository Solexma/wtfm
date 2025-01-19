use super::types::GitInfo;
use std::path::Path;

#[allow(dead_code)]
pub fn parse_git_info(git_path: &Path) -> GitInfo {
    GitInfo {
        path: git_path.to_path_buf(),
        ..Default::default()
    }
}
