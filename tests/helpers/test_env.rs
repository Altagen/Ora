use anyhow::Result;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Test environment that isolates Ora operations
pub struct TestEnvironment {
    _temp_dir: TempDir,
    config_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
    install_dir: PathBuf,
    bin_dir: PathBuf,
}

impl TestEnvironment {
    /// Create a new isolated test environment
    pub fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let base = temp_dir.path();

        let config_dir = base.join("config");
        let data_dir = base.join("data");
        let cache_dir = base.join("cache");
        let install_dir = base.join("install");
        let bin_dir = base.join("bin");

        // Create all directories
        std::fs::create_dir_all(&config_dir)?;
        std::fs::create_dir_all(&data_dir)?;
        std::fs::create_dir_all(&cache_dir)?;
        std::fs::create_dir_all(&install_dir)?;
        std::fs::create_dir_all(&bin_dir)?;

        // Create a permissive security config for tests
        let security_config = config_dir.join("security.toml");
        std::fs::write(&security_config, r#"
[network.git]
https_only = false
allowed_schemes = ["https", "http", "file"]

[network]
max_download_size = 10737418240  # 10GB for tests
request_timeout = 300
max_redirects = 10

[validation]
max_archive_size = 10737418240  # 10GB
max_extracted_size = 21474836480  # 20GB
"#)?;

        Ok(Self {
            _temp_dir: temp_dir,
            config_dir,
            data_dir,
            cache_dir,
            install_dir,
            bin_dir,
        })
    }

    /// Get config directory path
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    /// Get data directory path
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Get cache directory path
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Get install directory path
    pub fn install_dir(&self) -> &Path {
        &self.install_dir
    }

    /// Get bin directory path
    pub fn bin_dir(&self) -> &Path {
        &self.bin_dir
    }

    /// Get the base temporary directory
    pub fn base_dir(&self) -> &Path {
        self._temp_dir.path()
    }

    /// Set environment variables to use this test environment
    #[allow(dead_code)]
    pub fn set_env_vars(&self) {
        std::env::set_var("ORA_CONFIG_DIR", &self.config_dir);
        std::env::set_var("ORA_DATA_DIR", &self.data_dir);
        std::env::set_var("ORA_CACHE_DIR", &self.cache_dir);
    }

    /// Unset environment variables
    pub fn unset_env_vars(&self) {
        std::env::remove_var("ORA_CONFIG_DIR");
        std::env::remove_var("ORA_DATA_DIR");
        std::env::remove_var("ORA_CACHE_DIR");
    }

    /// Check if a package is installed
    #[allow(dead_code)]
    pub fn is_package_installed(&self, name: &str) -> bool {
        let installed_db = self.config_dir.join("installed.toml");
        if !installed_db.exists() {
            return false;
        }

        if let Ok(content) = std::fs::read_to_string(&installed_db) {
            content.contains(&format!("name = \"{}\"", name))
        } else {
            false
        }
    }

    /// Check if a binary symlink exists
    #[allow(dead_code)]
    pub fn has_binary_link(&self, name: &str) -> bool {
        self.bin_dir.join(name).exists()
    }

    /// List installed packages
    #[allow(dead_code)]
    pub fn list_installed_packages(&self) -> Result<Vec<String>> {
        let installed_db = self.config_dir.join("installed.toml");
        if !installed_db.exists() {
            return Ok(vec![]);
        }

        let content = std::fs::read_to_string(&installed_db)?;
        let db: toml::Value = toml::from_str(&content)?;

        let mut packages = Vec::new();
        if let Some(pkgs) = db.get("packages").and_then(|p| p.as_table()) {
            for (name, _) in pkgs {
                packages.push(name.clone());
            }
        }

        packages.sort();
        Ok(packages)
    }

    /// Clean up the environment
    #[allow(dead_code)]
    pub fn cleanup(&self) {
        self.unset_env_vars();
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        self.unset_env_vars();
    }
}

impl Default for TestEnvironment {
    fn default() -> Self {
        Self::new().expect("Failed to create test environment")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_creation() {
        let env = TestEnvironment::new().unwrap();
        assert!(env.config_dir().exists());
        assert!(env.data_dir().exists());
        assert!(env.cache_dir().exists());
        assert!(env.install_dir().exists());
        assert!(env.bin_dir().exists());
    }

    #[test]
    fn test_env_isolation() {
        let env1 = TestEnvironment::new().unwrap();
        let env2 = TestEnvironment::new().unwrap();
        assert_ne!(env1.base_dir(), env2.base_dir());
    }
}
