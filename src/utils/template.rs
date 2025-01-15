use crate::config::wizard::WizardAnswers;
use tera::{Context, Tera};

// Template structure:
// In the beginning we will implement a standard GitHub README.md template
// Then we will move to more structured forms like DocBook and Diataxis
pub fn generate_readme_with_template(answers: &WizardAnswers) -> String {
    // At the moment we embed the template directly in the binary
    // TODO:
    // - Add a way to load the template from a file
    // - Add a way to load the template from a remote source
    let template_str = include_str!("../../templates/readme.tera");

    let mut tera = Tera::default();
    tera.add_raw_template("readme.tera", template_str)
        .expect("Failed to parse template");

    let mut context = Context::new();
    context.insert("project_name", &answers.project_name);
    context.insert("description", &answers.description);
    context.insert("version", &answers.version);
    context.insert("license", &answers.license);
    context.insert("setup_ci", &answers.setup_ci);
    context.insert("author_quantity", &answers.author_quantity);
    context.insert("authors", &answers.authors);

    tera.render("readme.tera", &context)
        .expect("Failed to render template")
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
