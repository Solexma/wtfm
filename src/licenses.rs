use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LicenseCategory {
    Permissive,
    Copyleft,
    WeakCopyleft,
    PublicDomain,
    Creative,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct License {
    pub spdx_id: String,
    pub name: String,
    pub category: LicenseCategory,
    pub osi_approved: bool,
    pub fsf_libre: bool,
    pub deprecated: bool,
    pub url: Option<String>,
}

impl License {
    pub fn new(
        spdx_id: &str,
        name: &str,
        category: LicenseCategory,
        osi_approved: bool,
        fsf_libre: bool,
        deprecated: bool,
    ) -> Self {
        Self {
            spdx_id: spdx_id.to_string(),
            name: name.to_string(),
            category,
            osi_approved,
            fsf_libre,
            deprecated,
            url: None,
        }
    }

    pub fn get_licenses() -> Vec<License> {
        vec![
            License::new(
                "MIT",
                "MIT License",
                LicenseCategory::Permissive,
                true,
                true,
                false,
            ),
            License::new(
                "Apache-2.0",
                "Apache License 2.0",
                LicenseCategory::Permissive,
                true,
                true,
                false,
            ),
            License::new(
                "GPL-3.0-only",
                "GNU General Public License v3.0 only",
                LicenseCategory::Copyleft,
                true,
                true,
                false,
            ),
            // Add more licenses...
        ]
    }

    pub fn get_active_licenses() -> Vec<License> {
        Self::get_licenses()
            .into_iter()
            .filter(|l| !l.deprecated)
            .collect()
    }

    pub fn get_osi_approved() -> Vec<License> {
        Self::get_licenses()
            .into_iter()
            .filter(|l| l.osi_approved && !l.deprecated)
            .collect()
    }

    pub fn get_fsf_libre() -> Vec<License> {
        Self::get_licenses()
            .into_iter()
            .filter(|l| l.fsf_libre && !l.deprecated)
            .collect()
    }
}

impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_creation() {
        let license = License::new(
            "MIT",
            "MIT License",
            LicenseCategory::Permissive,
            true,
            true,
            false,
        );
        assert_eq!(license.spdx_id, "MIT");
        assert_eq!(license.name, "MIT License");
        assert_eq!(license.category, LicenseCategory::Permissive);
        assert!(license.osi_approved);
        assert!(license.fsf_libre);
        assert!(!license.deprecated);
        assert_eq!(license.url, None);
    }

    #[test]
    fn test_filter_methods() {
        let licenses = License::get_licenses();
        assert!(!licenses.is_empty());

        let active = License::get_active_licenses();
        assert!(active.iter().all(|l| !l.deprecated));

        let osi = License::get_osi_approved();
        assert!(osi.iter().all(|l| l.osi_approved && !l.deprecated));

        let fsf = License::get_fsf_libre();
        assert!(fsf.iter().all(|l| l.fsf_libre && !l.deprecated));
    }
}
