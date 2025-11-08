use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::cli::args::ConfigArgs;
use crate::config::{GlobalConfig, SecurityConfig};
use crate::storage::database::{load_global_config, load_installed_db};
use crate::storage::paths::Paths;

pub async fn execute(args: ConfigArgs) -> Result<()> {
    match args.command {
        crate::cli::args::ConfigCommand::Show => show_config().await,
        crate::cli::args::ConfigCommand::Verify => verify_config().await,
        crate::cli::args::ConfigCommand::Init => init_config().await,
    }
}

/// Show current configuration paths and status
async fn show_config() -> Result<()> {
    println!("📋 Ora Configuration Status\n");

    // Show environment variable overrides
    println!("🔧 Environment Variables:");
    if let Ok(config_dir) = std::env::var("ORA_CONFIG_DIR") {
        println!("   ORA_CONFIG_DIR = {} (override active)", config_dir);
    } else {
        println!("   ORA_CONFIG_DIR = (not set, using default)");
    }
    if let Ok(data_dir) = std::env::var("ORA_DATA_DIR") {
        println!("   ORA_DATA_DIR   = {} (override active)", data_dir);
    } else {
        println!("   ORA_DATA_DIR   = (not set, using default)");
    }
    if let Ok(cache_dir) = std::env::var("ORA_CACHE_DIR") {
        println!("   ORA_CACHE_DIR  = {} (override active)", cache_dir);
    } else {
        println!("   ORA_CACHE_DIR  = (not set, using default)");
    }
    println!();

    // Show configuration file paths
    println!("📁 Configuration Files:");

    // Global config
    let global_config_path = Paths::config_file()?;
    print_file_status("Global Config", &global_config_path);

    // Security config
    if let Ok(config_dir) = Paths::config_dir() {
        let security_config_path = config_dir.join("security.toml");
        print_file_status("Security Config", &security_config_path);
    }

    // Installed database
    let installed_db_path = Paths::installed_db_file()?;
    print_file_status("Installed Packages DB", &installed_db_path);

    println!();

    // Show data directories
    println!("📂 Data Directories:");
    if let Ok(config_dir) = Paths::config_dir() {
        print_dir_status("Config", &config_dir);
    }
    if let Ok(data_dir) = Paths::data_dir() {
        print_dir_status("Data", &data_dir);
    }
    if let Ok(cache_dir) = Paths::cache_dir() {
        print_dir_status("Cache", &cache_dir);
    }
    println!();

    // Try to load configs and show status
    println!("📊 Configuration Status:");

    // Global config
    match load_global_config().await {
        Ok(config) => {
            println!("   ✓ Global Config: Loaded successfully");
            println!("     ├─ Registries: {}", config.registries.len());
            let enabled_count = config.registries.iter().filter(|r| r.enabled).count();
            println!("     └─ Enabled: {}", enabled_count);
        }
        Err(e) => {
            println!("   ✗ Global Config: Failed to load");
            println!("     └─ Error: {}", e);
        }
    }

    // Security config
    match SecurityConfig::load() {
        Ok(_config) => {
            println!("   ✓ Security Config: Loaded successfully");
        }
        Err(e) => {
            println!("   ✗ Security Config: Failed to load");
            println!("     └─ Error: {}", e);
        }
    }

    // Installed database
    match load_installed_db().await {
        Ok(db) => {
            println!("   ✓ Installed Packages: {} package(s)", db.packages.len());
        }
        Err(e) => {
            println!("   ✗ Installed Packages: Failed to load");
            println!("     └─ Error: {}", e);
        }
    }

    println!();
    println!("💡 Tip: Run 'ora config verify' to check for issues");

    Ok(())
}

