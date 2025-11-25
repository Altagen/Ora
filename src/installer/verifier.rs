use anyhow::{Context, Result};
use std::path::Path;

use crate::config::repo::{ChecksumConfig, GpgConfig, RepoConfig};
use crate::installer::downloader::Downloader;
use crate::security::{parse_checksum_file, verify_checksum, verify_signature};
use crate::storage::cache::Cache;

pub struct Verifier {
    downloader: Downloader,
}

impl Verifier {
    pub fn new() -> Result<Self> {
        Ok(Self {
            downloader: Downloader::new()?,
        })
    }

    pub async fn verify(
        &self,
        file_path: &Path,
        config: &RepoConfig,
        version: &str,
        os: &str,
        arch: &str,
        allow_insecure: bool,
    ) -> Result<()> {
        log::debug!("Verifying downloaded file: {:?}", file_path);

        // Check if we should skip verification
        if allow_insecure {
            log::warn!("Skipping security verification (--insecure flag)");
            return Ok(());
        }

        // Verify checksum if configured
        if let Some(checksum_config) = &config.security.checksum {
            self.verify_checksum_from_config(file_path, checksum_config, version, os, arch)
                .await?;
        } else if config.security.allow_insecure {
            log::warn!("No checksum configured and allow_insecure is true");
        } else {
            anyhow::bail!("No checksum configured and allow_insecure is false");
        }

        // Verify GPG signature if configured
        if let Some(gpg_config) = &config.security.gpg {
            self.verify_signature_from_config(file_path, gpg_config, version, os, arch)
                .await?;
        }

        log::debug!("Verification completed successfully");
        Ok(())
    }

    async fn verify_checksum_from_config(
        &self,
        file_path: &Path,
        checksum_config: &ChecksumConfig,
        version: &str,
        os: &str,
        arch: &str,
    ) -> Result<()> {
        use crate::utils::templating::resolve_template_safe;
        use std::collections::HashMap;

        let mut vars = HashMap::new();
        vars.insert("version".to_string(), version.to_string());
        vars.insert("os".to_string(), os.to_string());
        vars.insert("arch".to_string(), arch.to_string());

        let checksum_url = resolve_template_safe(&checksum_config.url, &vars)
            .context("Failed to resolve checksum URL template")?;

        log::debug!("Downloading checksum from: {}", checksum_url);
        let checksum_content = self
            .downloader
            .download_text(&checksum_url)
            .await
            .context("Failed to download checksum")?;

        let expected_hash = if checksum_config.is_single_hash() {
            // Extract only the hash part (first whitespace-delimited token)
            // The file may contain "hash  filename" format
            checksum_content
                .split_whitespace()
                .next()
                .context("Empty checksum file")?
                .to_string()
        } else {
            // Parse multi-hash file
            let filename = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .context("Invalid filename")?;

            parse_checksum_file(&checksum_content, filename).ok_or_else(|| {
                anyhow::anyhow!("Failed to find checksum for file in checksum file")
            })?
        };

        verify_checksum(file_path, &expected_hash, &checksum_config.algorithm).await?;

        Ok(())
    }

    async fn verify_signature_from_config(
        &self,
        file_path: &Path,
        gpg_config: &GpgConfig,
        version: &str,
        os: &str,
        arch: &str,
    ) -> Result<()> {
        use crate::utils::templating::resolve_template_safe;
        use std::collections::HashMap;

        let mut vars = HashMap::new();
        vars.insert("version".to_string(), version.to_string());
        vars.insert("os".to_string(), os.to_string());
        vars.insert("arch".to_string(), arch.to_string());

        let signature_url = resolve_template_safe(&gpg_config.signature_url, &vars)
            .context("Failed to resolve signature URL template")?;

        log::debug!("Downloading signature from: {}", signature_url);

        // Download signature file
        let file_name = file_path
            .file_name()
            .context("Invalid file path: no filename")?
            .to_string_lossy();
        let sig_filename = format!("{}.sig", file_name);
        let sig_path = Cache::download_path(&sig_filename)?;

        self.downloader.download(&signature_url, &sig_path).await?;

        // Verify signature
        verify_signature(file_path, &sig_path, gpg_config.public_key.as_deref()).await?;

        Ok(())
    }
}
