use crate::cli::args::ValidateArgs;
use crate::config::repo::RepoConfig;
use anyhow::{Context, Result};

pub async fn execute(args: ValidateArgs) -> Result<()> {
    println!("🔍 Validating .repo file: {}", args.repo_file);

    // Read file
    let content = tokio::fs::read_to_string(&args.repo_file)
        .await
        .context("Failed to read .repo file")?;

    // Parse TOML
    let repo_config: RepoConfig = toml::from_str(&content).context("Failed to parse .repo file")?;

    // Basic validations
    println!("✓ Valid TOML syntax");
    println!("✓ Package name: {}", repo_config.name);
    println!("✓ Description: {}", repo_config.description);

    // Check required fields
    if repo_config.name.is_empty() {
        anyhow::bail!("Package name cannot be empty");
    }

    if repo_config.description.is_empty() {
        anyhow::bail!("Description cannot be empty");
    }

    // Check source config
    println!("✓ Source type: {:?}", repo_config.source.provider_type);

    // Check binaries
    if repo_config.install.binaries.is_empty() {
        println!("⚠  Warning: No binaries specified");
    } else {
        println!("✓ Binaries: {:?}", repo_config.install.binaries);
    }

    // Check security
    if repo_config.security.allow_insecure {
        println!("⚠  Warning: Package allows insecure installation");
    }

    if let Some(checksum) = &repo_config.security.checksum {
        println!("✓ Checksum algorithm: {:?}", checksum.algorithm);
        println!("✓ Checksum format: {:?}", checksum.format);
    } else if !repo_config.security.allow_insecure {
        anyhow::bail!("No checksum configured and allow_insecure is false");
    }

    // Check platform config
    if let Some(platform) = &repo_config.platform {
        if !platform.os_map.is_empty() {
            println!("✓ OS mappings: {:?}", platform.os_map);
        }
        if !platform.arch_map.is_empty() {
            println!("✓ Arch mappings: {:?}", platform.arch_map);
        }
    }

    // Check version discovery
    if let Some(version_config) = &repo_config.source.version {
        println!("✓ Version discovery configured");
        println!("  - Type: {:?}", version_config.discovery_type);
        println!("  - URL: {}", version_config.discovery_url);
    }

    println!("\n✅ Validation successful! The .repo file is valid.");

    Ok(())
}
