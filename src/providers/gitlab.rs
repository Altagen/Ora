use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;

use crate::config::repo::RepoConfig;
use crate::providers::traits::{Version, VersionProvider};
use crate::utils::http::HttpClient;
use crate::utils::templating::resolve_template_safe;
use std::collections::HashMap;

pub struct GitlabProvider {
    repo: String,
    instance: String,
    config: RepoConfig,
    client: HttpClient,
}

#[derive(Debug, Deserialize)]
struct GitlabRelease {
    tag_name: String,
    name: String,
    released_at: String,
}

impl GitlabProvider {
    pub fn new(repo: String, instance: Option<String>, config: RepoConfig) -> Result<Self> {
        Ok(Self {
            repo,
            instance: instance.unwrap_or_else(|| "https://gitlab.com".to_string()),
            config,
            client: HttpClient::new()?,
        })
    }

    fn api_url(&self) -> String {
        let encoded_repo = urlencoding::encode(&self.repo);
        format!(
            "{}/api/v4/projects/{}/releases",
            self.instance, encoded_repo
        )
    }
}

#[async_trait]
impl VersionProvider for GitlabProvider {
    async fn list_versions(&self) -> Result<Vec<Version>> {
        log::info!("Fetching versions from GitLab: {}", self.repo);

        let url = self.api_url();
        let releases: Vec<GitlabRelease> = self
            .client
            .get_json(&url)
            .await
            .context("Failed to fetch GitLab releases")?;

        Ok(releases
            .into_iter()
            .map(|r| Version {
                tag: r.tag_name,
                name: r.name,
                published_at: r.released_at,
                prerelease: false,
            })
            .collect())
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
