use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::config::repo::RepoConfig;
use crate::storage::cache::Cache;
use crate::utils::http;

pub struct RegistrySync;

#[derive(Debug, PartialEq)]
pub enum RegistryType {
    Git,       // Git repository (contains .git in URL)
    DirectUrl, // Direct .repo file URL (HTTP/HTTPS endpoint)
}

impl RegistryType {
    pub fn from_url(url: &str) -> Self {
        // Check if URL contains .git anywhere (common for Git repositories)
        // Examples: https://github.com/user/repo.git or https://gitlab.com/user/repo.git
        if url.contains(".git") {
            RegistryType::Git
        } else {
            // Default to Direct URL for HTTP/HTTPS endpoints
            // This allows nginx redirects, clean URLs, and flexible setups
            // Examples: https://example.com/windsurf.repo or https://example.com/windsurf/
            RegistryType::DirectUrl
        }
    }
}

impl RegistrySync {
    pub async fn sync_registry(name: &str, url: &str) -> Result<()> {
        let registry_type = RegistryType::from_url(url);

        match registry_type {
            RegistryType::Git => {
                log::info!("Syncing Git registry '{}' from {}", name, url);
                let registry_path = Cache::registry_path(name)?;

                // Check if it's a git repository
                if registry_path.join(".git").exists() {
                    // Pull latest changes
                    Self::git_pull(&registry_path)?;
                } else {
                    // Clone for the first time
                    Self::git_clone(url, &registry_path)?;
                }

                log::info!("Registry '{}' synced successfully", name);
            }
            RegistryType::DirectUrl => {
                log::info!("Syncing Direct URL registry '{}' from {}", name, url);
                // For Direct URL registries, we fetch the .repo file on-demand
                // No need to sync/download it now
                log::debug!("Direct URL registries are fetched on-demand, no sync needed");
            }
        }

        Ok(())
    }

    pub async fn find_package_in_registry(
        registry_name: &str,
        package_name: &str,
    ) -> Result<RepoConfig> {
        let config = crate::storage::database::load_global_config().await?;

        // Find the registry to determine its type
        let registry = config
            .registries
            .iter()
            .find(|r| r.name == registry_name)
            .context(format!("Registry '{}' not found", registry_name))?;

        let registry_type = RegistryType::from_url(&registry.url);

        match registry_type {
            RegistryType::Git => {
                // Git registry: look for .repo file in local clone
                let registry_path = Cache::registry_path(registry_name)?;

                if !registry_path.exists() {
                    anyhow::bail!("Registry '{}' not synced", registry_name);
                }

                // Look for package.repo file in ora-registry/ directory
                let repo_file = registry_path
                    .join("ora-registry")
                    .join(format!("{}.repo", package_name));

                if !repo_file.exists() {
                    anyhow::bail!(
                        "Package '{}' not found in registry '{}'. \
                         Registry must contain an 'ora-registry/' directory with .repo files.",
                        package_name,
                        registry_name
                    );
                }

                // Load and parse .repo file
                let content = tokio::fs::read_to_string(&repo_file)
                    .await
                    .context("Failed to read .repo file")?;

                let repo_config: RepoConfig =
                    toml::from_str(&content).context("Failed to parse .repo file")?;

                Ok(repo_config)
            }
            RegistryType::DirectUrl => {
                // Direct URL registry: fetch .repo file via HTTP
                log::debug!("Fetching .repo file from {}", registry.url);

                let client = http::HttpClient::new()?;
                let response = client
                    .get(&registry.url)
                    .await
                    .context(format!("Failed to fetch .repo file from {}", registry.url))?;

                let content = response
                    .text()
                    .await
                    .context("Failed to read .repo file content")?;

                let repo_config: RepoConfig =
                    toml::from_str(&content).context("Failed to parse .repo file")?;

                // Validate that the package name from URL matches
                if repo_config.name != package_name {
                    log::warn!(
                        "Package name mismatch: registry expects '{}' but .repo contains '{}'",
                        package_name,
                        repo_config.name
                    );
                }

                Ok(repo_config)
            }
        }
    }

    fn git_clone(url: &str, dest: &PathBuf) -> Result<()> {
        log::debug!("Cloning {} to {:?}", url, dest);

        // SECURITY: Validate Git URL to prevent command injection
        crate::security::validate_git_url(url).context("Git URL validation failed")?;

        // Ensure parent directory exists
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Use shallow clone (depth=1) to protect against git bombs
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.depth(1);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        log::debug!("Using shallow clone (depth=1) for security");
        builder
            .clone(url, dest)
            .context("Failed to clone repository")?;

        // Check repository size after cloning
        Self::check_repo_size(dest)?;

        Ok(())
    }

    fn check_repo_size(repo_path: &PathBuf) -> Result<()> {
        use std::fs;

        // Calculate .git directory size
        let git_dir = repo_path.join(".git");
        let size_bytes = Self::dir_size(&git_dir)?;
        let size_mb = size_bytes / (1024 * 1024);

        // Load config to get max size limit
        let config = tokio::runtime::Handle::try_current().ok().and_then(|_| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current()
                    .block_on(async { crate::storage::database::load_global_config().await.ok() })
            })
        });

        let max_size_mb = config
            .as_ref()
            .map(|c| c.security.max_git_size_mb)
            .unwrap_or(1024);

        if size_mb > max_size_mb {
            // Clean up oversized repository
            let _ = fs::remove_dir_all(repo_path);

            anyhow::bail!(
                "Repository size ({} MB) exceeds security limit ({} MB). \
                 This may be a git bomb attack. Increase limit in config if legitimate.",
                size_mb,
                max_size_mb
            );
        }

        log::debug!(
            "Repository size: {} MB (limit: {} MB)",
            size_mb,
            max_size_mb
        );
        Ok(())
    }

    fn dir_size(path: &std::path::Path) -> Result<u64> {
        let mut total = 0;

        if path.is_dir() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    total += Self::dir_size(&path)?;
                } else {
                    total += entry.metadata()?.len();
                }
            }
        }

        Ok(total)
    }

    fn git_pull(repo_path: &PathBuf) -> Result<()> {
        log::debug!("Pulling latest changes in {:?}", repo_path);

        let repo = git2::Repository::open(repo_path).context("Failed to open repository")?;

        // Fetch
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["main", "master"], None, None)?;

        // Get reference to FETCH_HEAD
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

        // Merge
        let analysis = repo.merge_analysis(&[&fetch_commit])?;

        if analysis.0.is_up_to_date() {
            log::debug!("Already up to date");
        } else if analysis.0.is_fast_forward() {
            // Fast-forward merge
            let refname = "refs/heads/main"; // or master
            let mut reference = repo.find_reference(refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            repo.set_head(refname)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        }

        Ok(())
    }
}
