use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;

use crate::config::repo::RepoConfig;
use crate::providers::traits::{Version, VersionProvider};
use crate::utils::http::HttpClient;
use crate::utils::templating::resolve_template_safe;
use std::collections::HashMap;

pub struct GithubProvider {
    repo: String,
    config: RepoConfig,
    client: HttpClient,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    name: String,
    published_at: String,
    prerelease: bool,
}

impl GithubProvider {
    pub fn new(repo: String, config: RepoConfig) -> Result<Self> {
        Ok(Self {
            repo,
            config,
            client: HttpClient::new()?,
        })
    }

    fn api_url(&self) -> String {
        format!("https://api.github.com/repos/{}/releases", self.repo)
    }
}

#[async_trait]
impl VersionProvider for GithubProvider {
    async fn list_versions(&self) -> Result<Vec<Version>> {
        log::info!("Fetching versions from GitHub: {}", self.repo);

        let url = self.api_url();
        let releases: Vec<GithubRelease> = self
            .client
            .get_json(&url)
            .await
            .context("Failed to fetch GitHub releases")?;

        Ok(releases
            .into_iter()
            .map(|r| Version {
                tag: r.tag_name,
                name: r.name,
                published_at: r.published_at,
                prerelease: r.prerelease,
            })
            .collect())
    }

    async fn get_download_url(&self, version: &str, os: &str, arch: &str) -> Result<String> {
        let mut vars = HashMap::new();
        vars.insert("version".to_string(), version.to_string());
        vars.insert("os".to_string(), os.to_string());
        vars.insert("arch".to_string(), arch.to_string());

        let download_config = self.config.source.download.as_ref()
            .context("No download configuration found")?;

        if let Some(url_template) = &download_config.url {
            resolve_template_safe(url_template, &vars)
                .context("Failed to resolve download URL template")
        } else if let Some(urls) = &download_config.urls {
            let platform_key = format!("{}_{}", os, arch);
            urls.get(&platform_key)
                .cloned()
                .context(format!("No download URL for platform: {}", platform_key))
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
