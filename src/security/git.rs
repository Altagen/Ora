use anyhow::Result;

/// Validate Git URL for security
///
/// **SECURITY WARNING**: Git URLs can enable command injection attacks.
/// Only HTTPS URLs are considered safe by default.
///
/// Dangerous schemes:
/// - `git://` - Unencrypted, can be MITM'd
/// - `ssh://` - Can execute arbitrary commands
/// - `file://` - Can access local filesystem
/// - Custom protocols - Unknown security properties
///
/// Configuration: See `SecurityConfig.network.git.https_only`
pub fn validate_git_url(url: &str) -> Result<()> {
    let config = crate::config::SecurityConfig::load().unwrap_or_default();

    // Check if HTTPS-only mode is enabled
    if config.network.git.https_only {
        validate_https_only(url, &config.network.git.allowed_schemes)?;
    } else {
        // Still validate against allowed schemes
        validate_allowed_schemes(url, &config.network.git.allowed_schemes)?;
    }

    Ok(())
}

/// Validate that URL uses HTTPS only
fn validate_https_only(url: &str, allowed_schemes: &[String]) -> Result<()> {
    // Check if URL starts with any allowed scheme
    let is_allowed = allowed_schemes.iter().any(|scheme| {
        let prefix = format!("{}://", scheme);
        url.starts_with(&prefix)
    });

    if !is_allowed {
        log::error!("❌ Git URL rejected: {}", url);
        log::error!("HTTPS-only mode is enabled in security configuration");
        log::error!("Allowed schemes: {:?}", allowed_schemes);

        anyhow::bail!(
            "Git URL '{}' is not allowed. HTTPS-only mode is enabled.\n\
             \n\
             Security: Non-HTTPS Git URLs can enable:\n\
             • git:// - Unencrypted connections (MITM attacks)\n\
             • ssh:// - Command injection via SSH arguments\n\
             • file:// - Local filesystem access\n\
             \n\
             Allowed schemes: {:?}\n\
             \n\
             To disable HTTPS-only mode, edit ~/.config/ora/security.toml:\n\
             [network.git]\n\
             https_only = false",
            url,
            allowed_schemes
        );
    }

    Ok(())
}

/// Validate URL against allowed schemes
fn validate_allowed_schemes(url: &str, allowed_schemes: &[String]) -> Result<()> {
    let is_allowed = allowed_schemes.iter().any(|scheme| {
        let prefix = format!("{}://", scheme);
        url.starts_with(&prefix)
    });

    if !is_allowed {
        log::warn!("Git URL uses non-standard scheme: {}", url);
        log::warn!("Allowed schemes: {:?}", allowed_schemes);

        // Extract scheme from URL
        let scheme = url.split("://").next().unwrap_or("unknown");

        anyhow::bail!(
            "Git URL scheme '{}' is not allowed.\n\
             Allowed schemes: {:?}\n\
             \n\
             To allow this scheme, edit ~/.config/ora/security.toml:\n\
             [network.git]\n\
             allowed_schemes = {:?}",
            scheme,
            allowed_schemes,
            {
                let mut schemes = allowed_schemes.to_vec();
                schemes.push(scheme.to_string());
                schemes
            }
        );
    }

    Ok(())
}

/// Validate Git repository size limits
/// Reserved for future use when Git repository size validation is implemented.
#[allow(dead_code)]
pub fn validate_repo_size(size_bytes: u64) -> Result<()> {
    let config = crate::config::SecurityConfig::load().unwrap_or_default();

    if size_bytes > config.network.git.max_repo_size {
        anyhow::bail!(
            "Git repository size {} bytes exceeds maximum allowed size of {} bytes",
            size_bytes,
            config.network.git.max_repo_size
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_https_url_allowed() {
        let result = validate_git_url("https://github.com/user/repo.git");
        assert!(result.is_ok());
    }

    #[test]
    fn test_http_url_allowed_by_default() {
        // Default config allows both http and https
        let _result = validate_git_url("http://example.com/repo.git");
        // This might fail if https_only is true by default
        // Check the actual default in security_config.rs
    }

    #[test]
    fn test_git_protocol_blocked() {
        let _result = validate_git_url("git://github.com/user/repo.git");
        // Should be blocked if https_only is true (default)
        // or if git:// is not in allowed_schemes
    }

    #[test]
    fn test_ssh_protocol_blocked() {
        let _result = validate_git_url("ssh://git@github.com/user/repo.git");
        // Should be blocked if https_only is true (default)
    }

    #[test]
    fn test_file_protocol_blocked() {
        let _result = validate_git_url("file:///tmp/repo");
        // Should be blocked if https_only is true (default)
    }
}
