use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::global::InstallMode;
use crate::config::repo::InstallConfig;
use crate::config::security_limits::MAX_PATH_LENGTH;
use crate::storage::paths::Paths;

pub struct Deployer;

impl Deployer {
    pub fn deploy(
        extract_dir: &Path,
        install_config: &InstallConfig,
        mode: InstallMode,
        package_name: &str,
        version: &str,
    ) -> Result<DeploymentResult> {
        log::info!("Deploying package to install directory");

        let install_dir = Paths::packages_dir(mode.clone())?
            .join(package_name)
            .join(version);
        let bin_dir = Paths::bin_dir(mode.clone())?;

        // Ensure directories exist
        std::fs::create_dir_all(&install_dir)?;
        std::fs::create_dir_all(&bin_dir)?;

        // Canonicalize paths for security validation
        let canonical_install_dir = install_dir
            .canonicalize()
            .context("Failed to canonicalize install directory")?;
        let canonical_bin_dir = bin_dir
            .canonicalize()
            .context("Failed to canonicalize bin directory")?;

        // Copy all files from extract_dir to install_dir
        Self::copy_directory(extract_dir, &install_dir)?;

        // Create symlinks for binaries
        let mut symlinks = Vec::new();
        let mut files = Vec::new();

        // Get all binaries
        if install_config.binaries.is_empty() {
            anyhow::bail!("No binaries specified in install config");
        }

        // Create symlinks for each binary
        for binary_pattern in &install_config.binaries {
            let binary_src = Self::resolve_path(&install_dir, binary_pattern)?;

            // Validate binary is within install directory
            Self::validate_path_within_base(&binary_src, &canonical_install_dir, "Binary")?;

            let binary_name = binary_src
                .file_name()
                .context("Invalid binary path")?
                .to_string_lossy()
                .to_string();
            let binary_link = bin_dir.join(&binary_name);

            // Create and validate symlink
            Self::create_symlink(
                &binary_src,
                &binary_link,
                &canonical_install_dir,
                &canonical_bin_dir,
            )?;
            symlinks.push(binary_link.to_string_lossy().to_string());
        }

        // Additional files
        for additional in &install_config.files {
            let src = Self::resolve_path(&install_dir, &additional.src)?;
            let dst = install_dir.join(&additional.dst);

            // Validate source is within install directory
            Self::validate_path_within_base(
                &src,
                &canonical_install_dir,
                "Additional file source",
            )?;

            // Validate destination doesn't escape
            let canonical_dst = if dst.exists() {
                dst.canonicalize()?
            } else {
                // For non-existent paths, check parent
                if let Some(parent) = dst.parent() {
                    std::fs::create_dir_all(parent)?;
                    parent.canonicalize()?
                } else {
                    anyhow::bail!("Invalid destination path: {}", dst.display());
                }
            };

            if !canonical_dst.starts_with(&canonical_install_dir) {
                anyhow::bail!(
                    "Additional file destination '{}' is outside install directory",
                    additional.dst
                );
            }

            if let Some(parent) = dst.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&src, &dst)?;
        }

