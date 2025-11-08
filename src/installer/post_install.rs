use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command as TokioCommand;
use tokio::time::timeout;

pub async fn run_post_install(
    script: &str,
    install_dir: &Path,
    version: &str,
    custom_env: &HashMap<String, String>,
    allow_without_confirmation: bool,
) -> Result<()> {
    log::warn!("⚠️  SECURITY WARNING: Package contains post-install script");
    log::warn!("Post-install scripts can execute arbitrary code on your system");

    // Show the script to the user
    println!("\n⚠️  SECURITY WARNING ⚠️");
    println!("This package contains a post-install script that will execute on your system.");
    println!("Post-install scripts can run arbitrary commands with your user permissions.");
    println!("\n📜 Script content:");
    println!("─────────────────────────────────────────────────────────");
    println!("{}", script);
    println!("─────────────────────────────────────────────────────────");

    // Request explicit user confirmation unless --insecure flag was used
    if !allow_without_confirmation {
        println!("\n❓ Do you want to run this post-install script? [y/N]");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read user input")?;

        let confirmed =
            input.trim().eq_ignore_ascii_case("y") || input.trim().eq_ignore_ascii_case("yes");

        if !confirmed {
            log::info!("Post-install script execution cancelled by user");
            println!("⚠️  Post-install script skipped. Package may not function correctly.");
            return Ok(());
        }
    } else {
        log::warn!("Skipping user confirmation due to --insecure flag");
    }

    log::info!("Running post-install script with user approval");

    // Prepare environment
    let mut env_vars = HashMap::new();
    env_vars.insert(
        "INSTALL_DIR".to_string(),
        install_dir.to_string_lossy().to_string(),
    );
    env_vars.insert("VERSION".to_string(), version.to_string());

    // Add custom env vars
    for (key, value) in custom_env {
        let resolved_value = resolve_env_value(value, install_dir, version);
        env_vars.insert(key.clone(), resolved_value);
    }

    // Load security configuration to get timeout setting
    let security_config = crate::config::SecurityConfig::load().unwrap_or_default();
    let timeout_duration = Duration::from_secs(security_config.scripts.timeout_seconds);

    log::info!(
        "Executing post-install script with timeout of {} seconds",
        security_config.scripts.timeout_seconds
    );

    // Execute command with timeout enforcement
    let mut cmd = TokioCommand::new("sh");
    cmd.arg("-c")
        .arg(script)
        .current_dir(install_dir)
        .envs(&env_vars)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output_future = cmd.output();

    let output = match timeout(timeout_duration, output_future).await {
        Ok(result) => result.context("Failed to execute post-install script")?,
        Err(_) => {
            log::error!(
                "❌ Post-install script exceeded timeout of {} seconds",
                security_config.scripts.timeout_seconds
            );
            anyhow::bail!(
                "Post-install script timed out after {} seconds. \
                 \n\nThis timeout is configured in security.toml: \
                 \n  [scripts] \
                 \n  timeout_seconds = {} \
                 \n\nTo increase the timeout, edit ~/.config/ora/security.toml",
                security_config.scripts.timeout_seconds,
                security_config.scripts.timeout_seconds
            );
        }
    };

    // Log output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        log::info!("Post-install stdout:\n{}", stdout);
    }

    if !stderr.is_empty() {
        log::warn!("Post-install stderr:\n{}", stderr);
    }

    if !output.status.success() {
        log::warn!(
            "Post-install script exited with code: {:?}",
            output.status.code()
        );
        // Don't fail the installation, just warn
    }

    Ok(())
}

fn resolve_env_value(value: &str, install_dir: &Path, version: &str) -> String {
    value
        .replace("{install_dir}", &install_dir.to_string_lossy())
        .replace("{version}", version)
}
