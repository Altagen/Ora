use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tar::Archive;
use xz2::read::XzDecoder;

use crate::config::security_limits::*;

pub struct Extractor;

/// Tracks extraction statistics for security limits
struct ExtractionStats {
    total_bytes_extracted: u64,
    files_extracted: usize,
}

impl Extractor {
    pub fn extract(archive_path: &Path, dest_dir: &Path) -> Result<()> {
        log::info!("Extracting {:?} to {:?}", archive_path, dest_dir);

        // Clean extraction directory if it exists to prevent accumulation of old extracts
        if dest_dir.exists() {
            log::debug!("Removing old extraction directory: {:?}", dest_dir);
            std::fs::remove_dir_all(dest_dir)
                .context("Failed to remove old extraction directory")?;
        }

        // Ensure destination directory exists
        std::fs::create_dir_all(dest_dir)?;

        // Determine archive type by extension
        let path_str = archive_path.to_string_lossy();

        if path_str.ends_with(".tar.gz") || path_str.ends_with(".tgz") {
            Self::extract_tar_gz(archive_path, dest_dir)?;
        } else if path_str.ends_with(".tar.xz") || path_str.ends_with(".txz") {
            Self::extract_tar_xz(archive_path, dest_dir)?;
        } else if path_str.ends_with(".zip") {
            Self::extract_zip(archive_path, dest_dir)?;
        } else if path_str.ends_with(".tar") {
            Self::extract_tar(archive_path, dest_dir)?;
        } else {
            anyhow::bail!("Unsupported archive format: {}", path_str);
        }

        log::info!("Extraction completed");
        Ok(())
    }

    /// Checks extraction limits and updates statistics
    fn check_extraction_limits(stats: &mut ExtractionStats, file_size: u64) -> Result<()> {
        // Check individual file size
        if file_size > MAX_EXTRACTED_FILE_SIZE {
            anyhow::bail!(
                "File size ({} bytes) exceeds maximum allowed size ({} bytes). Possible zip bomb.",
                file_size,
                MAX_EXTRACTED_FILE_SIZE
            );
        }

        // Check total extracted size
        if stats.total_bytes_extracted + file_size > MAX_TOTAL_EXTRACTED_SIZE {
            anyhow::bail!(
                "Total extracted size ({} bytes) would exceed maximum ({} bytes). Possible zip bomb.",
                stats.total_bytes_extracted + file_size,
                MAX_TOTAL_EXTRACTED_SIZE
            );
        }

        // Check number of files
        if stats.files_extracted >= MAX_FILES_IN_ARCHIVE {
            anyhow::bail!(
                "Number of files ({}) exceeds maximum ({}). Possible billion laughs attack.",
                stats.files_extracted,
                MAX_FILES_IN_ARCHIVE
            );
        }

        // Update stats
        stats.total_bytes_extracted += file_size;
        stats.files_extracted += 1;

        Ok(())
    }

    /// Validates that a path doesn't escape the destination directory
    /// Returns the safe path if valid, or error if path traversal detected
    fn validate_extraction_path(dest_dir: &Path, entry_path: &Path) -> Result<PathBuf> {
        // Check path length
        if entry_path.as_os_str().len() > MAX_PATH_LENGTH {
            anyhow::bail!(
                "Path length ({}) exceeds maximum ({}): {}",
                entry_path.as_os_str().len(),
                MAX_PATH_LENGTH,
                entry_path.display()
            );
        }

        // Check directory depth
        let depth = entry_path.components().count();
        if depth > MAX_DIRECTORY_DEPTH {
            anyhow::bail!(
                "Directory depth ({}) exceeds maximum ({}): {}",
                depth,
                MAX_DIRECTORY_DEPTH,
                entry_path.display()
            );
        }
        // Get canonical destination path
        let canonical_dest = dest_dir
            .canonicalize()
            .context("Failed to canonicalize destination directory")?;

        // Build the full output path
        let full_path = dest_dir.join(entry_path);

        // For security, check if any component is ".." or "."
        for component in entry_path.components() {
            match component {
                std::path::Component::ParentDir => {
                    anyhow::bail!(
                        "Path traversal detected: archive contains '..' in path: {}",
                        entry_path.display()
                    );
                }
                std::path::Component::Normal(_) => {
                    // Normal component, continue
                }
                std::path::Component::RootDir => {
                    anyhow::bail!(
                        "Absolute path detected: archive contains absolute path: {}",
                        entry_path.display()
                    );
                }
                std::path::Component::CurDir => {
                    // Current dir is fine
                }
                std::path::Component::Prefix(_) => {
                    #[cfg(windows)]
                    anyhow::bail!(
                        "Windows path prefix detected in archive: {}",
                        entry_path.display()
                    );
                    #[cfg(not(windows))]
                    anyhow::bail!("Unexpected path prefix: {}", entry_path.display());
                }
            }
        }

        // If the path exists, canonicalize and verify it's within dest_dir
        if full_path.exists() {
            let canonical_path = full_path
                .canonicalize()
                .context("Failed to canonicalize extracted path")?;

            if !canonical_path.starts_with(&canonical_dest) {
                anyhow::bail!(
                    "Path traversal detected: '{}' resolves outside destination directory",
                    entry_path.display()
                );
            }
        } else {
            // For non-existent paths, check the parent
            if let Some(parent) = full_path.parent() {
                if parent.exists() {
                    let canonical_parent = parent
                        .canonicalize()
                        .context("Failed to canonicalize parent path")?;

                    if !canonical_parent.starts_with(&canonical_dest) {
                        anyhow::bail!(
                            "Path traversal detected: '{}' would be created outside destination directory",
                            entry_path.display()
                        );
                    }
                }
            }
        }

        Ok(full_path)
    }

