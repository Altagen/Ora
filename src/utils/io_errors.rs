//! User-friendly IO error handling
//!
//! This module provides better error messages for common IO errors,
//! especially permission issues and disk space problems.

use anyhow::{Context, Result};
use std::io;
use std::path::Path;

/// Read a file with user-friendly error messages
///
/// This function provides specific, actionable error messages for:
/// - Permission denied (EACCES)
/// - File not found (ENOENT) - returns None instead of error
/// - Other IO errors with full context
pub fn read_file_user_friendly(path: &Path) -> Result<Option<String>> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(Some(content)),
        Err(e) => {
            let path_display = path.display();

            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    anyhow::bail!(
                        "❌ Permission denied reading: {}\n\
                         \n\
                         Fix this by running:\n\
                         ├─ chmod 644 {}\n\
                         └─ Or check parent directory permissions: ls -ld {}\n\
                         \n\
                         If this is a system-wide config, you may need sudo.",
                        path_display,
                        path_display,
                        path.parent().unwrap_or(path).display()
                    );
                }
                io::ErrorKind::NotFound => {
                    // File doesn't exist - this is OK for configs (use defaults)
                    Ok(None)
                }
                _ => Err(e).context(format!("Failed to read file: {}", path_display)),
            }
        }
    }
}

/// Read a file asynchronously with user-friendly error messages
pub async fn read_file_user_friendly_async(path: &Path) -> Result<Option<String>> {
    match tokio::fs::read_to_string(path).await {
        Ok(content) => Ok(Some(content)),
        Err(e) => {
            let path_display = path.display();

            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    anyhow::bail!(
                        "❌ Permission denied reading: {}\n\
                         \n\
                         Fix this by running:\n\
                         ├─ chmod 644 {}\n\
                         └─ Or check parent directory permissions: ls -ld {}\n\
                         \n\
                         If this is a system-wide config, you may need sudo.",
                        path_display,
                        path_display,
                        path.parent().unwrap_or(path).display()
                    );
                }
                io::ErrorKind::NotFound => {
                    // File doesn't exist - this is OK for configs (use defaults)
                    Ok(None)
                }
                _ => Err(e).context(format!("Failed to read file: {}", path_display)),
            }
        }
    }
}

/// Write a file with user-friendly error messages
///
/// This function provides specific, actionable error messages for:
/// - Permission denied (EACCES)
/// - Disk full (ENOSPC)
/// - Read-only filesystem (EROFS)
/// - Other IO errors with full context
pub fn write_file_user_friendly(path: &Path, content: &str) -> Result<()> {
    match std::fs::write(path, content) {
        Ok(()) => Ok(()),
        Err(e) => {
            let path_display = path.display();

            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    anyhow::bail!(
                        "❌ Permission denied writing: {}\n\
                         \n\
                         Fix this by running:\n\
                         ├─ chmod 644 {} (if file exists)\n\
                         ├─ chmod 755 {} (parent directory)\n\
                         └─ Or run with sudo for system-wide configs\n\
                         \n\
                         Alternative: Use a different location with ORA_CONFIG_DIR environment variable",
                        path_display,
                        path_display,
                        path.parent().unwrap_or(path).display()
                    );
                }
                _ => {
                    // Check for specific raw OS errors
                    match e.raw_os_error() {
                        Some(28) => {
                            // ENOSPC on Linux
                            anyhow::bail!(
                                "❌ Disk full - cannot write to: {}\n\
                                 \n\
                                 Free up disk space or use a different location:\n\
                                 ├─ For config: export ORA_CONFIG_DIR=/path/with/space\n\
                                 ├─ For cache: export ORA_CACHE_DIR=/path/with/space\n\
                                 └─ Check disk usage: df -h {}",
                                path_display,
                                path.parent().unwrap_or(path).display()
                            );
                        }
                        Some(30) => {
                            // EROFS on Linux (read-only filesystem)
                            anyhow::bail!(
                                "❌ Read-only filesystem: {}\n\
                                 \n\
                                 This location is read-only (immutable OS or mounted read-only).\n\
                                 Use a writable location:\n\
                                 └─ export ORA_CONFIG_DIR=$HOME/.config/ora",
                                path_display
                            );
                        }
                        _ => Err(e).context(format!("Failed to write file: {}", path_display)),
                    }
                }
            }
        }
    }
}

