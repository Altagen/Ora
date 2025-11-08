use anyhow::Result;
use std::path::Path;

/// Verify GPG signature of a file
///
/// **SECURITY WARNING**: GPG verification is not yet implemented.
/// This function will FAIL by default to prevent false sense of security.
///
/// To bypass (NOT RECOMMENDED), use the `--allow-insecure` flag or set
/// `security.registries.require_gpg_signatures = false` in config.
pub async fn verify_signature(
    file_path: &Path,
    _signature_path: &Path,
    _public_key: Option<&str>,
) -> Result<()> {
    // SECURITY: Fail loudly instead of silently succeeding
    log::error!("❌ GPG signature verification is NOT IMPLEMENTED");
    log::error!("File: {:?}", file_path);
    log::error!("GPG verification cannot protect against tampered packages.");

    anyhow::bail!(
        "GPG signature verification is not yet implemented. \
         This is a critical security feature that prevents package tampering. \
         \n\nOptions:\
         \n  1. Wait for GPG implementation (recommended)\
         \n  2. Use checksum-only verification (less secure)\
         \n  3. Use --allow-insecure flag (NOT RECOMMENDED for production)\
         \n\nTo disable this check globally, set in ~/.config/ora/security.toml:\
         \n  [registries]\
         \n  require_gpg_signatures = false"
    );
}

/// Import a GPG public key
///
/// **NOT IMPLEMENTED**: This function will fail.
/// Reserved for future use when GPG verification is fully implemented.
#[allow(dead_code)]
pub async fn import_public_key(_key_data: &str) -> Result<()> {
    anyhow::bail!(
        "GPG public key import is not yet implemented. \
         Use checksum verification instead."
    );
}

/// Check if a GPG key fingerprint is revoked
///
/// **NOT IMPLEMENTED**: Always returns false (assumes key is valid)
/// Reserved for future use when GPG verification is fully implemented.
#[allow(dead_code)]
pub fn check_key_revoked(_fingerprint: &str, _revoked_keys: &[String]) -> bool {
    log::warn!("GPG key revocation check not implemented, assuming key is valid");
    false
}
