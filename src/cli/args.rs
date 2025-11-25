use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ora")]
#[command(about = "Omni Repository for Archives - Decentralized package manager")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output (INFO level logs)
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Enable debug output (DEBUG level logs, implies --verbose)
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a package
    Install(InstallArgs),

    /// Uninstall a package
    Uninstall(UninstallArgs),

    /// Update package(s)
    Update(UpdateArgs),

    /// List installed packages
    List(ListArgs),

    /// Search for packages in registries
    Search(SearchArgs),

    /// Show package information
    Info(InfoArgs),

    /// Manage registries
    Registry(RegistryArgs),

    /// Validate a .repo file
    Validate(ValidateArgs),

    /// Manage security configuration
    Security(SecurityArgs),

    /// Manage configuration files
    Config(ConfigArgs),
}

#[derive(clap::Args)]
pub struct InstallArgs {
    pub package: String,

    #[arg(short, long)]
    pub version: Option<String>,

    #[arg(long)]
    pub repo: Option<String>,

    #[arg(long)]
    pub userland: bool,

    #[arg(long)]
    pub system: bool,

    #[arg(long, name = "allow-insecure")]
    pub allow_insecure: bool,

    /// Install from a local tar.gz archive
    #[arg(long)]
    pub local: Option<String>,

    /// Metadata file for local installation (.toml)
    #[arg(long)]
    pub metadata: Option<String>,
}

#[derive(clap::Args)]
pub struct ValidateArgs {
    /// Path to the .repo file to validate
    pub repo_file: String,
}

#[derive(clap::Args)]
pub struct UninstallArgs {
    pub package: String,

    #[arg(short, long)]
    pub version: Option<String>,

    #[arg(long)]
    pub purge: bool,
}

#[derive(clap::Args)]
pub struct UpdateArgs {
    pub package: Option<String>,

    #[arg(long)]
    pub all: bool,
}

#[derive(clap::Args)]
pub struct ListArgs {
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(clap::Args)]
pub struct SearchArgs {
    pub query: String,
}

#[derive(clap::Args)]
pub struct InfoArgs {
    pub package: String,
}

#[derive(clap::Args)]
pub struct RegistryArgs {
    #[command(subcommand)]
    pub command: RegistryCommand,
}

#[derive(Subcommand)]
pub enum RegistryCommand {
    Add {
        name: String,
        url: String,
        #[arg(long, default_value = "public")]
        trust_level: String,
        #[arg(long)]
        ca_cert: Option<String>,
        #[arg(long)]
        pin_cert: bool,
        /// Git branch to use for this registry (optional, defaults to repository's default branch)
        #[arg(long)]
        branch: Option<String>,
    },
    List {
        #[arg(short, long)]
        verbose: bool,
    },
    Remove {
        name: String,
    },
    /// Sync registries (download/update package definitions)
    Sync {
        /// Optional registry name to sync (syncs all if not specified)
        name: Option<String>,
    },
    Verify {
        name: String,
    },
    UpdatePin {
        name: String,
    },
}

#[derive(clap::Args)]
pub struct SecurityArgs {
    #[command(subcommand)]
    pub command: SecurityCommand,
}

#[derive(Subcommand)]
pub enum SecurityCommand {
    /// Initialize security configuration file with defaults
    Init,

    /// Show current security configuration
    Show,

    /// Reset security configuration to defaults
    Reset,
}

#[derive(clap::Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Show configuration paths and status
    Show,

    /// Verify all configuration files are valid
    Verify,

    /// Initialize all configuration files with defaults
    Init,
}
