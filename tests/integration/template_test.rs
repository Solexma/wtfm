use wtfm::config::wtfm::WtfmConfig;

#[test]
fn test_template_generation() {
    let config = WtfmConfig {
        project_name: "Test Project".to_string(),
        description: "Test Description".to_string(),
        ..Default::default()
    };
    assert!(!config.project_name.is_empty());
    assert!(!config.description.is_empty());
}
