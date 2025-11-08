use crate::config::repo::{DiscoveryType, VersionDiscoveryConfig};
use crate::utils::http::HttpClient;
use anyhow::{Context, Result};
use serde_json::Value;

/// Version discovery service for custom APIs
pub struct VersionDiscovery {
    config: VersionDiscoveryConfig,
    client: HttpClient,
}

impl VersionDiscovery {
    pub fn new(config: VersionDiscoveryConfig) -> Result<Self> {
        Ok(Self {
            config,
            client: HttpClient::new()?,
        })
    }

    /// Discover available versions from the configured source
    pub async fn discover_versions(&self) -> Result<Vec<String>> {
        log::info!("Discovering versions from: {}", self.config.discovery_url);

        match &self.config.discovery_type {
            DiscoveryType::GithubApi => self.discover_github_api().await,
            DiscoveryType::GitlabApi => self.discover_gitlab_api().await,
            DiscoveryType::Json => self.discover_json().await,
            DiscoveryType::Text => self.discover_text().await,
            DiscoveryType::HtmlScraping => self.discover_html().await,
        }
    }

    /// Discover versions from GitHub API
    async fn discover_github_api(&self) -> Result<Vec<String>> {
        let content = self.client.get_text(&self.config.discovery_url).await?;
        let releases: Vec<Value> =
            serde_json::from_str(&content).context("Failed to parse GitHub API response")?;

        let versions: Vec<String> = releases
            .iter()
            .filter_map(|release| release["tag_name"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(versions)
    }

    /// Discover versions from GitLab API
    async fn discover_gitlab_api(&self) -> Result<Vec<String>> {
        let content = self.client.get_text(&self.config.discovery_url).await?;
        let releases: Vec<Value> =
            serde_json::from_str(&content).context("Failed to parse GitLab API response")?;

        let versions: Vec<String> = releases
            .iter()
            .filter_map(|release| release["tag_name"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(versions)
    }

    /// Discover versions from JSON with JSONPath
    async fn discover_json(&self) -> Result<Vec<String>> {
        let content = self.client.get_text(&self.config.discovery_url).await?;
        let json: Value =
            serde_json::from_str(&content).context("Failed to parse JSON response")?;

        if let Some(json_path) = &self.config.json_path {
            self.extract_from_json(&json, json_path)
        } else {
            anyhow::bail!("json_path is required for JSON discovery type")
        }
    }

    /// Extract versions from JSON using simple path syntax
    /// Supports: $.field, $.array[*], $.nested.field
    fn extract_from_json(&self, json: &Value, path: &str) -> Result<Vec<String>> {
        let path = path.trim_start_matches("$.");
        let parts: Vec<&str> = path.split('.').collect();

        let current = json;
        let mut results = Vec::new();

        Self::traverse_json_path(current, &parts, 0, &mut results)?;

        if results.is_empty() {
            anyhow::bail!("No versions found at path: {}", path);
        }

        Ok(results)
    }

    fn traverse_json_path(
        current: &Value,
        parts: &[&str],
        depth: usize,
        results: &mut Vec<String>,
    ) -> Result<()> {
        if depth >= parts.len() {
            // Reached the end of the path
            if let Some(s) = current.as_str() {
                results.push(s.to_string());
            }
            return Ok(());
        }

        let part = parts[depth];

        if part.ends_with("[*]") {
            // Array traversal
            let field = part.trim_end_matches("[*]");
            let array = if field.is_empty() {
                current
            } else {
                &current[field]
            };

            if let Some(arr) = array.as_array() {
                for item in arr {
                    Self::traverse_json_path(item, parts, depth + 1, results)?;
                }
            }
        } else {
            // Object field access
            if let Some(next) = current.get(part) {
                Self::traverse_json_path(next, parts, depth + 1, results)?;
            }
        }

        Ok(())
    }

    /// Discover versions from plain text using regex
    async fn discover_text(&self) -> Result<Vec<String>> {
        let content = self.client.get_text(&self.config.discovery_url).await?;

        if let Some(pattern) = &self.config.regex {
            // SECURITY: Use safe regex builder with ReDoS protection
            let re = crate::utils::regex::build_safe_regex(pattern)
                .context("Failed to build safe regex")?;

            let versions: Vec<String> = re
                .captures_iter(&content)
                .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                .collect();

            if versions.is_empty() {
                anyhow::bail!("No versions found with regex: {}", pattern);
            }

            Ok(versions)
        } else {
            anyhow::bail!("regex is required for text discovery type")
        }
    }

    /// Discover versions from HTML using scraping
    async fn discover_html(&self) -> Result<Vec<String>> {
        // For HTML scraping, we'll use regex on the HTML content
        // This is a simple implementation - could be enhanced with proper HTML parsing
        let content = self.client.get_text(&self.config.discovery_url).await?;

        if let Some(pattern) = &self.config.regex {
            // SECURITY: Use safe regex builder with ReDoS protection
            let re = crate::utils::regex::build_safe_regex(pattern)
                .context("Failed to build safe regex")?;

            let versions: Vec<String> = re
                .captures_iter(&content)
                .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                .collect();

            if versions.is_empty() {
                anyhow::bail!("No versions found in HTML with regex: {}", pattern);
            }

            Ok(versions)
        } else {
            anyhow::bail!("regex is required for HTML scraping")
        }
    }

    /// Get the latest version from discovered versions
    /// Reserved for future use when explicit latest version fetching is needed.
    #[allow(dead_code)]
    pub async fn get_latest_version(&self) -> Result<String> {
        let versions = self.discover_versions().await?;

        // Return the first version (usually latest in APIs)
        versions.first().cloned().context("No versions available")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_path_simple() {
        let json = serde_json::json!({
            "version": "1.0.0"
        });

        let discovery = VersionDiscovery {
            config: VersionDiscoveryConfig {
                discovery_url: "".to_string(),
                discovery_type: DiscoveryType::Json,
                json_path: Some("$.version".to_string()),
                regex: None,
            },
            client: HttpClient::new().expect("Failed to create HTTP client in test"),
        };

        let result = discovery
            .extract_from_json(&json, "$.version")
            .expect("Failed to extract from JSON in test");
        assert_eq!(result, vec!["1.0.0"]);
    }

    #[test]
    fn test_json_path_array() {
        let json = serde_json::json!({
            "releases": [
                {"version": "1.0.0"},
                {"version": "2.0.0"}
            ]
        });

        let discovery = VersionDiscovery {
            config: VersionDiscoveryConfig {
                discovery_url: "".to_string(),
                discovery_type: DiscoveryType::Json,
                json_path: Some("$.releases[*].version".to_string()),
                regex: None,
            },
            client: HttpClient::new().expect("Failed to create HTTP client in test"),
        };

        let result = discovery
            .extract_from_json(&json, "$.releases[*].version")
            .expect("Failed to extract from JSON in test");
        assert_eq!(result, vec!["1.0.0", "2.0.0"]);
    }
}
