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
    log::info!("Verifying checksum for {:?}", file_path);

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

    log::info!("Checksum verified successfully");
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
