use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoConfig {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    pub source: SourceConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformConfig>,
    pub install: InstallConfig,
    pub security: SecurityConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _registry: Option<RegistrySignature>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceConfig {
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<DownloadConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<VersionDiscoveryConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderType {
    GithubReleases,
    GitlabReleases,
    CustomApi,
    DirectUrl,
    WebpageScraping,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DownloadConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstallConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<InstallMode>,
    #[serde(default)]
    pub binaries: Vec<String>,
    #[serde(default)]
    pub files: Vec<AdditionalFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_install: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InstallMode {
    Userland,
    System,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdditionalFile {
    pub src: String,
    pub dst: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecurityConfig {
    #[serde(default)]
    pub allow_insecure: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<ChecksumConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpg: Option<GpgConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<SecurityWarnings>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChecksumConfig {
    pub url: String,
    pub algorithm: ChecksumAlgorithm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename_pattern: Option<String>,
    #[serde(default = "default_checksum_format")]
    pub format: ChecksumFormat,
}

fn default_checksum_format() -> ChecksumFormat {
    ChecksumFormat::MultiHash
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChecksumFormat {
    SingleHash,
    MultiHash,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChecksumAlgorithm {
    Sha256,
    Sha512,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GpgConfig {
    pub signature_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default)]
    pub revoked_keys: Vec<RevokedKey>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RevokedKey {
    pub fingerprint: String,
    pub revoked_at: String,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetadataConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_version: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<SystemDependency>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemDependency {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegistrySignature {
    pub added_at: String,
    pub verified: bool,
    pub verified_by: String,
    pub signature: String,
}

// ========== v2 Schema Additions ==========

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlatformConfig {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub os_map: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub arch_map: HashMap<String, String>,
    /// URL filters for webpage-scraping provider: maps "os_arch" to URL substring
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub url_filters: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionDiscoveryConfig {
    pub discovery_url: String,
    pub discovery_type: DiscoveryType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// Regex pattern to extract full URLs from HTML (for webpage-scraping)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_pattern: Option<String>,
    /// Regex pattern to extract version from URL (for webpage-scraping)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_pattern: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiscoveryType {
    GithubApi,
    GitlabApi,
    Json,
    Text,
    HtmlScraping,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecurityWarnings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

fn default_true() -> bool {
    true
}

// ========== Helper Methods ==========

impl RepoConfig {
    /// Get OS mapping from platform config
    pub fn get_os_map(&self) -> HashMap<String, String> {
        self.platform
            .as_ref()
            .map(|p| p.os_map.clone())
            .unwrap_or_default()
    }

    /// Get arch mapping from platform config
    pub fn get_arch_map(&self) -> HashMap<String, String> {
        self.platform
            .as_ref()
            .map(|p| p.arch_map.clone())
            .unwrap_or_default()
    }
}

impl ChecksumConfig {
    /// Check if this is single hash format
    pub fn is_single_hash(&self) -> bool {
        matches!(self.format, ChecksumFormat::SingleHash)
    }
}
