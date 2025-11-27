use anyhow::Result;

use crate::cli::args::{InstallArgs, UpdateArgs};
use crate::cli::commands::{install, uninstall};
use crate::registry::RegistryManager;
use crate::storage::database::load_installed_db;

pub async fn execute(args: UpdateArgs) -> Result<()> {
    let db = load_installed_db().await?;

    let packages_to_update: Vec<String> = if args.all {
        db.packages.keys().cloned().collect()
    } else if let Some(pkg) = args.package {
        vec![pkg]
    } else {
        anyhow::bail!("Specify a package name or use --all");
    };

    for package_name in packages_to_update {
        log::debug!("Checking updates for {}...", package_name);

        let installed = match db.packages.get(&package_name) {
            Some(pkg) => pkg,
            None => {
                // If updating a specific package (not --all), fail
                if !args.all {
                    anyhow::bail!("Package '{}' not installed", package_name);
                }
                // For --all, just skip packages that aren't installed
                log::warn!("Package '{}' not installed, skipping", package_name);
                continue;
            }
        };

        // Parse registry_source to determine how to load the .repo file
        let (repo_file_path, registry_name) =
            if let Some(file_path) = installed.registry_source.strip_prefix("file:") {
                // Package was installed from a local .repo file
                log::debug!(
                    "Package '{}' was installed from local file: {}",
                    package_name,
                    file_path
                );
                (Some(file_path.to_string()), None)
            } else if let Some(reg_name) = installed.registry_source.strip_prefix("registry:") {
                // Package was installed from a registry
                log::debug!(
                    "Package '{}' was installed from registry: {}",
                    package_name,
                    reg_name
                );
                (None, Some(reg_name.to_string()))
            } else {
                log::warn!(
                    "Unknown registry_source format for '{}': {}",
                    package_name,
                    installed.registry_source
                );
                log::warn!("Attempting to find package in registries...");
                (None, None)
            };

        // Load repo config to check allow_insecure flag
        let (repo_config, _) = if let Some(repo_file) = &repo_file_path {
            // Load from local file
            let content = match tokio::fs::read_to_string(repo_file).await {
                Ok(c) => c,
                Err(e) => {
                    log::warn!(
                        "Could not read .repo file '{}' for package '{}': {}",
                        repo_file,
                        package_name,
                        e
                    );
                    log::warn!("The original .repo file may have been moved or deleted.");
                    log::warn!("Skipping update for '{}'", package_name);
                    continue;
                }
            };
            let config = match toml::from_str(&content) {
                Ok(c) => c,
                Err(e) => {
                    log::warn!("Could not parse .repo file for '{}': {}", package_name, e);
                    continue;
                }
            };
            (config, installed.registry_source.clone())
        } else if let Some(reg_name) = &registry_name {
            // Find in specific registry
            match RegistryManager::find_package_in_specific_registry(&package_name, reg_name).await
            {
                Ok(result) => result,
                Err(e) => {
                    log::warn!(
                        "Could not find package '{}' in registry '{}': {}",
                        package_name,
                        reg_name,
                        e
                    );
                    continue;
                }
            }
        } else {
            // Fallback: search in all registries
            match RegistryManager::find_package(&package_name).await {
                Ok(result) => result,
                Err(e) => {
                    log::warn!("Could not find package '{}': {}", package_name, e);
                    continue;
                }
            }
        };

        // Determine if we should use allow_insecure
        // Use the flag from the installed package if it was set during installation
        // Otherwise, fall back to the repo config
        let allow_insecure = installed.allow_insecure || repo_config.security.allow_insecure;

        // Get latest version (simplified - would need provider logic)
        println!("Updating {} (current: {})", package_name, installed.version);

        // Uninstall old version
        let uninstall_args = crate::cli::args::UninstallArgs {
            package: package_name.clone(),
            version: None,
            purge: false,
        };
        uninstall::execute(uninstall_args).await?;

        // Install new version
        let install_args = InstallArgs {
            package: package_name.clone(),
            version: None, // Latest
            repo: repo_file_path,
            userland: installed.install_mode == "userland",
            system: installed.install_mode == "system",
            allow_insecure,
            local: None,
            metadata: None,
        };
        install::execute(install_args).await?;
    }

    Ok(())
}
