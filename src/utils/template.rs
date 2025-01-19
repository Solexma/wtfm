use crate::config::wtfm::WtfmConfig;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use tera::{Context, Tera};

pub fn generate_readme_with_template(config: &WtfmConfig) -> Result<String, Box<dyn Error>> {
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates");
    let template_glob = template_path.join("**/*.tera");
    let template_pattern = template_glob.to_str().ok_or("Invalid template path")?;

    let mut tera = match Tera::new(template_pattern) {
        Ok(t) => t,
        Err(e) => {
            return Err(format!("Failed to initialize Tera: {}", e).into());
        }
    };

    tera.register_function("newline", |_args: &HashMap<String, tera::Value>| {
        Ok(tera::Value::String("\n".to_string()))
    });

    let mut context = Context::new();
    context.insert("project_name", &config.project_name);
    context.insert("description", &config.description);
    context.insert("version", &config.version);
    context.insert("license", &config.license);
    context.insert("setup_ci", &config.setup_ci);
    context.insert("authors", &config.authors);
    context.insert("features", &config.features);
    context.insert("prerequisites", &config.prerequisites);
    context.insert("install_steps", &config.install_steps);
    context.insert("ci_platform", &config.ci_platform);
    context.insert("ci_features", &config.ci_features);
    context.insert("ci_branches", &config.ci_branches);

    match tera.render("readme.tera", &context) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("Template rendering error: {}", e).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::wtfm::WtfmConfig;

    #[test]
    fn test_generate_readme() {
        let config = WtfmConfig {
            project_name: "Test Project".to_string(),
            description: "A test project".to_string(),
            ..Default::default()
        };
        let readme = generate_readme_with_template(&config).unwrap();
        assert!(readme.contains(&config.project_name));
        assert!(readme.contains(&config.description));
    }
}
