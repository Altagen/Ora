use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GlobalConfig {
    /// Config schema version (follows Ora major version)
    /// Format: "x.y" where x = Ora major, y = config changes
    #[serde(default = "default_config_version")]
    pub config_version: String,

    #[serde(default)]
    pub registries: Vec<Registry>,
    #[serde(default)]
    pub install: InstallSettings,
    #[serde(default)]
    pub security: SecuritySettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_insecure_warnings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper: Option<ScraperSettings>,

    /// Package aliases (e.g., "k" -> "kubectl")
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub aliases: std::collections::HashMap<String, String>,
}

fn default_config_version() -> String {
    "0.1".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Registry {
    pub name: String,
    pub url: String,
    #[serde(default = "default_trust_level")]
    pub trust_level: TrustLevel,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpg_key: Option<String>,
    /// Optional Git branch to use for this registry (defaults to repository's default branch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Optional registry directory name (defaults to "ora-registry")
    /// This is the directory within the Git repository that contains .repo files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_dir: Option<String>,

    /// Registry priority for conflict resolution (lower = higher priority)
    /// Planned for v0.2.3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
}

impl Registry {
    /// Get the registry directory name, with fallback to "ora-registry"
    pub fn get_registry_dir(&self) -> &str {
        self.registry_dir.as_deref().unwrap_or("ora-registry")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TrustLevel {
    Public,
    Private,
}

fn default_trust_level() -> TrustLevel {
    TrustLevel::Public
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TlsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_cert: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_cert_inline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_pin: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstallSettings {
    #[serde(default = "default_install_mode")]
    pub default_mode: InstallMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userland_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_dir: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InstallMode {
    Userland,
    System,
}

fn default_install_mode() -> InstallMode {
    InstallMode::Userland
}

impl Default for InstallSettings {
    fn default() -> Self {
        Self {
            default_mode: InstallMode::Userland,
            userland_dir: None,
            system_dir: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecuritySettings {
    #[serde(default = "default_require_checksums")]
    pub require_checksums: bool,
    #[serde(default)]
    pub require_signatures: bool,
    /// Maximum allowed git repository size in MB (default: 1024 MB = 1 GB)
    #[serde(default = "default_max_git_size_mb")]
    pub max_git_size_mb: u64,
}

fn default_require_checksums() -> bool {
    true
}

fn default_max_git_size_mb() -> u64 {
    1024 // 1 GB default limit
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            require_checksums: true,
            require_signatures: false,
            max_git_size_mb: 1024,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScraperSettings {
    /// Cache TTL in seconds for webpage-scraping provider (default: 3600 = 1 hour)
    #[serde(default = "default_scraper_ttl")]
    pub ttl: Option<u64>,
}

fn default_scraper_ttl() -> Option<u64> {
    Some(3600) // 1 hour
}
