use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::repo::RepoConfig;
use crate::providers::traits::{Version, VersionProvider};
use crate::storage::paths::Paths;
use crate::utils::http::HttpClient;

/// Cached URL data for webpage scraping
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedUrl {
    url: String,
    version: String,
    platform: String,
}

/// Cache metadata
#[derive(Debug, Serialize, Deserialize)]
struct UrlCache {
    timestamp: u64,
    urls: Vec<CachedUrl>,
}

pub struct WebpageScrapingProvider {
    config: RepoConfig,
    client: HttpClient,
    cache: Option<UrlCache>,
}

impl WebpageScrapingProvider {
    pub fn new(config: RepoConfig) -> Result<Self> {
        Ok(Self {
            config,
            client: HttpClient::new()?,
            cache: None,
        })
    }

    /// Get cache file path for this provider
    fn cache_path(&self) -> Result<PathBuf> {
        let cache_dir = Paths::cache_dir()?.join("scrapers");
        std::fs::create_dir_all(&cache_dir)?;

        // Create unique cache file based on discovery URL
        let version_config = self.config.source.version.as_ref()
            .context("version discovery config required for webpage-scraping")?;

        let hash = format!("{:x}", md5::compute(&version_config.discovery_url));
        Ok(cache_dir.join(format!("{}.json", hash)))
    }

    /// Check if cache is still valid based on TTL
    async fn is_cache_valid(&self, cache: &UrlCache) -> Result<bool> {
        // Load global config to get TTL
        let global_config = crate::storage::database::load_global_config().await.ok();
        let ttl = global_config
            .and_then(|c| c.scraper)
            .and_then(|s| s.ttl)
            .unwrap_or(3600); // Default 1 hour

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let age = now.saturating_sub(cache.timestamp);

        Ok(age < ttl)
    }

    /// Load cache from disk
    async fn load_cache(&mut self) -> Result<()> {
        let cache_path = self.cache_path()?;

        if !cache_path.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&cache_path)?;
        let cache: UrlCache = serde_json::from_str(&content)?;

        if self.is_cache_valid(&cache).await? {
            log::info!("Using valid cache from: {:?}", cache_path);
            self.cache = Some(cache);
        } else {
            log::info!("Cache expired, will re-scrape");
            // Delete expired cache
            let _ = std::fs::remove_file(&cache_path);
        }

        Ok(())
    }

    /// Save cache to disk
    fn save_cache(&self, urls: Vec<CachedUrl>) -> Result<()> {
        let cache_path = self.cache_path()?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let cache = UrlCache {
            timestamp: now,
            urls,
        };

        let content = serde_json::to_string_pretty(&cache)?;
        std::fs::write(&cache_path, content)?;

        log::info!("Saved cache to: {:?}", cache_path);
        Ok(())
    }

    /// Scrape URLs from webpage
    async fn scrape_urls(&mut self) -> Result<Vec<CachedUrl>> {
        let version_config = self.config.source.version.as_ref()
            .context("version discovery config required for webpage-scraping")?;

        log::info!("Scraping URLs from: {}", version_config.discovery_url);

        // Fetch HTML content
        let html = self.client.get_text(&version_config.discovery_url).await?;

        log::debug!("Received HTML: {} bytes, first 200 chars: {}",
            html.len(),
            &html.chars().take(200).collect::<String>());

        // Extract URLs using regex
        let url_pattern = version_config.url_pattern.as_ref()
            .context("url_pattern required for webpage-scraping")?;

        let url_regex = crate::utils::regex::build_safe_regex(url_pattern)
            .context("Failed to build URL regex")?;

        // Extract version using regex
        let version_pattern = version_config.version_pattern.as_ref()
            .context("version_pattern required for webpage-scraping")?;

        let version_regex = crate::utils::regex::build_safe_regex(version_pattern)
            .context("Failed to build version regex")?;

        // Filter only archive formats (zip, tar.gz, tar.xz, tar.bz2)
        let archive_extensions = [".zip", ".tar.gz", ".tar.xz", ".tar.bz2", ".tgz"];

        let mut cached_urls = Vec::new();
        let url_matches: Vec<_> = url_regex.find_iter(&html).collect();
        log::debug!("Found {} URL regex matches", url_matches.len());
        if !url_matches.is_empty() {
            log::debug!("First URL match sample: '{}'", url_matches[0].as_str());
        }

        for url_match in url_matches {
            let url = url_match.as_str();

            // Filter: only keep archive formats
            if !archive_extensions.iter().any(|ext| url.ends_with(ext)) {
                log::trace!("Skipping non-archive URL: {}", url);
                continue;
            }

            log::debug!("Archive URL found: {}", url);

            // Extract version from URL
            if let Some(version_cap) = version_regex.captures(url) {
                if let Some(version_match) = version_cap.get(1) {
                    let version = version_match.as_str().to_string();

                    // Determine platform from URL
                    // Extract platform identifier (e.g., "linux-x64", "darwin-arm64")
                    let platform = Self::extract_platform_from_url(url);

                    cached_urls.push(CachedUrl {
                        url: url.to_string(),
                        version: version.clone(),
                        platform: platform.clone(),
                    });
                    log::debug!("Added URL: version={}, platform={}", version, platform);
                } else {
                    log::debug!("Version regex matched but no capture group for: {}", url);
                }
            } else {
                log::debug!("Version regex did not match: {}", url);
            }
        }

        if cached_urls.is_empty() {
            anyhow::bail!("No URLs found matching the patterns");
        }

        log::info!("Scraped {} URLs", cached_urls.len());

        // Save to cache
        self.save_cache(cached_urls.clone())?;

        Ok(cached_urls)
    }

    /// Extract platform identifier from URL
    /// Example: "linux-x64", "darwin-arm64", "win32-x64-archive"
    fn extract_platform_from_url(url: &str) -> String {
        // Common patterns: linux-x64, darwin-arm64, win32-x64-archive, etc.
        let platform_patterns = [
            "linux-x64",
            "linux-arm64",
            "darwin-x64",
            "darwin-arm64",
            "win32-x64-archive",
            "win32-arm64-archive",
            "win32-x64",
            "win32-arm64",
        ];

        for pattern in &platform_patterns {
            if url.contains(pattern) {
                return pattern.to_string();
            }
        }

        "unknown".to_string()
    }

    /// Get or scrape URLs
    async fn get_urls(&mut self) -> Result<Vec<CachedUrl>> {
        // Try to load cache first
        self.load_cache().await?;

        if let Some(cache) = &self.cache {
            return Ok(cache.urls.clone());
        }

        // Cache miss or invalid - scrape
        self.scrape_urls().await
    }
}

