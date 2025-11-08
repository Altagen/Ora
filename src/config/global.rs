use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GlobalConfig {
    #[serde(default)]
    pub registries: Vec<Registry>,
    #[serde(default)]
    pub install: InstallSettings,
    #[serde(default)]
    pub security: SecuritySettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_insecure_warnings: Option<Vec<String>>,
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
}

fn default_require_checksums() -> bool {
    true
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            require_checksums: true,
            require_signatures: false,
        }
    }
}
