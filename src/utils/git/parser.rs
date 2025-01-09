use std::path::Path;
use super::GitInfo;

pub(crate) fn parse_git_info(path: &Path) -> GitInfo {
    let _git_path = path.join(".git");
    
    GitInfo {
        is_git_repo: true,
        current_branch: None,
        remote_url: None,
        tags: vec![],
    }
}

fn read_current_branch(_git_path: &Path) -> Option<String> {
    None
}

fn read_remote_url(_git_path: &Path) -> Option<String> {
    None
}

fn read_tags(_git_path: &Path) -> Vec<String> {
    vec![]
}