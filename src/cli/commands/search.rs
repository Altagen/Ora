use anyhow::Result;

use crate::cli::args::SearchArgs;
use crate::registry::RegistryIndex;
use crate::storage::database::load_global_config;

pub async fn execute(args: SearchArgs) -> Result<()> {
    log::info!("Searching for: {}", args.query);

    let config = load_global_config().await?;

    if config.registries.is_empty() {
        println!("No registries configured. Add a registry with 'ora registry add'");
        return Ok(());
    }

    let mut found_any = false;

    for registry in &config.registries {
        if !registry.enabled {
            continue;
        }

        match RegistryIndex::search_packages(&registry.name, &args.query).await {
            Ok(packages) => {
                if !packages.is_empty() {
                    found_any = true;
                    println!("\nRegistry: {}", registry.name);
                    for pkg in packages {
                        println!("  - {}", pkg);
                    }
                }
            }
            Err(e) => {
                log::warn!("Error searching registry '{}': {}", registry.name, e);
            }
        }
    }

    if !found_any {
        println!("No packages found matching '{}'", args.query);
    }

    Ok(())
}
