/// Configuration Migration System
///
/// Handles automatic migration of config files across versions.
/// Config version follows Ora's major version (e.g., Ora 0.x → config 0.y)
use anyhow::Result;

use crate::config::global::GlobalConfig;
use crate::config::installed::{InstalledDatabase, InstalledPackage};

/// Current config version for GlobalConfig (Ora 0.2.2)
pub const CURRENT_CONFIG_VERSION: &str = "0.1";

/// Current schema version for InstalledPackage (Ora 0.2.2)
pub const CURRENT_SCHEMA_VERSION: &str = "0.1";

/// Migrate global config to latest version
///
/// This function only supports forward migrations (upgrades).
/// Downgrades are rejected with a clear error message.
pub fn migrate_global_config(config: &mut GlobalConfig) -> Result<()> {
    let from_version = if config.config_version.is_empty() {
        "0.0" // Old config without versioning
    } else {
        &config.config_version
    };

    if from_version == CURRENT_CONFIG_VERSION {
        return Ok(()); // Already up to date
    }

    // Detect downgrade: config version is newer than what we support
    if is_version_newer(from_version, CURRENT_CONFIG_VERSION) {
        anyhow::bail!(
            "Config version {} is newer than what this Ora version supports ({}).\n\
             \n\
             You are running an older version of Ora than what created this config.\n\
             \n\
             Solution:\n\
             └─ Upgrade Ora to the latest version: ora self-update",
            from_version,
            CURRENT_CONFIG_VERSION
        );
    }

    log::info!(
        "Migrating global config from {} to {}",
        from_version,
        CURRENT_CONFIG_VERSION
    );

    // Migration chain (upgrades only)
    match from_version {
        "0.0" => {
            migrate_config_v0_0_to_v0_1(config)?;
            config.config_version = CURRENT_CONFIG_VERSION.to_string();
        }
        // Future migrations:
        // "0.1" => {
        //     migrate_config_v0_1_to_v0_2(config)?;
        //     config.config_version = CURRENT_CONFIG_VERSION.to_string();
        // }
        v => {
            anyhow::bail!(
                "Unknown config version: {}. Expected a version <= {}",
                v,
                CURRENT_CONFIG_VERSION
            );
        }
    }

    Ok(())
}

/// Migration from v0.0 (no version field) to v0.1
fn migrate_config_v0_0_to_v0_1(_config: &mut GlobalConfig) -> Result<()> {
    log::debug!("Applying migration: config 0.0 → 0.1");

    // v0.0 → v0.1: Just adding version field and future-ready fields
    // All existing fields are compatible with #[serde(default)]

    // New fields added in 0.1:
    // - config_version: String (this migration adds it)
    // - aliases: HashMap<String, String> (defaults to empty)
    // - registry.priority: Option<u8> (defaults to None)

    log::debug!("Migration 0.0 → 0.1 complete (no structural changes needed)");
    Ok(())
}

/// Migrate installed package to latest schema
///
/// This function only supports forward migrations (upgrades).
/// Downgrades are rejected with a clear error message.
pub fn migrate_installed_package(package: &mut InstalledPackage) -> Result<()> {
    let from_version = if package.schema_version.is_empty() {
        "0.0"
    } else {
        &package.schema_version.clone()
    };

    if from_version == CURRENT_SCHEMA_VERSION {
        return Ok(());
    }

    // Detect downgrade: schema version is newer than what we support
    if is_version_newer(from_version, CURRENT_SCHEMA_VERSION) {
        anyhow::bail!(
            "Package '{}' has schema version {} which is newer than supported ({}).\n\
             \n\
             You are running an older version of Ora.\n\
             \n\
             Solution:\n\
             └─ Upgrade Ora to the latest version: ora self-update",
            package.name,
            from_version,
            CURRENT_SCHEMA_VERSION
        );
    }

    log::debug!(
        "Migrating package '{}' from schema {} to {}",
        package.name,
        from_version,
        CURRENT_SCHEMA_VERSION
    );

    match from_version {
        "0.0" => {
            migrate_package_v0_0_to_v0_1(package)?;
            package.schema_version = CURRENT_SCHEMA_VERSION.to_string();
        }
        // Future migrations:
        // "0.1" => {
        //     migrate_package_v0_1_to_v0_2(package)?;
        //     package.schema_version = CURRENT_SCHEMA_VERSION.to_string();
        // }
        v => {
            anyhow::bail!(
                "Unknown schema version: {}. Expected a version <= {}",
                v,
                CURRENT_SCHEMA_VERSION
            );
        }
    }

    Ok(())
}

/// Migration from schema v0.0 to v0.1
fn migrate_package_v0_0_to_v0_1(package: &mut InstalledPackage) -> Result<()> {
    log::debug!(
        "Applying migration: package '{}' schema 0.0 → 0.1",
        package.name
    );

    // v0.0 → v0.1: Adding schema_version and metadata field
    // All existing fields remain compatible

    // New fields in 0.1:
    // - schema_version: String (this migration adds it)
    // - metadata: HashMap<String, String> (defaults to empty)
    // - allow_insecure: bool (already has #[serde(default)])

    log::debug!("Migration 0.0 → 0.1 complete for '{}'", package.name);
    Ok(())
}

