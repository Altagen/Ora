use anyhow::Result;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Mock Git registry for testing
pub struct MockRegistry {
    _temp_dir: TempDir,
    repo_path: PathBuf,
}

impl MockRegistry {
    /// Create a new mock registry with .repo files
    pub fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize git repository
        let repo = git2::Repository::init(&repo_path)?;

        // Create ora-registry directory (expected structure for registries)
        let packages_dir = repo_path.join("ora-registry");
        std::fs::create_dir_all(&packages_dir)?;

        // Copy .repo files from fixtures
        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("repo_files");

        if fixtures_dir.exists() {
            for entry in std::fs::read_dir(&fixtures_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("repo") {
                    let dest = packages_dir.join(entry.file_name());
                    std::fs::copy(&path, &dest)?;
                }
            }
        }

        // Create initial commit
        let mut index = repo.index()?;
        index.add_all(
            ["ora-registry/*.repo"].iter(),
            git2::IndexAddOption::DEFAULT,
            None,
        )?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let signature = git2::Signature::now("Ora Test", "test@example.com")?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit with test packages",
            &tree,
            &[],
        )?;

        Ok(Self {
            _temp_dir: temp_dir,
            repo_path,
        })
    }

    /// Get the path to the mock registry
    pub fn path(&self) -> &Path {
        &self.repo_path
    }

    /// Get the file:// URL for cloning
    pub fn url(&self) -> String {
        format!("file://{}", self.repo_path.display())
    }

    /// Add a new .repo file to the registry
    #[allow(dead_code)]
    pub fn add_repo_file(&self, name: &str, content: &str) -> Result<()> {
        let packages_dir = self.repo_path.join("ora-registry");
        let file_path = packages_dir.join(format!("{}.repo", name));

        std::fs::write(&file_path, content)?;

        // Commit the change
        let repo = git2::Repository::open(&self.repo_path)?;
        let mut index = repo.index()?;
        index.add_path(Path::new(&format!("ora-registry/{}.repo", name)))?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let signature = git2::Signature::now("Ora Test", "test@example.com")?;
        let parent = repo.head()?.peel_to_commit()?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &format!("Add {} package", name),
            &tree,
            &[&parent],
        )?;

        Ok(())
    }

    /// List all packages in the registry
    pub fn list_packages(&self) -> Result<Vec<String>> {
        let packages_dir = self.repo_path.join("ora-registry");
        let mut packages = Vec::new();

        if packages_dir.exists() {
            for entry in std::fs::read_dir(&packages_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("repo") {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        packages.push(name.to_string());
                    }
                }
            }
        }

        packages.sort();
        Ok(packages)
    }
}

impl Default for MockRegistry {
    fn default() -> Self {
        Self::new().expect("Failed to create mock registry")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_registry_creation() {
        let registry = MockRegistry::new().unwrap();
        assert!(registry.path().exists());
        assert!(registry.path().join("ora-registry").exists());
    }

    #[test]
    fn test_mock_registry_url() {
        let registry = MockRegistry::new().unwrap();
        let url = registry.url();
        assert!(url.starts_with("file://"));
    }

    #[test]
    fn test_list_packages() {
        let registry = MockRegistry::new().unwrap();
        let packages = registry.list_packages().unwrap();
        assert!(!packages.is_empty());
        // Should contain our test packages
        assert!(packages.contains(&"windman".to_string()));
        assert!(packages.contains(&"prometheus".to_string()));
    }
}
