use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct InstalledDatabase {
    #[serde(default)]
    pub packages: HashMap<String, InstalledPackage>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub installed_at: DateTime<Utc>,
    pub install_mode: String,
    pub install_dir: String,
    pub files: Vec<String>,
    pub symlinks: Vec<String>,
    pub registry_source: String,
    #[serde(default)]
    pub checksums: HashMap<String, String>,
    /// Whether the package was installed with --allow-insecure flag
    #[serde(default)]
    pub allow_insecure: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_installed_package_serialization_with_allow_insecure() {
        let package = InstalledPackage {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            installed_at: Utc::now(),
            install_mode: "userland".to_string(),
            install_dir: "/test/dir".to_string(),
            files: vec!["file1".to_string()],
            symlinks: vec!["link1".to_string()],
            registry_source: "test-registry".to_string(),
            checksums: HashMap::new(),
            allow_insecure: true,
        };

        let serialized = toml::to_string(&package).unwrap();
        let deserialized: InstalledPackage = toml::from_str(&serialized).unwrap();

        assert!(deserialized.allow_insecure);
    }

    #[test]
    fn test_installed_package_deserialization_without_allow_insecure() {
        // BUG-2: Test that allow_insecure defaults to false when not present
        let toml_str = r#"
            name = "test-package"
            version = "1.0.0"
            installed_at = "2024-01-01T00:00:00Z"
            install_mode = "userland"
            install_dir = "/test/dir"
            files = ["file1"]
            symlinks = ["link1"]
            registry_source = "test-registry"
        "#;

        let package: InstalledPackage = toml::from_str(toml_str).unwrap();
        assert!(!package.allow_insecure);
    }

    #[test]
    fn test_installed_package_deserialization_with_allow_insecure_false() {
        let toml_str = r#"
            name = "test-package"
            version = "1.0.0"
            installed_at = "2024-01-01T00:00:00Z"
            install_mode = "userland"
            install_dir = "/test/dir"
            files = ["file1"]
            symlinks = ["link1"]
            registry_source = "test-registry"
            allow_insecure = false
        "#;

        let package: InstalledPackage = toml::from_str(toml_str).unwrap();
        assert!(!package.allow_insecure);
    }

    #[test]
    fn test_installed_package_deserialization_with_allow_insecure_true() {
        let toml_str = r#"
            name = "test-package"
            version = "1.0.0"
            installed_at = "2024-01-01T00:00:00Z"
            install_mode = "userland"
            install_dir = "/test/dir"
            files = ["file1"]
            symlinks = ["link1"]
            registry_source = "test-registry"
            allow_insecure = true
        "#;

        let package: InstalledPackage = toml::from_str(toml_str).unwrap();
        assert!(package.allow_insecure);
    }
}
