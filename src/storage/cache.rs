// Cache module for storing temporary downloads and registry data
use anyhow::Result;
use std::path::PathBuf;

use crate::storage::paths::Paths;

pub struct Cache;

impl Cache {
    pub fn download_path(filename: &str) -> Result<PathBuf> {
        let cache_dir = Paths::cache_dir()?;
        let downloads_dir = cache_dir.join("downloads");
        std::fs::create_dir_all(&downloads_dir)?;
        Ok(downloads_dir.join(filename))
    }

    pub fn registry_path(registry_name: &str) -> Result<PathBuf> {
        let registries_dir = Paths::registries_cache_dir()?;
        std::fs::create_dir_all(&registries_dir)?;
        Ok(registries_dir.join(registry_name))
    }

    pub fn clear_downloads() -> Result<()> {
        let cache_dir = Paths::cache_dir()?;
        let downloads_dir = cache_dir.join("downloads");
        if downloads_dir.exists() {
            std::fs::remove_dir_all(&downloads_dir)?;
        }
        Ok(())
    }
}

// Note: Temporary file/directory guard structs removed
// Using tempfile crate directly (tempfile::NamedTempFile, tempfile::TempDir) instead
