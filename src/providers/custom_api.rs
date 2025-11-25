use anyhow::{Context, Result};
use async_trait::async_trait;

use crate::config::repo::RepoConfig;
use crate::providers::discovery::VersionDiscovery;
use crate::providers::traits::{Version, VersionProvider};
use crate::utils::http::HttpClient;
use crate::utils::templating::resolve_template_safe;
use std::collections::HashMap;

pub struct CustomApiProvider {
    #[allow(dead_code)]
    api_url: String,
    config: RepoConfig,
    #[allow(dead_code)]
    client: HttpClient,
}

impl CustomApiProvider {
    pub fn new(api_url: String, config: RepoConfig) -> Result<Self> {
        Ok(Self {
            api_url,
            config,
            client: HttpClient::new()?,
        })
    }
}

#[async_trait]
impl VersionProvider for CustomApiProvider {
    async fn list_versions(&self) -> Result<Vec<Version>> {
        // Use version discovery if configured
        if let Some(version_config) = &self.config.source.version {
            log::debug!("Using version discovery for custom API");
            let discovery = VersionDiscovery::new(version_config.clone())?;
            let version_strings = discovery.discover_versions().await?;

            // Convert to Version structs
            let versions: Vec<Version> = version_strings
                .into_iter()
                .map(|tag| Version {
                    tag: tag.clone(),
                    name: tag.clone(),
                    published_at: String::new(),
                    prerelease: tag.contains("alpha") || tag.contains("beta") || tag.contains("rc"),
                })
                .collect();

            return Ok(versions);
        }

        log::warn!("No version discovery configured for custom API");
        Ok(vec![])
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
            Ok(resolve_template_safe(url_template, &vars)?)
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

            Ok(Some(resolve_template_safe(&checksum_config.url, &vars)?))
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

            Ok(Some(resolve_template_safe(
                &gpg_config.signature_url,
                &vars,
            )?))
        } else {
            Ok(None)
        }
    }
}
