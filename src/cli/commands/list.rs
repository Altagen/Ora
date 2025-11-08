use anyhow::Result;

use crate::cli::args::ListArgs;
use crate::storage::database::load_installed_db;

pub async fn execute(args: ListArgs) -> Result<()> {
    let db = load_installed_db().await?;

    if db.packages.is_empty() {
        println!("No packages installed");
        return Ok(());
    }

    println!("Installed packages:");
    println!();

    for (name, pkg) in &db.packages {
        if args.verbose {
            println!("Package: {}", name);
            println!("  Version: {}", pkg.version);
            println!("  Installed: {}", pkg.installed_at);
            println!("  Mode: {}", pkg.install_mode);
            println!("  Directory: {}", pkg.install_dir);
            println!("  Symlinks: {}", pkg.symlinks.len());
            println!();
        } else {
            println!("  {} @ {} ({})", name, pkg.version, pkg.install_mode);
        }
    }

    Ok(())
}
