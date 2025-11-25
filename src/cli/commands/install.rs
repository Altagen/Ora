use anyhow::{Context, Result};
use chrono::Utc;
use std::path::Path;

use crate::cli::args::InstallArgs;
use crate::config::global::InstallMode;
use crate::config::installed::InstalledPackage;
use crate::config::local_metadata::LocalMetadata;
use crate::config::repo::RepoConfig;
use crate::config::GlobalConfig;
use crate::installer::{run_post_install, Deployer, Downloader, Extractor, Verifier};
use crate::providers::create_provider;
use crate::registry::RegistryManager;
use crate::security::{AuditLogger, SecurityWarningManager};
use crate::storage::cache::Cache;
use crate::storage::database::{load_global_config, load_installed_db, save_installed_db};
use crate::utils::platform::{default_arch_mapping, default_os_mapping, Platform};

pub async fn execute(args: InstallArgs) -> Result<()> {
    // Check if this is a local installation
    if args.local.is_some() {
        return execute_local_install(args).await;
    }

    log::info!("Installing package: {}", args.package);

    // Parse package name and optional registry (package@registry syntax)
    let (package_name, registry_name) = if args.package.contains('@') {
        let parts: Vec<&str> = args.package.splitn(2, '@').collect();
        (parts[0].to_string(), Some(parts[1].to_string()))
    } else {
        (args.package.clone(), None)
    };

    // Determine install mode
    let install_mode = if args.system {
        InstallMode::System
    } else {
        InstallMode::Userland
    };

    // Check if already installed
    let mut db = load_installed_db().await?;
    if db.packages.contains_key(&package_name) {
        log::warn!("Package '{}' is already installed", package_name);
        println!(
            "Package '{}' is already installed. Use 'ora update' to upgrade.",
            package_name
        );
        return Ok(());
    }

    // Load repo config and track registry source
    let (repo_config, registry_source) = if let Some(repo_file) = &args.repo {
        // Load from file
        let content = tokio::fs::read_to_string(repo_file)
            .await
            .context("Failed to read .repo file")?;
        let config =
            toml::from_str::<RepoConfig>(&content).context("Failed to parse .repo file")?;
        (config, format!("file:{}", repo_file))
    } else if let Some(registry) = registry_name {
        // Find in specific registry
        let (config, reg_name) =
            RegistryManager::find_package_in_specific_registry(&package_name, &registry).await?;
        (config, format!("registry:{}", reg_name))
    } else {
        // Find in any registry
        let (config, reg_name) = RegistryManager::find_package(&package_name).await?;
        (config, format!("registry:{}", reg_name))
    };

    // Check security warnings BEFORE starting installation
    // SECURITY: Do NOT silently ignore config load failures - they could be permission issues
    // that prevent security settings from being applied
    let global_config = match load_global_config().await {
        Ok(config) => config,
        Err(e) => {
            log::warn!("Failed to load global configuration: {}", e);
            eprintln!("⚠️  Warning: Could not load configuration file");
            eprintln!("   {}", e);
            eprintln!("   Using default configuration (this may affect security settings)");
            eprintln!();
            GlobalConfig::default()
        }
    };
    SecurityWarningManager::check_and_warn(&repo_config, args.allow_insecure, &global_config)?;

    // Detect platform
    let platform = Platform::detect();

    // Apply mappings (use helper methods for v1/v2 compatibility)
    let os_mapping = {
        let map = repo_config.get_os_map();
        if map.is_empty() {
            default_os_mapping()
        } else {
            map
        }
    };

    let arch_mapping = {
        let map = repo_config.get_arch_map();
        if map.is_empty() {
            default_arch_mapping()
        } else {
            map
        }
    };

    let mapped_os = platform.map_os(&os_mapping);
    let mapped_arch = platform.map_arch(&arch_mapping);

    log::info!(
        "Platform: {} ({}), Arch: {} ({})",
        platform.os,
        mapped_os,
        platform.arch,
        mapped_arch
    );

    // Create provider and get version
    let provider = create_provider(&repo_config)?;

    let version = if let Some(v) = &args.version {
        v.clone()
    } else {
        // Get latest non-prerelease version
        let versions = provider.list_versions().await?;
        let latest = versions
            .iter()
            .filter(|v| !v.prerelease)
            .max_by(|a, b| {
                // Use semver comparison if possible, fallback to string comparison
                match (
                    semver::Version::parse(&a.tag),
                    semver::Version::parse(&b.tag),
                ) {
                    (Ok(v_a), Ok(v_b)) => v_a.cmp(&v_b),
                    _ => a.tag.cmp(&b.tag),
                }
            })
            .context("No versions available")?;
        latest.tag.clone()
    };

    log::info!("Installing version: {}", version);

    // Get download URL
    let download_url = provider
        .get_download_url(&version, &mapped_os, &mapped_arch)
        .await?;

    log::info!("Download URL: {}", download_url);

    // Download
    // Strip trailing slashes from URL before extracting filename
    let url_without_trailing_slash = download_url.trim_end_matches('/');
    let filename = url_without_trailing_slash
        .split('/')
        .next_back()
        .context("Invalid download URL")?;

    // Validate that we got a non-empty filename
    if filename.is_empty() {
        anyhow::bail!(
            "Failed to extract filename from download URL: {}\n\
             The URL appears to be invalid. Please check the .repo file's download URL template.",
            download_url
        );
    }

    let download_path = Cache::download_path(filename)?;

    let downloader = Downloader::new()?;
    downloader.download(&download_url, &download_path).await?;

    // Verify
    let verifier = Verifier::new()?;
    verifier
        .verify(
            &download_path,
            &repo_config,
            &version,
            &mapped_os,
            &mapped_arch,
            args.allow_insecure,
        )
        .await?;

    // Extract
    let extract_dir = Cache::download_path(&format!("{}_extract", args.package))?;
    Extractor::extract(&download_path, &extract_dir)?;

    // Deploy
    let deployment = Deployer::deploy(
        &extract_dir,
        &repo_config.install,
        install_mode.clone(),
        &args.package,
        &version,
    )?;

    // Run post-install script if configured
    if let Some(post_install_script) = &repo_config.install.post_install {
        let install_dir = std::path::PathBuf::from(&deployment.install_dir);
        run_post_install(
            post_install_script,
            &install_dir,
            &version,
            &repo_config.install.env,
            args.allow_insecure,
        )
        .await?;
    }

    // Update installed database
    let installed_package = InstalledPackage {
        name: package_name.clone(),
        version: version.clone(),
        installed_at: Utc::now(),
        install_mode: format!("{:?}", install_mode).to_lowercase(),
        install_dir: deployment.install_dir.clone(),
        files: deployment.files,
        symlinks: deployment.symlinks,
        registry_source: registry_source.clone(),
        checksums: Default::default(),
        allow_insecure: args.allow_insecure,
    };

    db.packages.insert(package_name.clone(), installed_package);
    save_installed_db(&db).await?;

    // Audit log
    AuditLogger::new()?
        .log_install(&package_name, &version, &registry_source, true)
        .await?;

    println!("✓ Successfully installed {} {}", package_name, version);

    Ok(())
}

