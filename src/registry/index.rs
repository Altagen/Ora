use anyhow::Result;

use crate::storage::cache::Cache;

pub struct RegistryIndex;

impl RegistryIndex {
    pub async fn search_packages(registry_name: &str, query: &str) -> Result<Vec<String>> {
        let registry_path = Cache::registry_path(registry_name)?;
        let packages_dir = registry_path.join("packages");

        if !packages_dir.exists() {
            return Ok(vec![]);
        }

        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        // Read all .repo files in packages directory
        if let Ok(entries) = std::fs::read_dir(&packages_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".repo") {
                        let package_name = filename.trim_end_matches(".repo");
                        if package_name.to_lowercase().contains(&query_lower) {
                            results.push(package_name.to_string());
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Reserved for future use when listing all packages is needed.
    #[allow(dead_code)]
    pub async fn list_all_packages(registry_name: &str) -> Result<Vec<String>> {
        let registry_path = Cache::registry_path(registry_name)?;
        let packages_dir = registry_path.join("packages");

        if !packages_dir.exists() {
            return Ok(vec![]);
        }

        let mut results = Vec::new();

        // Read all .repo files in packages directory
        if let Ok(entries) = std::fs::read_dir(&packages_dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".repo") {
                        let package_name = filename.trim_end_matches(".repo");
                        results.push(package_name.to_string());
                    }
                }
            }
        }

        results.sort();
        Ok(results)
    }
}
