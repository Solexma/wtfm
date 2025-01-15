use super::CargoInfo;
use std::fs;
use std::path::Path;
use toml;

pub(crate) fn parse_cargo_info(cargo_path: &Path) -> CargoInfo {
    let package_name = read_package_name(cargo_path);
    let version = read_version(cargo_path);
    let authors = read_authors(cargo_path).unwrap_or_default();
    let description = read_description(cargo_path);
    let repository = read_repository(cargo_path);
    let keywords = read_keywords(cargo_path).unwrap_or_default();
    let categories = read_categories(cargo_path).unwrap_or_default();

    CargoInfo {
        package_name,
        version,
        authors,
        description,
        repository,
        keywords,
        categories,
    }
}

fn read_package_name(cargo_path: &Path) -> Option<String> {
    let content = fs::read_to_string(cargo_path).ok()?;
    let cargo_toml: toml::Value = content.parse().ok()?;

    cargo_toml
        .get("package")?
        .get("name")?
        .as_str()
        .map(String::from)
}

fn read_version(cargo_path: &Path) -> Option<String> {
    let content = fs::read_to_string(cargo_path).ok()?;
    let cargo_toml: toml::Value = content.parse().ok()?;

    cargo_toml
        .get("package")?
        .get("version")?
        .as_str()
        .map(String::from)
}

fn read_authors(cargo_path: &Path) -> Option<Vec<String>> {
    let content = fs::read_to_string(cargo_path).ok()?;
    let cargo_toml: toml::Value = content.parse().ok()?;

    cargo_toml
        .get("package")?
        .get("authors")?
        .as_str()
        .map(|s| s.split(',').map(String::from).collect())
}

fn read_description(cargo_path: &Path) -> Option<String> {
    let content = fs::read_to_string(cargo_path).ok()?;
    let cargo_toml: toml::Value = content.parse().ok()?;

    cargo_toml
        .get("package")?
        .get("description")?
        .as_str()
        .map(String::from)
}

fn read_repository(cargo_path: &Path) -> Option<String> {
    let content = fs::read_to_string(cargo_path).ok()?;
    let cargo_toml: toml::Value = content.parse().ok()?;

    cargo_toml
        .get("package")?
        .get("repository")?
        .as_str()
        .map(String::from)
}

fn read_keywords(cargo_path: &Path) -> Option<Vec<String>> {
    let content = fs::read_to_string(cargo_path).ok()?;
    let cargo_toml: toml::Value = content.parse().ok()?;

    cargo_toml
        .get("package")?
        .get("keywords")?
        .as_str()
        .map(|s| s.split(',').map(String::from).collect())
}

fn read_categories(cargo_path: &Path) -> Option<Vec<String>> {
    let content = fs::read_to_string(cargo_path).ok()?;
    let cargo_toml: toml::Value = content.parse().ok()?;

    cargo_toml
        .get("package")?
        .get("categories")?
        .as_str()
        .map(|s| s.split(',').map(String::from).collect())
}
