use anyhow::{Context, Result};
use async_trait::async_trait;

use crate::config::repo::RepoConfig;
use crate::providers::traits::{Version, VersionProvider};
use crate::utils::templating::resolve_template_safe;
use std::collections::HashMap;

pub struct DirectUrlProvider {
    config: RepoConfig,
}

impl DirectUrlProvider {
    pub fn new(config: RepoConfig) -> Result<Self> {
        Ok(Self { config })
    }
}

#[async_trait]
impl VersionProvider for DirectUrlProvider {
    async fn list_versions(&self) -> Result<Vec<Version>> {
        // Direct URL doesn't support version listing
        // Return a single dummy version
        Ok(vec![Version {
            tag: "latest".to_string(),
            name: "Latest".to_string(),
            published_at: chrono::Utc::now().to_rfc3339(),
            prerelease: false,
        }])
    }

    async fn get_download_url(&self, version: &str, os: &str, arch: &str) -> Result<String> {
        let mut vars = HashMap::new();
        vars.insert("version".to_string(), version.to_string());
        vars.insert("os".to_string(), os.to_string());
        vars.insert("arch".to_string(), arch.to_string());

        let download_config = self
            .config
            .source
            .download
            .as_ref()
            .context("No download configuration found")?;

        if let Some(url_template) = &download_config.url {
            resolve_template_safe(url_template, &vars)
                .context("Failed to resolve download URL template")
        } else if let Some(urls) = &download_config.urls {
            let platform_key = format!("{}_{}", os, arch);
            urls.get(&platform_key)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("No download URL for platform: {}", platform_key))
        } else {
            anyhow::bail!("No download configuration found")
        }
    }

    async fn get_checksum_url(
        &self,
        version: &str,
        os: &str,
        arch: &str,
    ) -> Result<Option<String>> {
        if let Some(checksum_config) = &self.config.security.checksum {
            let mut vars = HashMap::new();
            vars.insert("version".to_string(), version.to_string());
            vars.insert("os".to_string(), os.to_string());
            vars.insert("arch".to_string(), arch.to_string());

            let url = resolve_template_safe(&checksum_config.url, &vars)
                .context("Failed to resolve checksum URL template")?;
            Ok(Some(url))
        } else {
            Ok(None)
        }
    }

    async fn get_signature_url(
        &self,
        version: &str,
        os: &str,
        arch: &str,
    ) -> Result<Option<String>> {
        if let Some(gpg_config) = &self.config.security.gpg {
            let mut vars = HashMap::new();
            vars.insert("version".to_string(), version.to_string());
            vars.insert("os".to_string(), os.to_string());
            vars.insert("arch".to_string(), arch.to_string());

            let url = resolve_template_safe(&gpg_config.signature_url, &vars)
                .context("Failed to resolve signature URL template")?;
            Ok(Some(url))
        } else {
            Ok(None)
        }
    }
}
