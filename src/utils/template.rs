use crate::config::wizard::WizardAnswers;
use std::collections::HashMap;
use tera::{Context, Tera, Value};

fn newline_fn(_args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    Ok(Value::String("\n".to_string()))
}

// Template structure:
// In the beginning we will implement a standard GitHub README.md template
// Then we will move to more structured forms like DocBook and Diataxis
pub fn generate_readme_with_template(answers: &WizardAnswers) -> String {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            panic!("Parsing error(s): {}", e);
        }
    };

    // Registra la funzione newline
    tera.register_function("newline", newline_fn);

    let mut context = Context::new();
    context.insert("project_name", &answers.project_name);
    context.insert("description", &answers.description);
    context.insert("version", &answers.version);
    context.insert("license", &answers.license);
    context.insert("setup_ci", &answers.setup_ci);
    context.insert("authors", &answers.authors);

    match tera.render("readme.tera", &context) {
        Ok(rendered) => rendered,
        Err(e) => {
            panic!("Failed to render template: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::wizard::WizardAnswers;
    use crate::licenses::{License, LicenseCategory};

    fn create_test_answers() -> WizardAnswers {
        WizardAnswers {
            project_name: "Test Project".to_string(),
            description: "A test project".to_string(),
            version: "0.1.0".to_string(),
            license: License::new(
                "MIT",
                "MIT License",
                LicenseCategory::Permissive,
                true,
                true,
                false,
            ),
            setup_ci: true,
            author_quantity: 2,
            authors: vec!["Author 1".to_string(), "Author 2".to_string()],
        }
    }

    #[test]
    fn test_generate_readme() {
        let answers = create_test_answers();
        let readme = generate_readme_with_template(&answers);
        assert!(readme.contains(&answers.project_name));
        assert!(readme.contains(&answers.license.spdx_id));
    }

    #[test]
    fn test_generate_readme_without_ci() {
        let mut answers = create_test_answers();
        answers.setup_ci = false;
        let readme = generate_readme_with_template(&answers);
        assert!(!readme.contains("Continuous Integration"));
    }
}