    fn extract_tar_gz(archive_path: &Path, dest_dir: &Path) -> Result<()> {
        let file = File::open(archive_path).context("Failed to open archive")?;
        let decoder = GzDecoder::new(file);
        let archive = Archive::new(decoder);
        Self::extract_tar_safe(archive, dest_dir)
    }

    fn extract_tar_xz(archive_path: &Path, dest_dir: &Path) -> Result<()> {
        let file = File::open(archive_path).context("Failed to open archive")?;
        let decoder = XzDecoder::new(file);
        let archive = Archive::new(decoder);
        Self::extract_tar_safe(archive, dest_dir)
    }

    fn extract_tar(archive_path: &Path, dest_dir: &Path) -> Result<()> {
        let file = File::open(archive_path).context("Failed to open archive")?;
        let archive = Archive::new(file);
        Self::extract_tar_safe(archive, dest_dir)
    }

    /// Safe tar extraction with path validation
    fn extract_tar_safe<R: std::io::Read>(mut archive: Archive<R>, dest_dir: &Path) -> Result<()> {
        // Disable potentially dangerous features
        archive.set_preserve_permissions(false);
        archive.set_preserve_mtime(false);
        archive.set_unpack_xattrs(false);

        let mut stats = ExtractionStats {
            total_bytes_extracted: 0,
            files_extracted: 0,
        };

        for entry_result in archive
            .entries()
            .context("Failed to read archive entries")?
        {
            let entry = entry_result.context("Failed to read archive entry")?;

            // Get entry path and validate it
            let entry_path = entry.path().context("Failed to get entry path")?;
            let entry_path_buf = entry_path.to_path_buf();

            // Validate the path doesn't escape
            let safe_path = Self::validate_extraction_path(dest_dir, &entry_path_buf)?;

            // Check entry type
            let entry_type = entry.header().entry_type();

            match entry_type {
                tar::EntryType::Regular | tar::EntryType::Continuous => {
                    // Get file size and check limits
                    let file_size = entry.header().size().context("Failed to get file size")?;
                    Self::check_extraction_limits(&mut stats, file_size)?;

                    // Regular file - create parent directories and extract
                    if let Some(parent) = safe_path.parent() {
                        std::fs::create_dir_all(parent).context(format!(
                            "Failed to create parent directory: {}",
                            parent.display()
                        ))?;
                    }

                    // Extract file with size limit (defense in depth)
                    let mut limited_reader = entry.take(MAX_EXTRACTED_FILE_SIZE);
                    let mut outfile = File::create(&safe_path)
                        .context(format!("Failed to create file: {}", safe_path.display()))?;

                    std::io::copy(&mut limited_reader, &mut outfile)
                        .context(format!("Failed to extract file: {}", safe_path.display()))?;
                }
                tar::EntryType::Directory => {
                    // Directory - count as file but no size
                    Self::check_extraction_limits(&mut stats, 0)?;

                    std::fs::create_dir_all(&safe_path).context(format!(
                        "Failed to create directory: {}",
                        safe_path.display()
                    ))?;
                }
                tar::EntryType::Symlink | tar::EntryType::Link => {
                    // SECURITY: Block symlinks and hardlinks during extraction
                    // They will be validated separately during deployment
                    log::warn!(
                        "Skipping symlink/hardlink in archive: {}",
                        entry_path_buf.display()
                    );
                    continue;
                }
                _ => {
                    // Skip other entry types (char devices, block devices, fifos, etc.)
                    log::warn!(
                        "Skipping unsupported entry type {:?}: {}",
                        entry_type,
                        entry_path_buf.display()
                    );
                    continue;
                }
            }
        }

        log::info!(
            "Extraction complete: {} files, {} bytes",
            stats.files_extracted,
            stats.total_bytes_extracted
        );

        Ok(())
    }

