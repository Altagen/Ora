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
        log::info!("Checking updates for {}...", package_name);

        let installed = match db.packages.get(&package_name) {
            Some(pkg) => pkg,
            None => {
                log::warn!("Package '{}' not installed, skipping", package_name);
                continue;
            }
        };

        // Find .repo in registries
        let (_repo_config, _registry_name) =
            match RegistryManager::find_package(&package_name).await {
                Ok(result) => result,
                Err(e) => {
                    log::warn!("Could not find package '{}': {}", package_name, e);
                    continue;
                }
            };

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
            repo: None,
            userland: installed.install_mode == "userland",
            system: installed.install_mode == "system",
            allow_insecure: false,
            local: None,
            metadata: None,
        };
        install::execute(install_args).await?;
    }

    Ok(())
}
