/// Security configuration for Ora package manager
///
/// All security limits and policies are configurable via config file or environment variables.
/// This allows production deployments to tune security based on their threat model.
use serde::{Deserialize, Serialize};

/// Complete security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct SecurityConfig {
    /// Network security settings
    pub network: NetworkSecurityConfig,

    /// File extraction limits
    pub extraction: ExtractionSecurityConfig,

    /// Script execution policies
    pub scripts: ScriptSecurityConfig,

    /// Registry trust policies
    pub registries: RegistrySecurityConfig,

    /// Input validation rules
    pub validation: ValidationSecurityConfig,

    /// Resource limits
    pub resources: ResourceLimitConfig,
}

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkSecurityConfig {
    /// Allow only HTTPS URLs for downloads (recommended: true)
    pub https_only: bool,

    /// Follow HTTP redirects (recommended: false for security, true for compatibility)
    pub allow_redirects: bool,

    /// Maximum number of redirects to follow (if allowed)
    pub max_redirects: usize,

    /// Block access to private IP ranges (RFC 1918, RFC 4193)
    pub block_private_ips: bool,

    /// Block access to localhost/loopback
    pub block_localhost: bool,

    /// Block access to link-local addresses (169.254.x.x, fe80::)
    pub block_link_local: bool,

    /// Block access to cloud metadata endpoints
    pub block_metadata_endpoints: bool,

    /// Allowed URL schemes (default: ["https", "http"])
    pub allowed_schemes: Vec<String>,

    /// Maximum download size in bytes (2 GB default)
    pub max_download_size: u64,

    /// Network timeout in seconds
    pub timeout_seconds: u64,

    /// Validate DNS resolution before requests (prevents rebinding)
    pub validate_dns_resolution: bool,

    /// Git protocol restrictions
    pub git: GitSecurityConfig,
}

/// Git-specific security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GitSecurityConfig {
    /// Allow only HTTPS for Git operations (recommended: true)
    pub https_only: bool,

    /// Allowed Git URL schemes (default: ["https"])
    pub allowed_schemes: Vec<String>,

    /// Maximum repository size in bytes (100 MB default)
    pub max_repo_size: u64,

    /// Clone/fetch timeout in seconds
    pub timeout_seconds: u64,

    /// Use force during checkout (recommended: false)
    pub allow_force_checkout: bool,
}

/// File extraction security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtractionSecurityConfig {
    /// Maximum size of a single extracted file (1 GB default)
    pub max_file_size: u64,

    /// Maximum total size of all extracted files (5 GB default)
    pub max_total_size: u64,

    /// Maximum number of files in an archive (100,000 default)
    pub max_file_count: usize,

    /// Maximum directory depth (50 levels default)
    pub max_directory_depth: usize,

    /// Maximum path length (4096 bytes default)
    pub max_path_length: usize,

    /// Compression ratio warning threshold (100:1 default)
    pub compression_ratio_warning: u64,

    /// Block symlinks in archives (recommended: true)
    pub block_symlinks: bool,

    /// Block hardlinks in archives (recommended: true)
    pub block_hardlinks: bool,

    /// Block device files in archives (recommended: true)
    pub block_device_files: bool,

    /// Strip SUID/SGID bits (recommended: true)
    pub strip_setuid_bits: bool,
}

/// Script execution security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ScriptSecurityConfig {
    /// Require user confirmation for post-install scripts (recommended: true)
    pub require_confirmation: bool,

    /// Enable post-install scripts at all (recommended: true with confirmation)
    pub enabled: bool,

    /// Maximum script execution time in seconds (300 = 5 minutes default)
    pub timeout_seconds: u64,

    /// Show script content before execution (recommended: true)
    pub show_script_content: bool,

    /// Perform static analysis on scripts (warns about dangerous patterns)
    pub static_analysis: bool,

    /// Block scripts from public registries without explicit flag (recommended: true)
    pub block_public_registry_scripts: bool,

    /// Whitelist of allowed script interpreters (default: ["sh", "bash"])
    pub allowed_interpreters: Vec<String>,

    /// Environment variable filtering
    pub filter_sensitive_env_vars: bool,
}

