use wtfm::config::wizard::WizardAnswers;
use wtfm::licenses::License;

#[test]
#[ignore = "Manual test for debugging license selection"]
fn manual_license_selection() {
    println!("Available licenses:");
    for license in License::get_licenses() {
        println!("- {} ({})", license.name, license.spdx_id);
    }

    println!("\nStarting license selection wizard...");
    let license = WizardAnswers::from_interactive().license;
    println!("\nSelected license: {} ({})", license.name, license.spdx_id);
    println!("Category: {:?}", license.category);
    println!("OSI Approved: {}", license.osi_approved);
    println!("FSF Libre: {}", license.fsf_libre);
}
