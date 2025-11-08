use thiserror::Error;

// Allow dead_code for error variants planned for future use
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum OraError {
    #[error("Package not found: {0}")]
    PackageNotFound(String),

    #[error("Version not found: {package}@{version}")]
    VersionNotFound { package: String, version: String },

    #[error("Platform not supported: {os}-{arch}")]
    PlatformNotSupported { os: String, arch: String },

    #[error("Checksum verification failed")]
    ChecksumMismatch,

    #[error("GPG signature verification failed")]
    SignatureInvalid,

    #[error("Certificate fingerprint mismatch (possible MITM attack)")]
    CertificatePinMismatch,

    #[error("Registry not found: {0}")]
    RegistryNotFound(String),

    #[error("Invalid .repo format: {0}")]
    InvalidRepoFormat(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Package is insecure (no checksum verification available)")]
    InsecurePackage,
}
