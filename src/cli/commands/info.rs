use anyhow::Result;

use crate::cli::args::InfoArgs;
use crate::registry::RegistryManager;
use crate::storage::database::load_installed_db;

pub async fn execute(args: InfoArgs) -> Result<()> {
    log::info!("Getting info for: {}", args.package);

    // Check if installed
    let db = load_installed_db().await?;
    if let Some(installed) = db.packages.get(&args.package) {
        println!("Package: {}", args.package);
        println!("Status: Installed");
        println!("Version: {}", installed.version);
        println!("Installed: {}", installed.installed_at);
        println!("Mode: {}", installed.install_mode);
        println!("Directory: {}", installed.install_dir);
        println!("Symlinks: {}", installed.symlinks.len());
        println!();
    }

    // Get info from registry
    match RegistryManager::find_package(&args.package).await {
        Ok((repo_config, registry_name)) => {
            println!("Package: {}", repo_config.name);
            println!("Description: {}", repo_config.description);
            if let Some(homepage) = &repo_config.homepage {
                println!("Homepage: {}", homepage);
            }
            println!("Provider: {:?}", repo_config.source.provider_type);
            println!("Registry: {}", registry_name);
            if let Some(metadata) = &repo_config.metadata {
                if let Some(license) = &metadata.license {
                    println!("License: {}", license);
                }
                if !metadata.authors.is_empty() {
                    println!("Authors: {}", metadata.authors.join(", "));
                }
                if !metadata.tags.is_empty() {
                    println!("Tags: {}", metadata.tags.join(", "));
                }
            }
        }
        Err(e) => {
            log::warn!("Could not find package in registry: {}", e);
        }
    }

    Ok(())
}