/// Registry trust and validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RegistrySecurityConfig {
    /// Enforce trust levels (public vs private registries)
    pub enforce_trust_levels: bool,

    /// Require checksums for packages from public registries
    pub require_checksums_public: bool,

    /// Require checksums for packages from private registries
    pub require_checksums_private: bool,

    /// Require GPG signatures (when implemented)
    pub require_gpg_signatures: bool,

    /// Allow multiple registries with same package (recommended: false)
    pub allow_package_shadowing: bool,

    /// Fail on package found in multiple registries
    pub fail_on_ambiguous_package: bool,

    /// Maximum registry size for sync operations (100 MB default)
    pub max_registry_size: u64,

    /// Registry sync timeout in seconds
    pub sync_timeout_seconds: u64,
}

/// Input validation security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidationSecurityConfig {
    /// Maximum TOML file size in bytes (1 MB default)
    pub max_toml_size: u64,

    /// Maximum JSON response size in bytes (10 MB default)
    pub max_json_size: u64,

    /// Regex complexity limits
    pub regex: RegexValidationConfig,

    /// Template variable validation
    pub templates: TemplateValidationConfig,
}

/// Regex validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RegexValidationConfig {
    /// Maximum compiled regex size in bytes (1 MB default)
    pub max_compiled_size: usize,

    /// Maximum DFA size in bytes (1 MB default)
    pub max_dfa_size: usize,

    /// Maximum number of capture groups
    pub max_capture_groups: usize,

    /// Maximum pattern length
    pub max_pattern_length: usize,

    /// Maximum number of matches to process
    pub max_matches: usize,
}

/// Template validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TemplateValidationConfig {
    /// URL-encode template variables (recommended: true)
    pub url_encode_variables: bool,

    /// Block path traversal sequences in variables (../)
    pub block_path_traversal: bool,

    /// Block null bytes in variables
    pub block_null_bytes: bool,

    /// Block newlines in variables
    pub block_newlines: bool,

    /// Maximum variable value length
    pub max_variable_length: usize,
}

/// Resource limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ResourceLimitConfig {
    /// Enable resource limits (recommended: true)
    pub enabled: bool,

    /// Maximum concurrent downloads
    pub max_concurrent_downloads: usize,

    /// Maximum memory for operations (in bytes, 0 = unlimited)
    pub max_memory_bytes: u64,

    /// Maximum disk space for cache (in bytes, 0 = unlimited)
    pub max_cache_size_bytes: u64,
}

// ============================================================================
// DEFAULT IMPLEMENTATIONS - SECURE DEFAULTS
// ============================================================================

impl Default for NetworkSecurityConfig {
    fn default() -> Self {
        Self {
            https_only: false,      // Allow HTTP for compatibility, but warn
            allow_redirects: false, // Secure default: no redirects
            max_redirects: 3,
            block_private_ips: true,
            block_localhost: true,
            block_link_local: true,
            block_metadata_endpoints: true,
            allowed_schemes: vec!["https".to_string(), "http".to_string()],
            max_download_size: 2 * 1024 * 1024 * 1024, // 2 GB
            timeout_seconds: 300,                      // 5 minutes
            validate_dns_resolution: true,             // Prevent DNS rebinding
            git: GitSecurityConfig::default(),
        }
    }
}

impl Default for GitSecurityConfig {
    fn default() -> Self {
        Self {
            https_only: true, // SECURE: Only HTTPS for Git
            allowed_schemes: vec!["https".to_string()],
            max_repo_size: 100 * 1024 * 1024, // 100 MB
            timeout_seconds: 300,             // 5 minutes
            allow_force_checkout: false,      // Safe default
        }
    }
}

impl Default for ExtractionSecurityConfig {
    fn default() -> Self {
        Self {
            max_file_size: 1024 * 1024 * 1024,      // 1 GB
            max_total_size: 5 * 1024 * 1024 * 1024, // 5 GB
            max_file_count: 100_000,
            max_directory_depth: 50,
            max_path_length: 4096,
            compression_ratio_warning: 100,
            block_symlinks: true,
            block_hardlinks: true,
            block_device_files: true,
            strip_setuid_bits: true,
        }
    }
}

impl Default for ScriptSecurityConfig {
    fn default() -> Self {
        Self {
            require_confirmation: true, // SECURE: Always require confirmation
            enabled: true,
            timeout_seconds: 300, // 5 minutes
            show_script_content: true,
            static_analysis: true,
            block_public_registry_scripts: true, // SECURE: Block by default
            allowed_interpreters: vec!["sh".to_string(), "bash".to_string()],
            filter_sensitive_env_vars: true,
        }
    }
}

