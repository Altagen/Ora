use crate::config::global::InstallMode;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct Paths;

impl Paths {
    pub fn config_dir() -> Result<PathBuf> {
        // Check for ORA_CONFIG_DIR env var first (for testing)
        if let Ok(dir) = std::env::var("ORA_CONFIG_DIR") {
            return Ok(PathBuf::from(dir));
        }

        dirs::config_dir()
            .context("Failed to get config directory")
            .map(|p| p.join("ora"))
    }

    pub fn config_file() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn installed_db_file() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("installed.toml"))
    }

    pub fn data_dir() -> Result<PathBuf> {
        // Check for ORA_DATA_DIR env var first (for testing)
        if let Ok(dir) = std::env::var("ORA_DATA_DIR") {
            return Ok(PathBuf::from(dir));
        }

        dirs::data_local_dir()
            .context("Failed to get data directory")
            .map(|p| p.join("ora"))
    }

    pub fn packages_dir(mode: InstallMode) -> Result<PathBuf> {
        match mode {
            InstallMode::Userland => Ok(Self::data_dir()?.join("packages")),
            InstallMode::System => Ok(PathBuf::from("/opt/ora/packages")),
        }
    }

    pub fn bin_dir(mode: InstallMode) -> Result<PathBuf> {
        match mode {
            InstallMode::Userland => dirs::home_dir()
                .context("Failed to get home directory")
                .map(|p| p.join(".local/bin")),
            InstallMode::System => Ok(PathBuf::from("/usr/local/bin")),
        }
    }

    pub fn cache_dir() -> Result<PathBuf> {
        // Check for ORA_CACHE_DIR env var first (for testing)
        if let Ok(dir) = std::env::var("ORA_CACHE_DIR") {
            return Ok(PathBuf::from(dir));
        }

        dirs::cache_dir()
            .context("Failed to get cache directory")
            .map(|p| p.join("ora"))
    }

    pub fn registries_cache_dir() -> Result<PathBuf> {
        Ok(Self::cache_dir()?.join("registries"))
    }

    pub fn audit_log_file() -> Result<PathBuf> {
        Ok(Self::data_dir()?.join("audit.log"))
    }

    pub fn ensure_directories() -> Result<()> {
        std::fs::create_dir_all(Self::config_dir()?)?;
        std::fs::create_dir_all(Self::data_dir()?)?;
        std::fs::create_dir_all(Self::cache_dir()?)?;
        std::fs::create_dir_all(Self::registries_cache_dir()?)?;
        Ok(())
    }
}