    fn extract_zip(archive_path: &Path, dest_dir: &Path) -> Result<()> {
        let file = File::open(archive_path).context("Failed to open archive")?;
        let mut archive = zip::ZipArchive::new(file).context("Failed to read zip archive")?;

        let mut stats = ExtractionStats {
            total_bytes_extracted: 0,
            files_extracted: 0,
        };

        // Check total number of files upfront
        if archive.len() > MAX_FILES_IN_ARCHIVE {
            anyhow::bail!(
                "Archive contains {} files, exceeds maximum ({}). Possible billion laughs attack.",
                archive.len(),
                MAX_FILES_IN_ARCHIVE
            );
        }

        for i in 0..archive.len() {
            let file = archive.by_index(i).context("Failed to read zip entry")?;

            // Get the file name and validate it
            let file_name = file.name();
            let entry_path = Path::new(file_name);

            // Validate the path doesn't escape
            let safe_path = Self::validate_extraction_path(dest_dir, entry_path)?;

            if file.is_dir() {
                // Directory - count as file but no size
                Self::check_extraction_limits(&mut stats, 0)?;

                std::fs::create_dir_all(&safe_path).context(format!(
                    "Failed to create directory: {}",
                    safe_path.display()
                ))?;
            } else if file.is_file() {
                // Get file size and check limits
                let file_size = file.size();
                Self::check_extraction_limits(&mut stats, file_size)?;

                // Get unix mode before moving file
                #[cfg(unix)]
                let unix_mode = file.unix_mode();

                // Regular file
                if let Some(parent) = safe_path.parent() {
                    std::fs::create_dir_all(parent).context(format!(
                        "Failed to create parent directory: {}",
                        parent.display()
                    ))?;
                }

                let mut outfile = File::create(&safe_path)
                    .context(format!("Failed to create file: {}", safe_path.display()))?;

                // Extract with size limit (defense in depth)
                let mut limited_reader = file.take(MAX_EXTRACTED_FILE_SIZE);
                std::io::copy(&mut limited_reader, &mut outfile)
                    .context(format!("Failed to extract file: {}", safe_path.display()))?;

                // Set permissions on Unix (but not SUID/SGID bits)
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = unix_mode {
                        // Mask out SUID (04000) and SGID (02000) bits for security
                        let safe_mode = mode & 0o0777;
                        std::fs::set_permissions(
                            &safe_path,
                            std::fs::Permissions::from_mode(safe_mode),
                        )
                        .context("Failed to set file permissions")?;
                    }
                }
            } else {
                // Skip symlinks and other special files
                log::warn!("Skipping special file in zip archive: {}", file_name);
                continue;
            }
        }

        log::info!(
            "Extraction complete: {} files, {} bytes",
            stats.files_extracted,
            stats.total_bytes_extracted
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_path_traversal_detection() {
        // Create a temporary directory for testing
        let dest = std::env::temp_dir().join("ora_test_extraction");
        std::fs::create_dir_all(&dest).expect("Failed to create test directory");

        // Should reject path traversal
        let bad_path = Path::new("../../etc/passwd");
        assert!(Extractor::validate_extraction_path(&dest, bad_path).is_err());

        // Should reject absolute paths
        let bad_path = Path::new("/etc/passwd");
        assert!(Extractor::validate_extraction_path(&dest, bad_path).is_err());

        // Should accept normal relative paths
        let good_path = Path::new("subdir/file.txt");
        assert!(Extractor::validate_extraction_path(&dest, good_path).is_ok());

        // Cleanup
        let _ = std::fs::remove_dir_all(&dest);
    }

    #[test]
    fn test_extraction_cache_cleanup() {
        // BUG-3: Test that old extraction directory is cleaned before extracting
        use std::io::Write;

        // Create a temporary extraction directory
        let dest = std::env::temp_dir().join("ora_test_extraction_cleanup");

        // First, create the directory with an old file
        fs::create_dir_all(&dest).expect("Failed to create test directory");
        let old_file = dest.join("old_version").join("old_file.txt");
        fs::create_dir_all(old_file.parent().unwrap()).expect("Failed to create old directory");
        let mut file = fs::File::create(&old_file).expect("Failed to create old file");
        file.write_all(b"old content")
            .expect("Failed to write old file");

        // Verify the old file exists
        assert!(old_file.exists(), "Old file should exist");

        // Create a simple tar.gz archive with new content
        let archive_path = std::env::temp_dir().join("test_archive.tar.gz");
        {
            use flate2::write::GzEncoder;
            use flate2::Compression;
            use tar::Builder;

            let tar_gz = fs::File::create(&archive_path).expect("Failed to create archive file");
            let encoder = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = Builder::new(encoder);

            // Add a new file to the archive
            let new_content = b"new content";
            let mut header = tar::Header::new_gnu();
            header.set_size(new_content.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append_data(&mut header, "new_version/new_file.txt", &new_content[..])
                .expect("Failed to add file to archive");

            tar.finish().expect("Failed to finish archive");
        }

        // Extract the new archive - this should clean the old directory first
        Extractor::extract(&archive_path, &dest).expect("Failed to extract archive");

        // Verify old file no longer exists (directory was cleaned)
        assert!(!old_file.exists(), "Old file should have been cleaned up");

        // Verify new file exists
        let new_file = dest.join("new_version").join("new_file.txt");
        assert!(new_file.exists(), "New file should exist");

        // Read and verify new file content
        let content = fs::read_to_string(&new_file).expect("Failed to read new file");
        assert_eq!(content, "new content");

        // Cleanup
        let _ = fs::remove_file(&archive_path);
        let _ = fs::remove_dir_all(&dest);
    }
}
