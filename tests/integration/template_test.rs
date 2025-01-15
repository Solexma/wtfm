use wtfm::config::wizard::WizardAnswers;

#[test]
fn test_template_generation() {
    let answers = WizardAnswers::new_test();
    assert!(!answers.project_name.is_empty());
    assert!(!answers.description.is_empty());
}
