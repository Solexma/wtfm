use crate::config::wizard::WizardAnswers;
use tera::{Context, Tera};

pub fn generate_readme_with_template(answers: &WizardAnswers) -> String {
    // Embed the template directly in the binary
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

    #[test]
    fn test_generate_readme() {
        let answers = WizardAnswers {
            project_name: "Test Project".to_string(),
            description: "A test project".to_string(),
            version: "0.1.0".to_string(),
            license: "MIT".to_string(),
            setup_ci: true,
            author_quantity: 2,
            authors: vec!["Author 1".to_string(), "Author 2".to_string()],
        };

        let readme = generate_readme_with_template(&answers);
        
        // Verify that README contains correct information
        assert!(readme.contains("# Test Project 0.1.0"));
        assert!(readme.contains("A test project"));
        assert!(readme.contains("Licensed under MIT"));
        assert!(readme.contains("Continuous Integration"));
        assert!(readme.contains("Author 1"));
        assert!(readme.contains("Author 2"));
    }

    #[test]
    fn test_generate_readme_without_ci() {
        let answers = WizardAnswers {
            project_name: "Test Project".to_string(),
            description: "A test project".to_string(),
            version: "0.1.0".to_string(),
            license: "MIT".to_string(),
            setup_ci: false,
            author_quantity: 2,
            authors: vec!["Author 1".to_string(), "Author 2".to_string()],
        };

        let readme = generate_readme_with_template(&answers);
        
        // Verify that CI section is not included when setup_ci is false
        assert!(!readme.contains("Continuous Integration"));
        assert!(readme.contains("Author 1"));
        assert!(readme.contains("Author 2"));
    }
}