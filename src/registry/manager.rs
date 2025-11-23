use anyhow::{Context, Result};

use crate::config::global::Registry;
use crate::config::repo::RepoConfig;
use crate::registry::sync::RegistrySync;
use crate::storage::cache::Cache;
use crate::storage::database::{load_global_config, save_global_config};

pub struct RegistryManager;

impl RegistryManager {
    pub async fn add_registry(
        name: String,
        url: String,
        trust_level: String,
        ca_cert: Option<String>,
        pin_cert: bool,
    ) -> Result<()> {
        let mut config = load_global_config().await?;

        // Check if registry already exists
        if config.registries.iter().any(|r| r.name == name) {
            anyhow::bail!("Registry '{}' already exists", name);
        }

        let trust_level = match trust_level.to_lowercase().as_str() {
            "public" => crate::config::global::TrustLevel::Public,
            "private" => crate::config::global::TrustLevel::Private,
            _ => anyhow::bail!("Invalid trust level, must be 'public' or 'private'"),
        };

        let tls = if ca_cert.is_some() || pin_cert {
            Some(crate::config::global::TlsConfig {
                ca_cert,
                ca_cert_inline: None,
                cert_fingerprint: None, // TODO: Implement certificate pinning
                public_key_pin: None,
            })
        } else {
            None
        };

        let registry = Registry {
            name: name.clone(),
            url: url.clone(),
            trust_level,
            enabled: true,
            tls,
            gpg_key: None,
        };

        config.registries.push(registry);
        save_global_config(&config).await?;

        log::info!("Registry '{}' added successfully", name);

        // Sync the registry
        RegistrySync::sync_registry(&name, &url).await?;

        Ok(())
    }

    pub async fn list_registries(verbose: bool) -> Result<()> {
        let config = load_global_config().await?;

        if config.registries.is_empty() {
            println!("No registries configured");
            return Ok(());
        }

        for registry in &config.registries {
            if verbose {
                println!("Name: {}", registry.name);
                println!("  URL: {}", registry.url);
                println!("  Trust Level: {:?}", registry.trust_level);
                println!("  Enabled: {}", registry.enabled);
                println!();
            } else {
                let enabled_mark = if registry.enabled { "✓" } else { "✗" };
                println!("{} {} - {}", enabled_mark, registry.name, registry.url);
            }
        }

        Ok(())
    }

    pub async fn remove_registry(name: String) -> Result<()> {
        let mut config = load_global_config().await?;

        let initial_len = config.registries.len();
        config.registries.retain(|r| r.name != name);

        if config.registries.len() == initial_len {
            anyhow::bail!("Registry '{}' not found", name);
        }

        save_global_config(&config).await?;
        log::info!("Registry '{}' removed", name);

        Ok(())
    }

    pub async fn sync_registries(name: Option<String>) -> Result<()> {
        let config = load_global_config().await?;

        if let Some(name) = name {
            let registry = config
                .registries
                .iter()
                .find(|r| r.name == name)
                .context(format!("Registry '{}' not found", name))?;

            println!("Syncing registry: {}", registry.name);
            RegistrySync::sync_registry(&registry.name, &registry.url).await?;
            println!("✓ Registry '{}' synced successfully", registry.name);
        } else {
            if config.registries.is_empty() {
                println!("No registries configured. Add one with: ora registry add <name> <url>");
                return Ok(());
            }

            let enabled_registries: Vec<_> = config
                .registries
                .iter()
                .filter(|r| r.enabled)
                .collect();

            if enabled_registries.is_empty() {
                println!("No enabled registries to sync.");
                return Ok(());
            }

            println!("Syncing {} registr{}...",
                enabled_registries.len(),
                if enabled_registries.len() == 1 { "y" } else { "ies" }
            );

            for registry in enabled_registries {
                println!("  → Syncing '{}'...", registry.name);
                match RegistrySync::sync_registry(&registry.name, &registry.url).await {
                    Ok(_) => println!("    ✓ Synced successfully"),
                    Err(e) => {
                        log::error!("Failed to sync registry '{}': {}", registry.name, e);
                        println!("    ✗ Failed: {}", e);
                    }
                }
            }

            println!("\nSync complete!");
        }

        Ok(())
    }

