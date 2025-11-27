use anyhow::Result;

use crate::cli::args::{RegistryArgs, RegistryCommand};
use crate::registry::RegistryManager;

pub async fn execute(args: RegistryArgs) -> Result<()> {
    match args.command {
        RegistryCommand::Add {
            name,
            url,
            trust_level,
            ca_cert,
            pin_cert,
            branch,
            dir,
        } => {
            RegistryManager::add_registry(name, url, trust_level, ca_cert, pin_cert, branch, dir)
                .await?;
        }
        RegistryCommand::List { verbose } => {
            RegistryManager::list_registries(verbose).await?;
        }
        RegistryCommand::Remove { name } => {
            RegistryManager::remove_registry(name).await?;
        }
        RegistryCommand::Sync { name } => {
            RegistryManager::sync_registries(name).await?;
        }
        RegistryCommand::Verify { name } => {
            RegistryManager::verify_registry(name).await?;
        }
        RegistryCommand::UpdatePin { name } => {
            log::warn!("Certificate pinning update not yet fully implemented");
            println!("Updating certificate pin for: {}", name);
            // TODO: Implement certificate pin update
        }
    }

    Ok(())
}
