use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct GitInfo {
    pub path: PathBuf,
    pub branch: Option<String>,
    pub remote: Option<String>,
    pub tags: Vec<String>,
}
