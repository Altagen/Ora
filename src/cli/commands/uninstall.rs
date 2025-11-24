use anyhow::{Context, Result};
use std::path::Path;

use crate::cli::args::UninstallArgs;
use crate::security::AuditLogger;
use crate::storage::database::{load_installed_db, save_installed_db};

pub async fn execute(args: UninstallArgs) -> Result<()> {
    log::info!("Uninstalling package: {}", args.package);

    // Load installed database
    let mut db = load_installed_db().await?;

    // Find installed package
    let installed = db
        .packages
        .get(&args.package)
        .context(format!("Package '{}' is not installed", args.package))?
        .clone();

    // If version specified, check it matches
    if let Some(ver) = &args.version {
        if &installed.version != ver {
            anyhow::bail!(
                "Package '{}' version {} is not installed (installed: {})",
                args.package,
                ver,
                installed.version
            );
        }
    }

    log::info!("Removing files...");

    // Remove symlinks first
    for symlink in &installed.symlinks {
        let path = Path::new(symlink);
        if path.exists() || path.is_symlink() {
            std::fs::remove_file(symlink)
                .context(format!("Failed to remove symlink: {}", symlink))?;
            log::debug!("Removed symlink: {}", symlink);
        }
    }

    // Remove install directory
    let install_dir = Path::new(&installed.install_dir);
    if install_dir.exists() {
        std::fs::remove_dir_all(install_dir).context(format!(
            "Failed to remove directory: {}",
            installed.install_dir
        ))?;
        log::debug!("Removed directory: {}", installed.install_dir);

        // Clean up parent directory if it's empty
        if let Some(parent_dir) = install_dir.parent() {
            if parent_dir.exists() {
                match std::fs::read_dir(parent_dir) {
                    Ok(mut entries) => {
                        // Check if directory is empty
                        if entries.next().is_none() {
                            if let Err(e) = std::fs::remove_dir(parent_dir) {
                                log::debug!(
                                    "Could not remove empty parent directory {:?}: {}",
                                    parent_dir,
                                    e
                                );
                            } else {
                                log::debug!("Removed empty parent directory: {:?}", parent_dir);
                            }
                        }
                    }
                    Err(e) => {
                        log::debug!("Could not check parent directory {:?}: {}", parent_dir, e);
                    }
                }
            }
        }
    }

    // Remove from database
    db.packages.remove(&args.package);
    save_installed_db(&db).await?;

    // Audit log
    AuditLogger::new()?
        .log_uninstall(&args.package, &installed.version, true)
        .await?;

    println!(
        "✓ Successfully uninstalled {} v{}",
        args.package, installed.version
    );

    Ok(())
}