    pub async fn update_registries(name: Option<String>) -> Result<()> {
        let config = load_global_config().await?;

        if let Some(name) = name {
            let registry = config
                .registries
                .iter()
                .find(|r| r.name == name)
                .context(format!("Registry '{}' not found", name))?;

            RegistrySync::sync_registry(&registry.name, &registry.url).await?;
        } else {
            for registry in &config.registries {
                if registry.enabled {
                    log::info!("Updating registry: {}", registry.name);
                    RegistrySync::sync_registry(&registry.name, &registry.url).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn find_package(package_name: &str) -> Result<(RepoConfig, String)> {
        let config = load_global_config().await?;

        // UX IMPROVEMENT: Check if no registries are configured (first-run detection)
        if config.registries.is_empty() {
            anyhow::bail!(
                "❌ No registries configured yet!\n\
                 \n\
                 To use Ora, you need to add at least one package registry:\n\
                 \n\
                 Example:\n\
                 └─ ora registry add mycompany https://github.com/mycompany/packages\n\
                 \n\
                 Then try installing again:\n\
                 └─ ora install {}\n\
                 \n\
                 Run 'ora registry --help' for more information.",
                package_name
            );
        }

        let mut found_registries = Vec::new();
        let mut first_match: Option<(RepoConfig, String)> = None;
        let enabled_count = config.registries.iter().filter(|r| r.enabled).count();

        for registry in &config.registries {
            if !registry.enabled {
                continue;
            }

            match RegistrySync::find_package_in_registry(&registry.name, package_name).await {
                Ok(repo_config) => {
                    found_registries.push(registry.name.clone());

                    if first_match.is_none() {
                        first_match = Some((repo_config, registry.name.clone()));
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }

        match first_match {
            Some((repo_config, registry_name)) => {
                // Warn if package exists in multiple registries
                if found_registries.len() > 1 {
                    log::warn!(
                        "⚠️  Package '{}' exists in multiple registries: {}",
                        package_name,
                        found_registries.join(", ")
                    );
                    log::warn!(
                        "Using '{}' from registry '{}'. To select a specific registry, use: ora install {}@<registry>",
                        package_name,
                        registry_name,
                        package_name
                    );
                }

                log::info!(
                    "Found package '{}' in registry '{}'",
                    package_name,
                    registry_name
                );
                Ok((repo_config, registry_name))
            }
            None => {
                // UX IMPROVEMENT: Better error message when package not found
                if enabled_count == 0 {
                    anyhow::bail!(
                        "Package '{}' not found - No enabled registries.\n\
                         \n\
                         You have {} configured registr{} but {} disabled.\n\
                         Enable a registry with: ora registry enable <name>",
                        package_name,
                        config.registries.len(),
                        if config.registries.len() == 1 {
                            "y"
                        } else {
                            "ies"
                        },
                        if config.registries.len() == 1 {
                            "it's"
                        } else {
                            "they're all"
                        }
                    );
                } else {
                    anyhow::bail!(
                        "Package '{}' not found in any of {} configured registr{}.\n\
                         \n\
                         Searched in: {}\n\
                         \n\
                         Try:\n\
                         ├─ Updating registries: ora registry update\n\
                         ├─ Searching for packages: ora search {}\n\
                         └─ Adding more registries: ora registry add <name> <url>",
                        package_name,
                        enabled_count,
                        if enabled_count == 1 { "y" } else { "ies" },
                        config
                            .registries
                            .iter()
                            .filter(|r| r.enabled)
                            .map(|r| r.name.as_str())
                            .collect::<Vec<_>>()
                            .join(", "),
                        package_name
                    );
                }
            }
        }
    }

    pub async fn find_package_in_specific_registry(
        package_name: &str,
        registry_name: &str,
    ) -> Result<(RepoConfig, String)> {
        let config = load_global_config().await?;

        let registry = config
            .registries
            .iter()
            .find(|r| r.name == registry_name && r.enabled)
            .context(format!(
                "Registry '{}' not found or disabled",
                registry_name
            ))?;

        let repo_config = RegistrySync::find_package_in_registry(&registry.name, package_name)
            .await
            .context(format!(
                "Package '{}' not found in registry '{}'",
                package_name, registry_name
            ))?;

        log::info!(
            "Found package '{}' in registry '{}'",
            package_name,
            registry_name
        );
        Ok((repo_config, registry.name.clone()))
    }

    pub async fn verify_registry(name: String) -> Result<()> {
        let config = load_global_config().await?;

        println!("Verifying registry: {}", name);
        println!();

        // 1. Check if registry exists in config
        let registry = config
            .registries
            .iter()
            .find(|r| r.name == name)
            .context(format!("Registry '{}' not found in configuration", name))?;

        println!("✓ Registry found in configuration");
        println!("  Name: {}", registry.name);
        println!("  URL: {}", registry.url);
        println!("  Trust Level: {:?}", registry.trust_level);
        println!("  Enabled: {}", registry.enabled);

        // 2. Check if registry directory exists (has been synced)
        let registry_path = Cache::registry_path(&name)?;

        if !registry_path.exists() {
            println!("✗ Registry not synced locally");
            println!("  Expected path: {:?}", registry_path);
            println!("\n  Run 'ora registry sync {}' to download it", name);
            anyhow::bail!("Registry '{}' not synced", name);
        }

        println!("✓ Registry synced locally");
        println!("  Path: {:?}", registry_path);

        // 3. Check if it's a valid git repository
        match git2::Repository::open(&registry_path) {
            Ok(repo) => {
                println!("✓ Valid git repository");

                // Get current HEAD commit
                if let Ok(head) = repo.head() {
                    if let Some(commit) = head.target() {
                        println!("  Commit: {}", commit);
                    }
                }

                // Check remote URL
                if let Ok(remote) = repo.find_remote("origin") {
                    if let Some(url) = remote.url() {
                        println!("  Remote: {}", url);

                        // Verify it matches the configured URL
                        if url != registry.url {
                            println!("  ⚠️  Warning: Remote URL doesn't match configured URL");
                            println!("     Configured: {}", registry.url);
                            println!("     Actual: {}", url);
                        }
                    }
                }
            }
            Err(e) => {
                println!("✗ Not a valid git repository: {}", e);
                anyhow::bail!("Registry directory exists but is not a valid git repository");
            }
        }

        // 4. Check for ora-registry/ directory
        let ora_registry_dir = registry_path.join("ora-registry");

        if !ora_registry_dir.exists() {
            println!("✗ Missing 'ora-registry/' directory");
            println!("  A valid registry must contain an 'ora-registry/' directory with .repo files");
            anyhow::bail!("Registry '{}' is missing the required 'ora-registry/' directory", name);
        }

        println!("✓ 'ora-registry/' directory exists");
        let registry_dir = ora_registry_dir;

        // 5. Count .repo files
        let mut repo_files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&registry_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("repo") {
                    if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                        repo_files.push(file_name.to_string());
                    }
                }
            }
        }

        if repo_files.is_empty() {
            println!("⚠️  Warning: No .repo files found in registry directory");
            println!("  This registry appears to be empty");
        } else {
            println!("✓ Found {} package definition{}",
                repo_files.len(),
                if repo_files.len() == 1 { "" } else { "s" }
            );

            // Show first few packages
            let display_count = std::cmp::min(5, repo_files.len());
            for (i, file) in repo_files.iter().take(display_count).enumerate() {
                println!("  {}. {}", i + 1, file.trim_end_matches(".repo"));
            }

            if repo_files.len() > display_count {
                println!("  ... and {} more", repo_files.len() - display_count);
            }
        }

        println!();
        println!("✓ Registry '{}' verification complete!", name);

        Ok(())
    }
}