impl Default for RegistrySecurityConfig {
    fn default() -> Self {
        Self {
            enforce_trust_levels: true,
            require_checksums_public: true, // SECURE: Require checksums
            require_checksums_private: false, // More lenient for private
            require_gpg_signatures: false,  // Not implemented yet
            allow_package_shadowing: false, // SECURE: Fail on ambiguity
            fail_on_ambiguous_package: true,
            max_registry_size: 100 * 1024 * 1024, // 100 MB
            sync_timeout_seconds: 300,
        }
    }
}

impl Default for ValidationSecurityConfig {
    fn default() -> Self {
        Self {
            max_toml_size: 1024 * 1024,      // 1 MB
            max_json_size: 10 * 1024 * 1024, // 10 MB
            regex: RegexValidationConfig::default(),
            templates: TemplateValidationConfig::default(),
        }
    }
}

impl Default for RegexValidationConfig {
    fn default() -> Self {
        Self {
            max_compiled_size: 1024 * 1024, // 1 MB
            max_dfa_size: 1024 * 1024,      // 1 MB
            max_capture_groups: 50,
            max_pattern_length: 1000,
            max_matches: 1000,
        }
    }
}

impl Default for TemplateValidationConfig {
    fn default() -> Self {
        Self {
            url_encode_variables: true, // SECURE: Always encode
            block_path_traversal: true,
            block_null_bytes: true,
            block_newlines: true,
            max_variable_length: 1024,
        }
    }
}

impl Default for ResourceLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_concurrent_downloads: 3,
            max_memory_bytes: 0,                           // Unlimited by default
            max_cache_size_bytes: 10 * 1024 * 1024 * 1024, // 10 GB
        }
    }
}

// ============================================================================
// HELPER METHODS
// ============================================================================

impl SecurityConfig {
    /// Load security config from file or use defaults
    ///
    /// **UX IMPROVEMENT**: This function now provides clear feedback when:
    /// - Config file doesn't exist (uses defaults, logs info)
    /// - Config file has permission errors (returns error with fix suggestions)
    /// - Config file is malformed (returns error with line number)
    pub fn load() -> anyhow::Result<Self> {
        use crate::utils::io_errors::read_file_user_friendly;

        // Try to get config directory
        let config_path = match crate::storage::paths::Paths::config_dir() {
            Ok(path) => path,
            Err(e) => {
                log::warn!("Could not determine config directory: {}", e);
                log::debug!("Using default security configuration");
                return Ok(Self::default());
            }
        };

        let security_config_path = config_path.join("security.toml");

        // Try to read config file with user-friendly error messages
        match read_file_user_friendly(&security_config_path)? {
            Some(content) => {
                // File exists and was read successfully - parse it
                match toml::from_str::<SecurityConfig>(&content) {
                    Ok(config) => {
                        log::info!(
                            "✅ Loaded security configuration from {}",
                            security_config_path.display()
                        );
                        Ok(config)
                    }
                    Err(e) => {
                        anyhow::bail!(
                            "Failed to parse security configuration: {}\n\
                             \n\
                             File: {}\n\
                             Error: {}\n\
                             \n\
                             Fix this by:\n\
                             ├─ Checking TOML syntax: https://toml.io\n\
                             ├─ Running: ora security verify\n\
                             └─ Resetting to defaults: ora security reset",
                            security_config_path.display(),
                            security_config_path.display(),
                            e
                        );
                    }
                }
            }
            None => {
                // File doesn't exist - use defaults
                log::info!(
                    "No security configuration found at {}",
                    security_config_path.display()
                );
                log::debug!("Using default security configuration (production-ready defaults)");
                log::debug!("To customize, run: ora security init");
                Ok(Self::default())
            }
        }
    }

    /// Save security config to file with user-friendly error messages
    pub fn save(&self) -> anyhow::Result<()> {
        use crate::utils::io_errors::{create_dir_all_user_friendly, write_file_user_friendly};
        use anyhow::Context;

        let config_path = crate::storage::paths::Paths::config_dir()?;

        // Create config directory with user-friendly errors
        create_dir_all_user_friendly(&config_path)?;

        let security_config_path = config_path.join("security.toml");
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize security configuration to TOML")?;

        // Write config file with user-friendly errors
        write_file_user_friendly(&security_config_path, &content)?;

        log::info!(
            "✅ Saved security configuration to {}",
            security_config_path.display()
        );
        Ok(())
    }