/// Write a file asynchronously with user-friendly error messages
pub async fn write_file_user_friendly_async(path: &Path, content: &[u8]) -> Result<()> {
    match tokio::fs::write(path, content).await {
        Ok(()) => Ok(()),
        Err(e) => {
            let path_display = path.display();

            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    anyhow::bail!(
                        "❌ Permission denied writing: {}\n\
                         \n\
                         Fix this by running:\n\
                         ├─ chmod 644 {} (if file exists)\n\
                         ├─ chmod 755 {} (parent directory)\n\
                         └─ Or run with sudo for system-wide configs\n\
                         \n\
                         Alternative: Use a different location with ORA_CONFIG_DIR environment variable",
                        path_display,
                        path_display,
                        path.parent().unwrap_or(path).display()
                    );
                }
                _ => {
                    // Check for specific raw OS errors
                    match e.raw_os_error() {
                        Some(28) => {
                            // ENOSPC on Linux
                            anyhow::bail!(
                                "❌ Disk full - cannot write to: {}\n\
                                 \n\
                                 Free up disk space or use a different location:\n\
                                 ├─ For config: export ORA_CONFIG_DIR=/path/with/space\n\
                                 ├─ For cache: export ORA_CACHE_DIR=/path/with/space\n\
                                 └─ Check disk usage: df -h {}",
                                path_display,
                                path.parent().unwrap_or(path).display()
                            );
                        }
                        Some(30) => {
                            // EROFS on Linux (read-only filesystem)
                            anyhow::bail!(
                                "❌ Read-only filesystem: {}\n\
                                 \n\
                                 This location is read-only (immutable OS or mounted read-only).\n\
                                 Use a writable location:\n\
                                 └─ export ORA_CONFIG_DIR=$HOME/.config/ora",
                                path_display
                            );
                        }
                        _ => Err(e).context(format!("Failed to write file: {}", path_display)),
                    }
                }
            }
        }
    }
}

/// Create directory with user-friendly error messages
pub fn create_dir_all_user_friendly(path: &Path) -> Result<()> {
    match std::fs::create_dir_all(path) {
        Ok(()) => Ok(()),
        Err(e) => {
            let path_display = path.display();

            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    anyhow::bail!(
                        "❌ Permission denied creating directory: {}\n\
                         \n\
                         Fix this by running:\n\
                         ├─ chmod 755 {} (parent directory)\n\
                         └─ Or run with sudo for system-wide directories\n\
                         \n\
                         Alternative: Use environment variables to change locations:\n\
                         ├─ ORA_CONFIG_DIR for config directory\n\
                         ├─ ORA_DATA_DIR for data directory\n\
                         └─ ORA_CACHE_DIR for cache directory",
                        path_display,
                        path.parent().unwrap_or(path).display()
                    );
                }
                _ => {
                    match e.raw_os_error() {
                        Some(28) => {
                            // ENOSPC
                            anyhow::bail!(
                                "❌ Disk full - cannot create directory: {}\n\
                                 \n\
                                 Free up disk space or use a different location.",
                                path_display
                            );
                        }
                        Some(30) => {
                            // EROFS
                            anyhow::bail!(
                                "❌ Read-only filesystem - cannot create directory: {}\n\
                                 \n\
                                 Use environment variables to specify writable locations.",
                                path_display
                            );
                        }
                        _ => {
                            Err(e).context(format!("Failed to create directory: {}", path_display))
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_nonexistent_file_returns_none() {
        let result = read_file_user_friendly(Path::new("/nonexistent/file.txt"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_read_existing_file() {
        // Create a temp file
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("ora_test_read.txt");
        std::fs::write(&temp_file, "test content").expect("Failed to write test file");

        let result = read_file_user_friendly(&temp_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("test content".to_string()));

        // Cleanup
        let _ = std::fs::remove_file(&temp_file);
    }
}
