use tempfile::TempDir;
use wtfm::config::wtfm::WtfmConfig;
use wtfm::wizard::Wizard;

#[test]
fn test_wizard_save_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("wtfm.json");

    let config = WtfmConfig {
        project_name: "Test Project".to_string(),
        description: "Test Description".to_string(),
        ..Default::default()
    };

    let wizard = Wizard::new_with_config(config);
    assert!(wizard.save_config(&config_path).is_ok());
    assert!(config_path.exists());
}