async fn execute_local_install(args: InstallArgs) -> Result<()> {
    let archive_path = args
        .local
        .as_ref()
        .context("Local archive path is required but was not provided")?;

    // Require metadata file
    let metadata_path = args
        .metadata
        .as_ref()
        .context("--metadata is required for local installations")?;

    log::info!("Installing from local archive: {}", archive_path);

    // Load and validate metadata
    let metadata_content = tokio::fs::read_to_string(metadata_path)
        .await
        .context("Failed to read metadata file")?;
    let metadata: LocalMetadata =
        toml::from_str(&metadata_content).context("Failed to parse metadata file")?;
    metadata.validate()?;

    // Determine install mode
    let install_mode = if args.system {
        InstallMode::System
    } else {
        InstallMode::Userland
    };

    // Check if already installed
    let mut db = load_installed_db().await?;
    if db.packages.contains_key(&metadata.name) {
        log::warn!("Package '{}' is already installed", metadata.name);
        println!(
            "Package '{}' is already installed. Uninstall it first or use a different name.",
            metadata.name
        );
        return Ok(());
    }

    // Copy archive to cache and extract
    let archive_path = Path::new(archive_path);
    if !archive_path.exists() {
        anyhow::bail!("Archive file not found: {}", archive_path.display());
    }

    let cache_archive_path = Cache::download_path(&format!(
        "{}-{}-local.tar.gz",
        metadata.name, metadata.version
    ))?;
    tokio::fs::copy(archive_path, &cache_archive_path)
        .await
        .context("Failed to copy archive to cache")?;

    // Extract archive
    let extract_dir = Cache::download_path(&format!("{}_extract", metadata.name))?;
    Extractor::extract(&cache_archive_path, &extract_dir)?;

    // Deploy package
    let install_config = crate::config::repo::InstallConfig {
        mode: None, // Mode is passed separately to Deployer::deploy
        binaries: metadata.binaries.clone(),
        files: vec![],
        post_install: None,
        env: Default::default(),
    };

    let deployment = Deployer::deploy(
        &extract_dir,
        &install_config,
        install_mode.clone(),
        &metadata.name,
        &metadata.version,
    )?;

    // Update installed database
    let installed_package = InstalledPackage {
        name: metadata.name.clone(),
        version: metadata.version.clone(),
        installed_at: Utc::now(),
        install_mode: format!("{:?}", install_mode).to_lowercase(),
        install_dir: deployment.install_dir.clone(),
        files: deployment.files,
        symlinks: deployment.symlinks,
        registry_source: format!("local:{}", archive_path.display()),
        checksums: Default::default(),
        allow_insecure: args.allow_insecure,
    };

    db.packages.insert(metadata.name.clone(), installed_package);
    save_installed_db(&db).await?;

    // Audit log
    AuditLogger::new()?
        .log_install(&metadata.name, &metadata.version, "local", true)
        .await?;

    println!(
        "✓ Successfully installed {} {} from local archive",
        metadata.name, metadata.version
    );

    Ok(())
}
