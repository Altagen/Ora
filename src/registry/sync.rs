use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::config::repo::RepoConfig;
use crate::storage::cache::Cache;

pub struct RegistrySync;

impl RegistrySync {
    pub async fn sync_registry(name: &str, url: &str) -> Result<()> {
        log::info!("Syncing registry '{}' from {}", name, url);

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
        Ok(())
    }

    pub async fn find_package_in_registry(
        registry_name: &str,
        package_name: &str,
    ) -> Result<RepoConfig> {
        let registry_path = Cache::registry_path(registry_name)?;

        if !registry_path.exists() {
            anyhow::bail!("Registry '{}' not synced", registry_name);
        }

        // Look for package.repo file
        let repo_file = registry_path
            .join("packages")
            .join(format!("{}.repo", package_name));

        if !repo_file.exists() {
            anyhow::bail!(
                "Package '{}' not found in registry '{}'",
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

    fn git_clone(url: &str, dest: &PathBuf) -> Result<()> {
        log::debug!("Cloning {} to {:?}", url, dest);

        // SECURITY: Validate Git URL to prevent command injection
        crate::security::validate_git_url(url).context("Git URL validation failed")?;

        // Ensure parent directory exists
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }

        git2::Repository::clone(url, dest).context("Failed to clone repository")?;

        Ok(())
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
