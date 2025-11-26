use anyhow::{Context, Result};

use crate::config::{GlobalConfig, InstalledDatabase};
use crate::storage::paths::Paths;
use crate::utils::io_errors::read_file_user_friendly_async;

/// Load global configuration with user-friendly error messages
///
/// **SECURITY NOTE**: This function will FAIL with clear error message if config
/// file exists but is unreadable (e.g., permission denied). It will NOT silently
/// fall back to defaults, which could bypass security settings.
pub async fn load_global_config() -> Result<GlobalConfig> {
    let path = Paths::config_file()?;

    // Try to read config file with user-friendly error messages
    match read_file_user_friendly_async(&path).await? {
        Some(content) => {
            // File exists and was read successfully - parse it
            match toml::from_str::<GlobalConfig>(&content) {
                Ok(mut config) => {
                    log::info!("✅ Loaded global configuration from {}", path.display());

                    // Auto-migrate config if needed
                    let old_version = config.config_version.clone();
                    if let Err(e) = crate::config::migrations::migrate_global_config(&mut config) {
                        log::warn!("Failed to migrate config: {}", e);
                        return Err(e);
                    }

                    // Save migrated config back to disk
                    if config.config_version != old_version {
                        log::info!(
                            "Config migrated from {} to {}",
                            old_version,
                            config.config_version
                        );
                        save_global_config(&config).await?;
                    }

                    Ok(config)
                }
                Err(e) => {
                    anyhow::bail!(
                        "Failed to parse global configuration: {}\n\
                         \n\
                         File: {}\n\
                         Error: {}\n\
                         \n\
                         Fix this by:\n\
                         ├─ Checking TOML syntax: https://toml.io\n\
                         ├─ Running: ora config verify\n\
                         └─ Resetting to defaults: ora config reset",
                        path.display(),
                        path.display(),
                        e
                    );
                }
            }
        }
        None => {
            // File doesn't exist - use defaults (this is OK for first run)
            log::info!("No global configuration found at {}", path.display());
            log::info!("Using default configuration");
            log::info!("Add registries with: ora registry add <name> <url>");
            Ok(GlobalConfig::default())
        }
    }
}

/// Save global configuration with user-friendly error messages
pub async fn save_global_config(config: &GlobalConfig) -> Result<()> {
    use crate::utils::io_errors::write_file_user_friendly_async;

    Paths::ensure_directories()?;
    let path = Paths::config_file()?;

    let content = toml::to_string_pretty(config)
        .context("Failed to serialize global configuration to TOML")?;

    // Write config file with user-friendly errors (handles permission denied, disk full, etc.)
    write_file_user_friendly_async(&path, content.as_bytes()).await?;

    log::info!("✅ Saved global configuration to {}", path.display());
    Ok(())
}

/// Load installed packages database with user-friendly error messages
pub async fn load_installed_db() -> Result<InstalledDatabase> {
    let path = Paths::installed_db_file()?;

    // Try to read database file with user-friendly error messages
    match read_file_user_friendly_async(&path).await? {
        Some(content) => {
            // File exists and was read successfully - parse it
            match toml::from_str::<InstalledDatabase>(&content) {
                Ok(mut db) => {
                    log::debug!(
                        "✅ Loaded installed packages database from {}",
                        path.display()
                    );

                    // Auto-migrate installed packages if needed
                    let needs_save = db
                        .packages
                        .values()
                        .any(|p| p.schema_version.is_empty() || p.schema_version == "0.0");

                    if let Err(e) = crate::config::migrations::migrate_installed_database(&mut db) {
                        log::warn!("Failed to migrate installed database: {}", e);
                        return Err(e);
                    }

                    // Save migrated database back to disk if any package was migrated
                    if needs_save {
                        log::info!("Migrated installed packages database");
                        save_installed_db(&db).await?;
                    }

                    Ok(db)
                }
                Err(e) => {
                    anyhow::bail!(
                        "Failed to parse installed packages database: {}\n\
                         \n\
                         File: {}\n\
                         Error: {}\n\
                         \n\
                         This database tracks which packages are installed.\n\
                         If it's corrupted, you may need to:\n\
                         ├─ Backup the file\n\
                         ├─ Fix the TOML syntax\n\
                         └─ Or delete it (you'll lose track of installed packages)",
                        path.display(),
                        path.display(),
                        e
                    );
                }
            }
        }
        None => {
            // File doesn't exist - return empty database (first run)
            log::debug!("No installed packages database found at {}", path.display());
            log::debug!("Starting with empty database");
            Ok(InstalledDatabase::default())
        }
    }
}

/// Save installed packages database with user-friendly error messages
pub async fn save_installed_db(db: &InstalledDatabase) -> Result<()> {
    use crate::utils::io_errors::write_file_user_friendly_async;

    Paths::ensure_directories()?;
    let path = Paths::installed_db_file()?;

    let content = toml::to_string_pretty(db)
        .context("Failed to serialize installed packages database to TOML")?;

    // Write database file with user-friendly errors
    write_file_user_friendly_async(&path, content.as_bytes()).await?;

    log::debug!("✅ Saved installed packages database to {}", path.display());
    Ok(())
}
