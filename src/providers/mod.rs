pub mod custom_api;
pub mod direct_url;
pub mod discovery;
pub mod github;
pub mod gitlab;
pub mod traits;
pub mod webpage_scraping;

use anyhow::Result;
use std::sync::Arc;

pub use traits::VersionProvider;

use crate::config::repo::{ProviderType, RepoConfig};
use custom_api::CustomApiProvider;
use direct_url::DirectUrlProvider;
use github::GithubProvider;
use gitlab::GitlabProvider;
use webpage_scraping::WebpageScrapingProvider;

pub fn create_provider(config: &RepoConfig) -> Result<Arc<dyn VersionProvider>> {
    match &config.source.provider_type {
        ProviderType::GithubReleases => {
            let repo = config
                .source
                .repo
                .clone()
                .ok_or_else(|| anyhow::anyhow!("GitHub provider requires 'repo' field"))?;
            Ok(Arc::new(GithubProvider::new(repo, config.clone())?))
        }
        ProviderType::GitlabReleases => {
            let repo = config
                .source
                .repo
                .clone()
                .ok_or_else(|| anyhow::anyhow!("GitLab provider requires 'repo' field"))?;
            Ok(Arc::new(GitlabProvider::new(
                repo,
                config.source.instance.clone(),
                config.clone(),
            )?))
        }
        ProviderType::CustomApi => {
            let api_url =
                config.source.api_url.clone().ok_or_else(|| {
                    anyhow::anyhow!("Custom API provider requires 'api_url' field")
                })?;
            Ok(Arc::new(CustomApiProvider::new(api_url, config.clone())?))
        }
        ProviderType::DirectUrl => Ok(Arc::new(DirectUrlProvider::new(config.clone())?)),
        ProviderType::WebpageScraping => Ok(Arc::new(WebpageScrapingProvider::new(config.clone())?)),
    }
}