#[async_trait]
impl VersionProvider for WebpageScrapingProvider {
    async fn list_versions(&self) -> Result<Vec<Version>> {
        // Need mutable self to load cache/scrape
        let mut provider = Self::new(self.config.clone())?;
        let urls = provider.get_urls().await?;

        // Extract unique versions
        let mut version_set: std::collections::HashSet<String> = urls
            .iter()
            .map(|u| u.version.clone())
            .collect();

        let mut versions: Vec<String> = version_set.drain().collect();

        log::debug!("Total unique versions found: {}", versions.len());
        log::debug!("Before sorting (first 5): {:?}", &versions[..versions.len().min(5)]);

        // Sort versions in descending order (newest first) using semantic versioning
        versions.sort_by(|a, b| {
            // Try to parse as semver, fallback to string comparison if parsing fails
            let result = match (semver::Version::parse(a), semver::Version::parse(b)) {
                (Ok(v_a), Ok(v_b)) => {
                    log::trace!("Comparing semver: {} vs {} => {:?}", a, b, v_b.cmp(&v_a));
                    v_b.cmp(&v_a) // Descending order (newest first)
                },
                (Err(_), _) => {
                    log::trace!("Failed to parse '{}' as semver, using string comparison", a);
                    b.cmp(a)
                },
                (_, Err(_)) => {
                    log::trace!("Failed to parse '{}' as semver, using string comparison", b);
                    b.cmp(a)
                },
            };
            result
        });

        log::info!("Sorted versions (first 10): {:?}", &versions[..versions.len().min(10)]);
        log::debug!("Latest version selected: {}", versions.first().unwrap_or(&"none".to_string()));

        // Convert to Version structs
        let version_list: Vec<Version> = versions
            .into_iter()
            .map(|tag| Version {
                tag: tag.clone(),
                name: tag.clone(),
                published_at: String::new(),
                prerelease: tag.contains("alpha") || tag.contains("beta") || tag.contains("rc"),
            })
            .collect();

        Ok(version_list)
    }

    async fn get_download_url(&self, version: &str, os: &str, arch: &str) -> Result<String> {
        // Need mutable self to load cache/scrape
        let mut provider = Self::new(self.config.clone())?;
        let urls = provider.get_urls().await?;

        // Get platform filters from config
        let platform_config = self.config.platform.as_ref()
            .context("platform config with url_filters required for webpage-scraping")?;

        // Build platform key: e.g., "linux_x86_64"
        let platform_key = format!("{}_{}", os, arch);

        // Get the URL filter for this platform
        let url_filter = platform_config.url_filters.get(&platform_key)
            .context(format!("No URL filter found for platform: {}", platform_key))?;

        log::debug!("Looking for version={}, platform={}, filter={}", version, platform_key, url_filter);

        // Find matching URL
        let matching_url = urls.iter()
            .find(|u| u.version == version && u.platform.contains(url_filter))
            .context(format!("No URL found for version={}, platform={}", version, url_filter))?;

        Ok(matching_url.url.clone())
    }

    async fn get_checksum_url(
        &self,
        _version: &str,
        _os: &str,
        _arch: &str,
    ) -> Result<Option<String>> {
        // Webpage scraping typically doesn't have checksums
        Ok(None)
    }

    async fn get_signature_url(
        &self,
        _version: &str,
        _os: &str,
        _arch: &str,
    ) -> Result<Option<String>> {
        // Webpage scraping typically doesn't have signatures
        Ok(None)
    }
}
