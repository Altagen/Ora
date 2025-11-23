// Cache module for storing temporary downloads and registry data
use anyhow::Result;
use std::path::PathBuf;

use crate::storage::paths::Paths;

pub struct Cache;

impl Cache {
    pub fn download_path(filename: &str) -> Result<PathBuf> {
        // Validate filename
        if filename.is_empty() {
            anyhow::bail!("Cannot create download path: filename is empty");
        }

        let cache_dir = Paths::cache_dir()?;
        let downloads_dir = cache_dir.join("downloads");

        // Ensure downloads directory exists and is actually a directory
        if downloads_dir.exists() && !downloads_dir.is_dir() {
            anyhow::bail!(
                "Downloads path exists but is not a directory: {}",
                downloads_dir.display()
            );
        }

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
