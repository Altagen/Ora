use anyhow::{Context, Result};
use sha2::{Digest, Sha256, Sha512};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::config::repo::ChecksumAlgorithm;
use crate::error::OraError;

pub async fn verify_checksum(
    file_path: &Path,
    expected_hash: &str,
    algorithm: &ChecksumAlgorithm,
) -> Result<()> {
    log::debug!("Verifying checksum for {:?}", file_path);

    let computed_hash = compute_checksum(file_path, algorithm).await?;
    let expected_hash = expected_hash.trim().to_lowercase();

    if computed_hash != expected_hash {
        log::error!(
            "Checksum mismatch! Expected: {}, Got: {}",
            expected_hash,
            computed_hash
        );
        return Err(OraError::ChecksumMismatch.into());
    }

    log::debug!("Checksum verified successfully");
    Ok(())
}

pub async fn compute_checksum(file_path: &Path, algorithm: &ChecksumAlgorithm) -> Result<String> {
    let mut file = File::open(file_path)
        .await
        .context("Failed to open file for checksum")?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .context("Failed to read file for checksum")?;

    let hash = match algorithm {
        ChecksumAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
        ChecksumAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
    };

    Ok(hash)
}

pub fn parse_checksum_file(content: &str, filename: &str) -> Option<String> {
    // Parse checksums in format: "hash  filename" or "hash *filename"
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            // Use safe indexing with .get() to prevent potential panics
            let hash = match parts.first() {
                Some(h) => *h,
                None => continue, // Skip malformed line
            };
            let file = match parts.get(1) {
                Some(f) => f.trim_start_matches('*'),
                None => continue, // Skip malformed line
            };

            if file == filename || file.ends_with(filename) {
                return Some(hash.to_string());
            }
        }
    }

    None
}

/// Parses a single-hash checksum file that may contain "hash  filename" format
/// Returns only the hash portion, stripping any filename suffix
#[cfg(test)]
pub fn parse_single_hash(content: &str) -> Option<String> {
    content.split_whitespace().next().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_checksum_file_standard_format() {
        let content = "abc123def456  example.tar.gz\n789ghi012jkl  another.zip";
        assert_eq!(
            parse_checksum_file(content, "example.tar.gz"),
            Some("abc123def456".to_string())
        );
        assert_eq!(
            parse_checksum_file(content, "another.zip"),
            Some("789ghi012jkl".to_string())
        );
    }

    #[test]
    fn test_parse_checksum_file_with_asterisk() {
        let content = "abc123def456 *example.tar.gz";
        assert_eq!(
            parse_checksum_file(content, "example.tar.gz"),
            Some("abc123def456".to_string())
        );
    }

    #[test]
    fn test_parse_checksum_file_not_found() {
        let content = "abc123def456  example.tar.gz";
        assert_eq!(parse_checksum_file(content, "nonexistent.zip"), None);
    }

    #[test]
    fn test_parse_checksum_file_with_comments() {
        let content = "# This is a comment\nabc123def456  example.tar.gz\n# Another comment";
        assert_eq!(
            parse_checksum_file(content, "example.tar.gz"),
            Some("abc123def456".to_string())
        );
    }

    #[test]
    fn test_parse_single_hash_with_filename() {
        // BUG-1: Test parsing of single-hash format with filename suffix
        let content = "1c9297be4a084eea7ecaedf93eb03d058d6faae29bbc57ecdaf5063921491599  ripgrep-15.1.0-x86_64-unknown-linux-musl.tar.gz";
        assert_eq!(
            parse_single_hash(content),
            Some("1c9297be4a084eea7ecaedf93eb03d058d6faae29bbc57ecdaf5063921491599".to_string())
        );
    }

    #[test]
    fn test_parse_single_hash_without_filename() {
        let content = "abc123def456789";
        assert_eq!(
            parse_single_hash(content),
            Some("abc123def456789".to_string())
        );
    }

    #[test]
    fn test_parse_single_hash_with_whitespace() {
        let content = "  abc123def456789  \n";
        assert_eq!(
            parse_single_hash(content),
            Some("abc123def456789".to_string())
        );
    }

    #[test]
    fn test_parse_single_hash_empty() {
        let content = "";
        assert_eq!(parse_single_hash(content), None);
    }

    #[test]
    fn test_parse_single_hash_whitespace_only() {
        let content = "   \n  \t  ";
        assert_eq!(parse_single_hash(content), None);
    }
}