        // Collect all installed files
        for entry in WalkDir::new(&install_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                files.push(entry.path().to_string_lossy().to_string());
            }
        }

        Ok(DeploymentResult {
            install_dir: install_dir.to_string_lossy().to_string(),
            symlinks,
            files,
        })
    }

    /// Validates that a path is within the base directory
    fn validate_path_within_base(path: &Path, base: &Path, description: &str) -> Result<()> {
        let canonical_path = path.canonicalize().context(format!(
            "Failed to canonicalize {}: {}",
            description,
            path.display()
        ))?;

        if !canonical_path.starts_with(base) {
            anyhow::bail!(
                "{} '{}' is outside allowed directory '{}'",
                description,
                path.display(),
                base.display()
            );
        }

        Ok(())
    }

    fn resolve_path(base: &Path, relative: &str) -> Result<PathBuf> {
        // Validate path length
        if relative.len() > MAX_PATH_LENGTH {
            anyhow::bail!(
                "Path length ({}) exceeds maximum ({}): {}",
                relative.len(),
                MAX_PATH_LENGTH,
                relative
            );
        }

        // Handle wildcards and resolve to actual file
        let pattern = base.join(relative);
        let pattern_str = pattern.to_string_lossy();

        // Simple glob pattern matching
        if pattern_str.contains('*') {
            use glob::glob;
            let mut matches: Vec<_> = glob(&pattern_str)?.filter_map(Result::ok).collect();
            if matches.is_empty() {
                anyhow::bail!("No files matched pattern: {}", pattern_str);
            }
            matches.sort();

            // Use safe indexing to get first match
            matches
                .into_iter()
                .next()
                .ok_or_else(|| anyhow::anyhow!("No files matched pattern: {}", pattern_str))
        } else {
            // Verify the path exists
            if !pattern.exists() {
                anyhow::bail!("Path does not exist: {}", pattern.display());
            }
            Ok(pattern)
        }
    }

    fn create_symlink(
        src: &Path,
        link: &Path,
        canonical_install_dir: &Path,
        canonical_bin_dir: &Path,
    ) -> Result<()> {
        // Validate symlink source (target) is within install directory
        let canonical_src = src.canonicalize().context(format!(
            "Failed to canonicalize symlink target: {}",
            src.display()
        ))?;

        if !canonical_src.starts_with(canonical_install_dir) {
            anyhow::bail!(
                "Symlink target '{}' is outside install directory",
                src.display()
            );
        }

        // Validate symlink location is within bin directory
        if let Some(link_parent) = link.parent() {
            let canonical_link_parent = link_parent.canonicalize().context(format!(
                "Failed to canonicalize symlink directory: {}",
                link_parent.display()
            ))?;

            if canonical_link_parent != *canonical_bin_dir {
                anyhow::bail!(
                    "Symlink location '{}' must be directly in bin directory '{}'",
                    link.display(),
                    canonical_bin_dir.display()
                );
            }
        } else {
            anyhow::bail!("Invalid symlink path: {}", link.display());
        }

        // Validate symlink target is a regular file
        if !canonical_src.is_file() {
            anyhow::bail!("Symlink target '{}' is not a regular file", src.display());
        }

        // Check if symlink target is executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(&canonical_src)?;
            let permissions = metadata.permissions();
            let mode = permissions.mode();

            // Check if file has execute permission for user
            if mode & 0o100 == 0 {
                log::warn!(
                    "Binary '{}' is not executable, setting execute permission",
                    canonical_src.display()
                );
                // Make it executable
                let mut new_permissions = permissions;
                new_permissions.set_mode(mode | 0o100);
                std::fs::set_permissions(&canonical_src, new_permissions)?;
            }
        }

        // Remove existing symlink if it exists
        if link.exists() || link.is_symlink() {
            // Verify it's actually a symlink before removing
            if link.is_symlink() {
                std::fs::remove_file(link).context(format!(
                    "Failed to remove existing symlink: {}",
                    link.display()
                ))?;
            } else {
                anyhow::bail!(
                    "Cannot create symlink: path '{}' already exists and is not a symlink",
                    link.display()
                );
            }
        }

        // Create the symlink
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(src, link).context(format!(
                "Failed to create symlink from '{}' to '{}'",
                link.display(),
                src.display()
            ))?;
        }

        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_file(src, link).context(format!(
                "Failed to create symlink from '{}' to '{}'",
                link.display(),
                src.display()
            ))?;
        }

        log::info!("Created symlink: {:?} -> {:?}", link, src);
        Ok(())
    }

    fn copy_directory(src: &Path, dst: &Path) -> Result<()> {
        std::fs::create_dir_all(dst)?;

        // Canonicalize destination to validate paths
        let canonical_dst = dst
            .canonicalize()
            .context("Failed to canonicalize destination directory")?;

        for entry in WalkDir::new(src) {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(src)?;
            let target = dst.join(relative);

            // Validate target doesn't escape destination
            // Check path components for traversal attempts
            for component in relative.components() {
                match component {
                    std::path::Component::ParentDir => {
                        anyhow::bail!("Path traversal detected in copy: '{}'", relative.display());
                    }
                    std::path::Component::RootDir => {
                        anyhow::bail!("Absolute path detected in copy: '{}'", relative.display());
                    }
                    _ => {}
                }
            }

            if entry.file_type().is_dir() {
                std::fs::create_dir_all(&target)?;
            } else if entry.file_type().is_file() {
                if let Some(parent) = target.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Verify target is within destination after creation
                if target.exists() || target.parent().map(|p| p.exists()).unwrap_or(false) {
                    let canonical_target = if target.exists() {
                        target.canonicalize()?
                    } else {
                        target
                            .parent()
                            .ok_or_else(|| anyhow::anyhow!("Invalid target path"))?
                            .canonicalize()?
                            .join(target.file_name().ok_or_else(|| {
                                anyhow::anyhow!("Target path has no filename component")
                            })?)
                    };

                    if let Ok(canonical_target) = canonical_target.canonicalize() {
                        if !canonical_target.starts_with(&canonical_dst) {
                            anyhow::bail!(
                                "File copy would escape destination: '{}'",
                                target.display()
                            );
                        }
                    }
                }

                std::fs::copy(path, &target)?;

                // Preserve permissions on Unix, but mask out SUID/SGID
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let metadata = std::fs::metadata(path)?;
                    let mode = metadata.permissions().mode();
                    // Mask out SUID (04000) and SGID (02000) bits
                    let safe_mode = mode & 0o0777;
                    std::fs::set_permissions(&target, std::fs::Permissions::from_mode(safe_mode))?;
                }
            } else if entry.file_type().is_symlink() {
                // SECURITY: Skip symlinks during copy
                // They were already blocked during extraction
                log::warn!(
                    "Skipping symlink during deployment copy: {}",
                    path.display()
                );
            }
        }

        Ok(())
    }
}

pub struct DeploymentResult {
    pub install_dir: String,
    pub symlinks: Vec<String>,
    pub files: Vec<String>,
}