    /// Generate example configuration file with all options documented
    /// Reserved for future use when example config generation is needed.
    #[allow(dead_code)]
    pub fn generate_example() -> String {
        format!(
            r#"# Ora Security Configuration
# This file controls all security policies for the package manager.
# All values shown are the secure defaults.

[network]
# Allow only HTTPS for downloads (set false for HTTP compatibility)
https_only = false

# Follow HTTP redirects (false = more secure, true = more compatible)
allow_redirects = false
max_redirects = 3

# SSRF Protection
block_private_ips = true
block_localhost = true
block_link_local = true
block_metadata_endpoints = true

# Allowed URL schemes
allowed_schemes = ["https", "http"]

# Download limits
max_download_size = {} # 2 GB
timeout_seconds = 300  # 5 minutes

# Prevent DNS rebinding attacks
validate_dns_resolution = true

[network.git]
# Git repository security
https_only = true
allowed_schemes = ["https"]
max_repo_size = {} # 100 MB
timeout_seconds = 300
allow_force_checkout = false

[extraction]
# File extraction limits (anti-zip-bomb)
max_file_size = {}      # 1 GB
max_total_size = {}     # 5 GB
max_file_count = 100000
max_directory_depth = 50
max_path_length = 4096
compression_ratio_warning = 100

# Block dangerous file types
block_symlinks = true
block_hardlinks = true
block_device_files = true
strip_setuid_bits = true

[scripts]
# Post-install script security
require_confirmation = true
enabled = true
timeout_seconds = 300  # 5 minutes
show_script_content = true
static_analysis = true
block_public_registry_scripts = true
allowed_interpreters = ["sh", "bash"]
filter_sensitive_env_vars = true

[registries]
# Registry trust and validation
enforce_trust_levels = true
require_checksums_public = true
require_checksums_private = false
require_gpg_signatures = false  # Not implemented yet
allow_package_shadowing = false
fail_on_ambiguous_package = true
max_registry_size = {}  # 100 MB
sync_timeout_seconds = 300

[validation]
# Input validation limits
max_toml_size = {}   # 1 MB
max_json_size = {}   # 10 MB

[validation.regex]
# Regex DoS prevention
max_compiled_size = {}  # 1 MB
max_dfa_size = {}       # 1 MB
max_capture_groups = 50
max_pattern_length = 1000
max_matches = 1000

[validation.templates]
# Template injection prevention
url_encode_variables = true
block_path_traversal = true
block_null_bytes = true
block_newlines = true
max_variable_length = 1024

[resources]
# Resource limits
enabled = true
max_concurrent_downloads = 3
max_memory_bytes = 0  # 0 = unlimited
max_cache_size_bytes = {}  # 10 GB
"#,
            2 * 1024 * 1024 * 1024u64,  // max_download_size
            100 * 1024 * 1024u64,       // git max_repo_size
            1024 * 1024 * 1024u64,      // max_file_size
            5 * 1024 * 1024 * 1024u64,  // max_total_size
            100 * 1024 * 1024u64,       // max_registry_size
            1024 * 1024u64,             // max_toml_size
            10 * 1024 * 1024u64,        // max_json_size
            1024 * 1024usize,           // regex max_compiled_size
            1024 * 1024usize,           // regex max_dfa_size
            10 * 1024 * 1024 * 1024u64, // max_cache_size
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_secure() {
        let config = SecurityConfig::default();

        // Network security
        assert!(config.network.block_private_ips);
        assert!(config.network.block_localhost);
        assert!(config.network.validate_dns_resolution);
        assert!(config.network.git.https_only);

        // Extraction security
        assert!(config.extraction.block_symlinks);
        assert!(config.extraction.strip_setuid_bits);

        // Script security
        assert!(config.scripts.require_confirmation);
        assert!(config.scripts.show_script_content);
        assert!(config.scripts.block_public_registry_scripts);

        // Registry security
        assert!(config.registries.enforce_trust_levels);
        assert!(config.registries.require_checksums_public);
        assert!(!config.registries.allow_package_shadowing);

        // Validation
        assert!(config.validation.templates.url_encode_variables);
        assert!(config.validation.templates.block_path_traversal);
    }

    #[test]
    fn test_serialization() {
        let config = SecurityConfig::default();
        let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize config to TOML");
        let deserialized: SecurityConfig =
            toml::from_str(&toml_str).expect("Failed to deserialize config from TOML");

        assert_eq!(config.network.https_only, deserialized.network.https_only);
        assert_eq!(
            config.scripts.timeout_seconds,
            deserialized.scripts.timeout_seconds
        );
    }
}
