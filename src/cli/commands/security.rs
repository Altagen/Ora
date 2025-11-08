use crate::cli::args::{SecurityArgs, SecurityCommand};
use crate::config::SecurityConfig;
use anyhow::Result;

pub async fn execute(args: SecurityArgs) -> Result<()> {
    match args.command {
        SecurityCommand::Init => init_config().await,
        SecurityCommand::Show => show_config().await,
        SecurityCommand::Reset => reset_config().await,
    }
}

/// Initialize security configuration file with defaults
async fn init_config() -> Result<()> {
    let config = SecurityConfig::default();

    // Check if config already exists
    if let Ok(config_path) = crate::storage::paths::Paths::config_dir() {
        let security_config_path = config_path.join("security.toml");
        if security_config_path.exists() {
            println!("⚠️  Security configuration already exists at:");
            println!("   {}", security_config_path.display());
            println!("\nUse 'ora security reset' to overwrite with defaults.");
            return Ok(());
        }
    }

    config.save()?;

    let config_path = crate::storage::paths::Paths::config_dir()?;
    let security_config_path = config_path.join("security.toml");

    println!("✓ Created security configuration at:");
    println!("   {}", security_config_path.display());
    println!("\nYou can now customize security settings by editing this file.");
    println!("\nKey settings to review:");
    println!("  • network.https_only: Enforce HTTPS-only downloads");
    println!("  • scripts.require_confirmation: Require approval for post-install scripts");
    println!("  • registries.require_checksums_public: Require checksums from public registries");
    println!("  • validation.templates.url_encode_variables: Prevent template injection");
    println!("\nRun 'ora security show' to view current settings.");

    Ok(())
}

/// Show current security configuration
async fn show_config() -> Result<()> {
    let config = SecurityConfig::load()?;

    println!("📋 Current Security Configuration\n");
    println!("═══════════════════════════════════════════════════════════");

    // Network Security
    println!("\n🌐 Network Security:");
    println!(
        "  HTTPS Only:              {}",
        format_bool(config.network.https_only)
    );
    println!(
        "  Allow Redirects:         {}",
        format_bool(config.network.allow_redirects)
    );
    println!(
        "  Block Private IPs:       {}",
        format_bool(config.network.block_private_ips)
    );
    println!(
        "  Block Localhost:         {}",
        format_bool(config.network.block_localhost)
    );
    println!(
        "  Validate DNS:            {}",
        format_bool(config.network.validate_dns_resolution)
    );
    println!(
        "  Max Download:            {}",
        format_size(config.network.max_download_size)
    );
    println!(
        "  Timeout:                 {}s",
        config.network.timeout_seconds
    );

    // Git Security
    println!("\n  Git Protocol:");
    println!(
        "    HTTPS Only:            {}",
        format_bool(config.network.git.https_only)
    );
    println!(
        "    Max Repo Size:         {}",
        format_size(config.network.git.max_repo_size)
    );
    println!(
        "    Allow Force Checkout:  {}",
        format_bool(config.network.git.allow_force_checkout)
    );

    // Extraction Security
    println!("\n📦 Extraction Security:");
    println!(
        "  Max File Size:           {}",
        format_size(config.extraction.max_file_size)
    );
    println!(
        "  Max Total Size:          {}",
        format_size(config.extraction.max_total_size)
    );
    println!(
        "  Max File Count:          {}",
        config.extraction.max_file_count
    );
    println!(
        "  Max Directory Depth:     {}",
        config.extraction.max_directory_depth
    );
    println!(
        "  Block Symlinks:          {}",
        format_bool(config.extraction.block_symlinks)
    );
    println!(
        "  Strip SETUID Bits:       {}",
        format_bool(config.extraction.strip_setuid_bits)
    );

    // Script Security
    println!("\n📜 Script Security:");
    println!(
        "  Scripts Enabled:         {}",
        format_bool(config.scripts.enabled)
    );
    println!(
        "  Require Confirmation:    {}",
        format_bool(config.scripts.require_confirmation)
    );
    println!(
        "  Show Script Content:     {}",
        format_bool(config.scripts.show_script_content)
    );
    println!(
        "  Static Analysis:         {}",
        format_bool(config.scripts.static_analysis)
    );
    println!(
        "  Block Public Scripts:    {}",
        format_bool(config.scripts.block_public_registry_scripts)
    );
    println!(
        "  Timeout:                 {}s",
        config.scripts.timeout_seconds
    );

    // Registry Security
    println!("\n🗃️  Registry Security:");
    println!(
        "  Enforce Trust Levels:    {}",
        format_bool(config.registries.enforce_trust_levels)
    );
    println!(
        "  Require Checksums (Pub): {}",
        format_bool(config.registries.require_checksums_public)
    );
    println!(
        "  Require GPG Signatures:  {}",
        format_bool(config.registries.require_gpg_signatures)
    );
    println!(
        "  Allow Pkg Shadowing:     {}",
        format_bool(config.registries.allow_package_shadowing)
    );
    println!(
        "  Max Registry Size:       {}",
        format_size(config.registries.max_registry_size)
    );

    // Validation Security
    println!("\n✅ Input Validation:");
    println!(
        "  Max TOML Size:           {}",
        format_size(config.validation.max_toml_size)
    );
    println!(
        "  Max JSON Size:           {}",
        format_size(config.validation.max_json_size)
    );
    println!(
        "  URL Encode Templates:    {}",
        format_bool(config.validation.templates.url_encode_variables)
    );
    println!(
        "  Block Path Traversal:    {}",
        format_bool(config.validation.templates.block_path_traversal)
    );
    println!(
        "  Max Regex Size:          {}",
        format_size(config.validation.regex.max_compiled_size as u64)
    );

    // Resource Limits
    println!("\n⚙️  Resource Limits:");
    println!(
        "  Limits Enabled:          {}",
        format_bool(config.resources.enabled)
    );
    println!(
        "  Max Concurrent DLs:      {}",
        config.resources.max_concurrent_downloads
    );
    println!(
        "  Max Cache Size:          {}",
        format_size(config.resources.max_cache_size_bytes)
    );

    println!("\n═══════════════════════════════════════════════════════════");

    let config_path = crate::storage::paths::Paths::config_dir()?;
    let security_config_path = config_path.join("security.toml");
    println!(
        "\n📁 Configuration file: {}",
        security_config_path.display()
    );
    println!("   Edit this file to customize security settings.");

    Ok(())
}

/// Reset security configuration to defaults
async fn reset_config() -> Result<()> {
    let config = SecurityConfig::default();
    config.save()?;

    let config_path = crate::storage::paths::Paths::config_dir()?;
    let security_config_path = config_path.join("security.toml");

    println!("✓ Reset security configuration to defaults:");
    println!("   {}", security_config_path.display());
    println!("\nRun 'ora security show' to view current settings.");

    Ok(())
}

// Helper functions for formatting
fn format_bool(value: bool) -> String {
    if value {
        "✓ Yes".to_string()
    } else {
        "✗ No".to_string()
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
