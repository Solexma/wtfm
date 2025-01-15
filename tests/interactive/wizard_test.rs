use wtfm::config::wizard::WizardAnswers;
use wtfm::licenses::License;

#[test]
#[ignore]
fn test_license_selection() {
    let available_licenses = License::get_licenses();
    let license = WizardAnswers::from_interactive().license;
    assert!(available_licenses
        .iter()
        .any(|l| l.spdx_id == license.spdx_id));
}

#[test]
#[ignore]
fn test_wizard() {
    let answers = WizardAnswers::from_interactive();
    assert!(!answers.project_name.is_empty());
    assert!(!answers.description.is_empty());
}