/// Migrate entire installed database
pub fn migrate_installed_database(db: &mut InstalledDatabase) -> Result<()> {
    let mut migrated_count = 0;

    for package in db.packages.values_mut() {
        let old_version = package.schema_version.clone();
        migrate_installed_package(package)?;

        if package.schema_version != old_version {
            migrated_count += 1;
        }
    }

    if migrated_count > 0 {
        log::info!("Migrated {} installed package(s)", migrated_count);
    }

    Ok(())
}

/// Check if version `a` is newer than version `b`
///
/// Compares versions in "x.y" format.
/// Returns true if a > b, false otherwise.
fn is_version_newer(a: &str, b: &str) -> bool {
    let parse = |v: &str| -> (u32, u32) {
        let parts: Vec<u32> = v.split('.').filter_map(|s| s.parse().ok()).collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
        )
    };

    parse(a) > parse(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::global::GlobalConfig;
    use crate::config::installed::{InstalledDatabase, InstalledPackage};
    use chrono::Utc;
    use std::collections::HashMap;

    #[test]
    fn test_migrate_config_v0_0_to_v0_1() {
        // Simulate old config without version field
        let mut config = GlobalConfig {
            config_version: "0.0".to_string(),
            ..Default::default()
        };

        migrate_global_config(&mut config).unwrap();

        assert_eq!(config.config_version, "0.1");
    }

    #[test]
    fn test_migrate_config_already_current() {
        let mut config = GlobalConfig {
            config_version: "0.1".to_string(),
            ..Default::default()
        };

        let result = migrate_global_config(&mut config);
        assert!(result.is_ok());
        assert_eq!(config.config_version, "0.1");
    }

    #[test]
    fn test_migrate_package_v0_0_to_v0_1() {
        let mut package = InstalledPackage {
            schema_version: "0.0".to_string(),
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            installed_at: Utc::now(),
            install_mode: "userland".to_string(),
            install_dir: "/test/dir".to_string(),
            files: vec![],
            symlinks: vec![],
            registry_source: "test".to_string(),
            checksums: HashMap::new(),
            allow_insecure: false,
            metadata: HashMap::new(),
        };

        migrate_installed_package(&mut package).unwrap();

        assert_eq!(package.schema_version, "0.1");
        assert_eq!(package.name, "test-package"); // Data preserved
    }

    #[test]
    fn test_migrate_installed_database() {
        let mut db = InstalledDatabase::default();

        // Add package with old schema
        let package = InstalledPackage {
            schema_version: "0.0".to_string(),
            name: "pkg1".to_string(),
            version: "1.0.0".to_string(),
            installed_at: Utc::now(),
            install_mode: "userland".to_string(),
            install_dir: "/test".to_string(),
            files: vec![],
            symlinks: vec![],
            registry_source: "test".to_string(),
            checksums: HashMap::new(),
            allow_insecure: false,
            metadata: HashMap::new(),
        };

        db.packages.insert("pkg1".to_string(), package);

        migrate_installed_database(&mut db).unwrap();

        assert_eq!(db.packages.get("pkg1").unwrap().schema_version, "0.1");
    }

    #[test]
    fn test_downgrade_config_rejected() {
        let mut config = GlobalConfig {
            config_version: "0.2".to_string(), // Newer than CURRENT (0.1)
            ..Default::default()
        };

        let result = migrate_global_config(&mut config);
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("is newer than what this Ora version supports"));
        assert!(error_msg.contains("ora self-update"));
    }

    #[test]
    fn test_downgrade_package_rejected() {
        let mut package = InstalledPackage {
            schema_version: "0.2".to_string(), // Newer than CURRENT (0.1)
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            installed_at: Utc::now(),
            install_mode: "userland".to_string(),
            install_dir: "/test/dir".to_string(),
            files: vec![],
            symlinks: vec![],
            registry_source: "test".to_string(),
            checksums: HashMap::new(),
            allow_insecure: false,
            metadata: HashMap::new(),
        };

        let result = migrate_installed_package(&mut package);
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("is newer than supported"));
    }

    #[test]
    fn test_is_version_newer() {
        // Newer versions
        assert!(is_version_newer("0.2", "0.1"));
        assert!(is_version_newer("1.0", "0.9"));
        assert!(is_version_newer("0.10", "0.9"));
        assert!(is_version_newer("2.0", "1.5"));

        // Same or older versions
        assert!(!is_version_newer("0.1", "0.1"));
        assert!(!is_version_newer("0.0", "0.1"));
        assert!(!is_version_newer("0.1", "0.2"));
        assert!(!is_version_newer("1.0", "2.0"));
    }

    #[test]
    fn test_unsupported_config_version() {
        let mut config = GlobalConfig {
            config_version: "99.0".to_string(),
            ..Default::default()
        };

        let result = migrate_global_config(&mut config);
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("is newer than what this Ora version supports"));
    }
}