/// Verify all configuration files are readable and valid
async fn verify_config() -> Result<()> {
    println!("🔍 Verifying Ora Configuration...\n");

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check environment variables
    println!("Checking environment variables...");
    if let Ok(config_dir) = std::env::var("ORA_CONFIG_DIR") {
        let path = PathBuf::from(&config_dir);
        if !path.exists() {
            warnings.push(format!(
                "ORA_CONFIG_DIR points to non-existent directory: {}",
                config_dir
            ));
        } else if !path.is_dir() {
            errors.push(format!("ORA_CONFIG_DIR is not a directory: {}", config_dir));
        }
    }
    println!("   ✓ Environment variables checked\n");

    // Verify global config
    println!("Verifying global configuration...");
    let global_config_path = Paths::config_file()?;
    match load_global_config().await {
        Ok(config) => {
            println!("   ✓ Global config is valid");

            // Check for empty registries
            if config.registries.is_empty() {
                warnings.push(
                    "No registries configured. Add one with: ora registry add <name> <url>"
                        .to_string(),
                );
            }

            // Check for disabled registries
            let disabled_count = config.registries.iter().filter(|r| !r.enabled).count();
            if disabled_count > 0 && disabled_count == config.registries.len() {
                warnings.push("All registries are disabled".to_string());
            }
        }
        Err(e) => {
            if global_config_path.exists() {
                errors.push(format!("Global config is invalid: {}", e));
            } else {
                warnings.push("Global config doesn't exist (will use defaults)".to_string());
            }
        }
    }
    println!();

    // Verify security config
    println!("Verifying security configuration...");
    if let Ok(config_dir) = Paths::config_dir() {
        let security_config_path = config_dir.join("security.toml");
        match SecurityConfig::load() {
            Ok(_) => {
                println!("   ✓ Security config is valid");
            }
            Err(e) => {
                if security_config_path.exists() {
                    errors.push(format!("Security config is invalid: {}", e));
                } else {
                    warnings.push(
                        "Security config doesn't exist (using defaults). Run: ora security init"
                            .to_string(),
                    );
                }
            }
        }
    }
    println!();

    // Verify installed database
    println!("Verifying installed packages database...");
    let installed_db_path = Paths::installed_db_file()?;
    match load_installed_db().await {
        Ok(_) => {
            println!("   ✓ Installed packages database is valid");
        }
        Err(e) => {
            if installed_db_path.exists() {
                errors.push(format!("Installed packages database is invalid: {}", e));
            } else {
                println!("   ℹ No packages installed yet");
            }
        }
    }
    println!();

    // Check directory permissions
    println!("Checking directory permissions...");
    if let Ok(config_dir) = Paths::config_dir() {
        check_directory_writable(&config_dir, "Config directory", &mut errors);
    }
    if let Ok(data_dir) = Paths::data_dir() {
        check_directory_writable(&data_dir, "Data directory", &mut errors);
    }
    if let Ok(cache_dir) = Paths::cache_dir() {
        check_directory_writable(&cache_dir, "Cache directory", &mut errors);
    }
    println!("   ✓ Directory permissions checked\n");

    // Print summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    if errors.is_empty() && warnings.is_empty() {
        println!("✅ All checks passed! Configuration is healthy.");
    } else {
        if !errors.is_empty() {
            println!("❌ Found {} error(s):", errors.len());
            for error in &errors {
                println!("   • {}", error);
            }
            println!();
        }

        if !warnings.is_empty() {
            println!("⚠️  Found {} warning(s):", warnings.len());
            for warning in &warnings {
                println!("   • {}", warning);
            }
            println!();
        }

        if !errors.is_empty() {
            anyhow::bail!(
                "Configuration verification failed with {} error(s)",
                errors.len()
            );
        }
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

/// Initialize all configuration files with defaults
async fn init_config() -> Result<()> {
    println!("🚀 Initializing Ora Configuration...\n");

    // Ensure directories exist
    println!("Creating directories...");
    Paths::ensure_directories()?;
    println!("   ✓ Directories created\n");

    // Initialize global config
    println!("Initializing global configuration...");
    let global_config_path = Paths::config_file()?;
    if global_config_path.exists() {
        println!(
            "   ℹ Global config already exists at: {}",
            global_config_path.display()
        );
        println!("   (not overwriting)");
    } else {
        let default_config = GlobalConfig::default();
        crate::storage::database::save_global_config(&default_config).await?;
        println!("   ✓ Created: {}", global_config_path.display());
    }
    println!();

    // Initialize security config
    println!("Initializing security configuration...");
    if let Ok(config_dir) = Paths::config_dir() {
        let security_config_path = config_dir.join("security.toml");
        if security_config_path.exists() {
            println!(
                "   ℹ Security config already exists at: {}",
                security_config_path.display()
            );
            println!("   (not overwriting)");
            println!("   To reset, use: ora security reset");
        } else {
            let default_config = SecurityConfig::default();
            default_config.save()?;
            println!("   ✓ Created: {}", security_config_path.display());
        }
    }
    println!();

    // Summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Configuration initialized successfully!");
    println!();
    println!("📋 Next steps:");
    println!("   1. Create a .repo file for your package");
    println!("      See: docs/CREATING_REPO_FILES.md");
    println!();
    println!("   2. Install packages:");
    println!("      ora install --repo ./package.repo");
    println!();
    println!("   3. View your configuration:");
    println!("      ora config show");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

/// Helper function to print file status
fn print_file_status(label: &str, path: &PathBuf) {
    let status = if path.exists() {
        if path.is_file() {
            let metadata = std::fs::metadata(path).ok();
            let size = metadata
                .map(|m| format_size(m.len()))
                .unwrap_or_else(|| "?".to_string());
            format!("✓ exists ({})", size)
        } else {
            "✗ exists but not a file".to_string()
        }
    } else {
        "ℹ not created yet (will use defaults)".to_string()
    };

    println!("   {:20} {} - {}", label, status, path.display());
}

/// Helper function to print directory status
fn print_dir_status(label: &str, path: &Path) {
    let status = if path.exists() {
        if path.is_dir() {
            "✓ exists".to_string()
        } else {
            "✗ exists but not a directory".to_string()
        }
    } else {
        "ℹ not created yet".to_string()
    };

    println!("   {:20} {} - {}", label, status, path.display());
}

/// Helper function to check if directory is writable
fn check_directory_writable(path: &PathBuf, label: &str, errors: &mut Vec<String>) {
    if !path.exists() {
        // Try to create it
        if let Err(e) = std::fs::create_dir_all(path) {
            errors.push(format!("{} cannot be created: {}", label, e));
        }
        return;
    }

    // Test write by creating a temp file
    let test_file = path.join(".ora_write_test");
    match std::fs::write(&test_file, b"test") {
        Ok(_) => {
            let _ = std::fs::remove_file(&test_file);
        }
        Err(e) => {
            errors.push(format!("{} is not writable: {}", label, e));
        }
    }
}

/// Format bytes as human-readable size
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;

    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    }
}
