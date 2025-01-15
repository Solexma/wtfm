use super::GitInfo;
use std::fs;
use std::path::Path;

pub(crate) fn parse_git_info(git_path: &Path) -> GitInfo {
    let current_branch = read_current_branch(git_path);
    let remote_url = read_remote_url(git_path);
    let tags = read_tags(git_path);

    GitInfo {
        is_git_repo: true,
        current_branch,
        remote_url,
        tags,
    }
}

fn read_current_branch(git_path: &Path) -> Option<String> {
    let head_path = git_path.join("HEAD");
    fs::read_to_string(head_path)
        .ok()?
        .split("ref: refs/heads/")
        .nth(1)?
        .trim()
        .to_string()
        .into()
}

/// Extracts the remote URL from the git config file.
///
/// # TODO
/// - Handle multiple remotes (currently returns only the first URL found)
/// - Add preference for "origin" remote
/// - Consider returning a Vec<String> for all remotes and let the user choose the one they want
fn read_remote_url(git_path: &Path) -> Option<String> {
    let config_content = fs::read_to_string(git_path.join("config")).ok()?;
    config_content
        .lines()
        .skip_while(|line| !line.trim().starts_with("[remote"))
        .find_map(|line| {
            if line.trim().starts_with("url = ") {
                Some(line.trim().strip_prefix("url = ")?.trim().to_string())
            } else {
                None
            }
        })
}

/// Extracts the tags from the git repository.
///
/// # TODO
/// - Handle multiple tags (currently returns only the first tag found)
/// - Consider returning a Vec<String> for all tags and let the user choose the one they want
fn read_tags(git_path: &Path) -> Vec<String> {
    let tags = git_path.join("refs/tags");
    let tags = fs::read_dir(tags).unwrap();
    let tags = tags
        .map(|tag| {
            tag.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    tags
}
