use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::signal;

mod cli;
mod config;
mod error;
mod installer;
mod providers;
mod registry;
mod security;
mod storage;
mod utils;

use cli::{Cli, Commands};

/// Global shutdown flag accessible to all operations
pub static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Check if shutdown has been requested
pub fn is_shutdown_requested() -> bool {
    SHUTDOWN_REQUESTED.load(Ordering::Relaxed)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments early to get verbosity flags
    let cli = Cli::parse();

    // Initialize logger based on verbosity flags
    // Default: WARN (only show warnings and errors, no timestamps)
    // --verbose: INFO (show info messages with timestamps)
    // --debug: DEBUG (show debug messages with timestamps)
    let log_level = if cli.debug {
        "debug"
    } else if cli.verbose {
        "info"
    } else {
        "warn"
    };

    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or(log_level));

    // Configure timestamp based on verbosity
    if !cli.debug && !cli.verbose {
        // Default mode: no timestamps for cleaner user-facing output
        builder.format_timestamp(None);
    }

    builder.init();

    // Set up graceful shutdown handler for SIGINT (Ctrl+C) and SIGTERM
    tokio::spawn(async {
        #[cfg(unix)]
        {
            let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to create SIGTERM handler");

            tokio::select! {
                _ = signal::ctrl_c() => {
                    log::warn!("🛑 Received SIGINT (Ctrl+C), initiating graceful shutdown...");
                }
                _ = sigterm.recv() => {
                    log::warn!("🛑 Received SIGTERM, initiating graceful shutdown...");
                }
            }
        }

        #[cfg(not(unix))]
        {
            signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
            log::warn!("🛑 Received Ctrl+C, initiating graceful shutdown...");
        }

        SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);

        log::info!("Cleaning up temporary files...");

        // Attempt to clean up cache downloads
        if let Err(e) = storage::cache::Cache::clear_downloads() {
            log::warn!("Failed to clean up downloads during shutdown: {}", e);
        }

        log::info!("✓ Graceful shutdown complete");
        std::process::exit(130); // Exit code 130 = terminated by Ctrl+C
    });

    // Execute command
    let result = match cli.command {
        Commands::Install(args) => cli::commands::install::execute(args).await,
        Commands::Uninstall(args) => cli::commands::uninstall::execute(args).await,
        Commands::Update(args) => cli::commands::update::execute(args).await,
        Commands::List(args) => cli::commands::list::execute(args).await,
        Commands::Search(args) => cli::commands::search::execute(args).await,
        Commands::Info(args) => cli::commands::info::execute(args).await,
        Commands::Registry(args) => cli::commands::registry::execute(args).await,
        Commands::Validate(args) => cli::commands::validate::execute(args).await,
        Commands::Security(args) => cli::commands::security::execute(args).await,
        Commands::Config(args) => cli::commands::config::execute(args).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);

        // Clean up on error
        log::info!("Cleaning up after error...");
        if let Err(cleanup_err) = storage::cache::Cache::clear_downloads() {
            log::warn!("Failed to clean up downloads: {}", cleanup_err);
        }

        std::process::exit(1);
    }

    Ok(())
}
