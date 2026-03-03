# Getting Started with Ora

This guide will help you get Ora up and running in minutes.

## Table of Contents

- [Installation](#installation)
- [First Steps](#first-steps)
- [Adding a Registry](#adding-a-registry)
- [Installing Your First Package](#installing-your-first-package)
- [Managing Packages](#managing-packages)
- [Next Steps](#next-steps)

---

## Installation

### Option 1: Pre-built Binaries (Recommended)

Download the latest release from [GitHub Releases](https://github.com/Altagen/Ora/releases).

#### Linux

Three package formats are available for Linux (amd64 and arm64):

**Debian / Ubuntu — `.deb`**
```bash
# Download and install
curl -LO https://github.com/Altagen/Ora/releases/latest/download/ora-VERSION-linux-amd64.deb
sudo dpkg -i ora-VERSION-linux-amd64.deb
```

**Fedora / RHEL / openSUSE — `.rpm`**
```bash
# Download and install
curl -LO https://github.com/Altagen/Ora/releases/latest/download/ora-VERSION-linux-amd64.rpm
sudo rpm -i ora-VERSION-linux-amd64.rpm
# or: sudo dnf install ./ora-VERSION-linux-amd64.rpm
```

**Any Linux distribution — `.tar.gz`**
```bash
curl -LO https://github.com/Altagen/Ora/releases/latest/download/ora-VERSION-linux-amd64.tar.gz
tar xzf ora-VERSION-linux-amd64.tar.gz
mkdir -p ~/.local/bin
mv ora-VERSION-linux-amd64 ~/.local/bin/ora
```

Replace `VERSION` with the latest release tag (e.g. `0.2.5`) and `amd64` with `arm64` for ARM machines.

#### macOS

Two formats are available for macOS (amd64 and arm64).

**Installer package — `.pkg`**

Installs `ora` to `/usr/local/bin` (requires admin password).

> **Ora is not signed with an Apple Developer certificate.** macOS Gatekeeper will block
> the installer by default. Remove the quarantine attribute before running it:
>
> ```bash
> xattr -d com.apple.quarantine ora-VERSION-macos-amd64.pkg
> ```
>
> Without this step, macOS will refuse to open the package. To avoid this entirely,
> use the `.tar.gz` option below — it has no quarantine restrictions when extracted
> via the terminal.

```bash
# Intel (amd64)
curl -LO https://github.com/Altagen/Ora/releases/latest/download/ora-VERSION-macos-amd64.pkg
xattr -d com.apple.quarantine ora-VERSION-macos-amd64.pkg
sudo installer -pkg ora-VERSION-macos-amd64.pkg -target /

# Apple Silicon (arm64)
curl -LO https://github.com/Altagen/Ora/releases/latest/download/ora-VERSION-macos-arm64.pkg
xattr -d com.apple.quarantine ora-VERSION-macos-arm64.pkg
sudo installer -pkg ora-VERSION-macos-arm64.pkg -target /
```

**Archive — `.tar.gz`** *(zero friction)*

Extracted via `tar` in the terminal, the binary carries no quarantine attribute and
works immediately without any extra step.

```bash
# Intel (amd64)
curl -LO https://github.com/Altagen/Ora/releases/latest/download/ora-VERSION-macos-amd64.tar.gz
tar xzf ora-VERSION-macos-amd64.tar.gz
mkdir -p ~/.local/bin
mv ora-VERSION-macos-amd64 ~/.local/bin/ora

# Apple Silicon (arm64)
curl -LO https://github.com/Altagen/Ora/releases/latest/download/ora-VERSION-macos-arm64.tar.gz
tar xzf ora-VERSION-macos-arm64.tar.gz
mkdir -p ~/.local/bin
mv ora-VERSION-macos-arm64 ~/.local/bin/ora
```

Make sure `~/.local/bin` is in your `PATH`:
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### Verify checksums

All release artifacts are covered by `SHA256SUMS.txt`:
```bash
curl -LO https://github.com/Altagen/Ora/releases/latest/download/SHA256SUMS.txt
sha256sum -c SHA256SUMS.txt --ignore-missing
```

---

### Option 2: Using Cargo

```bash
cargo install --git https://github.com/Altagen/Ora
```

### Option 3: From Source

```bash
git clone https://github.com/Altagen/Ora
cd Ora
cargo build --release
mkdir -p ~/.local/bin
cp target/release/ora ~/.local/bin/
```

### Verify Installation

```bash
ora --version
# Output: ora 0.2.5
```

---

## First Steps

### 1. Initialize Configuration

Ora will create configuration files automatically, but you can initialize them explicitly:

```bash
ora config init
```

This creates:
- `~/.config/ora/config.toml` - Global configuration
- `~/.config/ora/security.toml` - Security policies
- `~/.config/ora/installed.toml` - Installed packages database

### 2. View Configuration

```bash
ora config show
```

Output shows:
- Configuration file locations
- Registered repositories
- Security settings
- Cache and data directories

### 3. Check Configuration Status

```bash
ora config verify
```

This validates all configuration files for syntax errors.

---

## Adding a Registry

Ora uses **git-based registries** to store package definitions. A registry is a git repository containing `.repo` files.

### What is a Registry?

A registry is a git repository that contains `.repo` files. Each `.repo` file describes:
- Where to download a package (GitHub, GitLab, direct URL)
- How to verify it (checksums)
- What binaries to install
- Platform-specific variants

### Registry Structure

A registry must have this structure:

```
my-registry/
└── ora-registry/       # Required directory
    ├── ripgrep.repo
    ├── fd.repo
    └── jq.repo
```

**Important**: The `ora-registry/` directory is required. Ora searches for `.repo` files in this directory.

### How Search Works

After adding a registry:
1. `ora registry sync` clones/pulls the git repository to `~/.cache/ora/registries/<name>/`
2. `ora search <query>` searches in the local `.repo` files (no server required)
3. Search only works after syncing registries

Example:
```bash
ora registry add my-tools https://github.com/me/ora-tools.git
ora registry sync        # Downloads the registry
ora search ripgrep       # Searches in local .repo files
```

### Adding Your First Registry

```bash
# Add a registry from a git repository
ora registry add my-registry https://github.com/username/ora-registry.git

# Example with a real registry (if available)
ora registry add official https://github.com/Altagen/Ora-packages.git
```

### List Registries

```bash
ora registry list
```

Output:
```
Configured registries:
  • my-registry
    URL: https://github.com/username/ora-registry.git
```

### Sync Registry

After adding a registry, sync it to download the latest package definitions:

**Sync all registries:**
```bash
ora registry sync
```

**Sync a specific registry:**
```bash
ora registry sync my-registry
```

Example output:
```
Syncing 2 registries...
  → Syncing 'main-registry'...
    ✓ Synced successfully
  → Syncing 'my-registry'...
    ✓ Synced successfully

Sync complete!
```

This clones or updates registered repositories from their remote sources.

### Verify Registry

Verify the integrity and validity of a registry:

```bash
ora registry verify my-registry
```

This performs comprehensive validation including:
1. **Configuration Check** - Registry exists in config
2. **Local Sync Check** - Registry has been synced locally
3. **Git Repository Validation** - Valid git repository with correct remote URL
4. **Structure Validation** - Required `ora-registry/` directory exists
5. **Package Count** - Lists available `.repo` files

Example output:
```
Verifying registry: my-registry

✓ Registry found in configuration
  Name: my-registry
  URL: https://github.com/username/ora-registry.git
  Trust Level: Public
  Enabled: true
✓ Registry synced locally
  Path: "/home/user/.cache/ora/registries/my-registry"
✓ Valid git repository
  Commit: 61e1b655924b03daf00ff578c600655b850f6610
  Remote: https://github.com/username/ora-registry.git
✓ 'ora-registry/' directory exists
✓ Found 3 package definitions
  1. ripgrep
  2. bat
  3. fd

✓ Registry 'my-registry' verification complete!
```

**Common issues:**
- **Registry not synced**: Run `ora registry sync <name>` first
- **Invalid git repository**: Re-sync or remove and re-add the registry
- **Missing ora-registry/ directory**: Registry may be misconfigured

### Remove a Registry

```bash
ora registry remove my-registry
```

---

## Installing Your First Package

### Search for Packages

Search looks in your **locally synced registries** (no internet required after sync):

```bash
# Search for packages
ora search ripgrep

# Search with partial name
ora search rip
```

**Note**: You must run `ora registry sync` first to download the registry contents.

Output:
```
Found 1 package(s):

ripgrep
  Fast line-oriented search tool
  Versions: 14.1.0, 14.0.3, 13.0.0
  Source: GitHub (BurntSushi/ripgrep)
```

### Get Package Information

```bash
ora info ripgrep
```

Output shows:
- Description
- Available versions
- Download source
- Supported platforms
- Checksum verification method

### Install a Package

```bash
# Install latest version
ora install ripgrep

# Install specific version
ora install ripgrep --version 14.1.0
```

The installation process:
1. ✅ Finds package definition in registry
2. ✅ Detects your OS and architecture
3. ✅ Downloads the appropriate binary
4. ✅ Verifies checksum
5. ✅ Extracts binaries
6. ✅ Installs to `~/.local/bin/` (by default)
7. ✅ Adds to package database

### Installing from a .repo File

If you have a `.repo` file locally:

```bash
ora install --repo ./mypackage.repo
```

---

## Managing Packages

### List Installed Packages

```bash
ora list
```

Output:
```
Installed packages:

ripgrep 14.1.0
  Installed: 2025-11-08
  Binary: ~/.local/bin/rg
  Registry: my-registry

fd 9.0.0
  Installed: 2025-11-08
  Binaries: ~/.local/bin/fd
  Registry: my-registry
```

### Update Packages

```bash
# Update all packages
ora update

# Update specific package
ora update ripgrep
```

### Uninstall Packages

```bash
ora uninstall ripgrep
```

This removes:
- The installed binaries
- The package from the database

---

## Installing from Local Archives

You can install a package from a local archive without downloading from the internet.

### Requirements

1. **Archive file**: A `.tar.gz` containing your binaries
2. **Metadata file**: A `.toml` file describing the package

### Metadata File Format

Create a file (e.g., `metadata.toml`):

```toml
name = "mypackage"
version = "1.0.0"
binaries = ["mybin", "another-bin"]
description = "Optional description"  # Optional
```

**Required fields:**
- `name`: Package name (must not be empty)
- `version`: Package version (must not be empty)
- `binaries`: List of binary files to install from the archive

### Installation Command

```bash
ora install mypackage --local ./archive.tar.gz --metadata ./metadata.toml
```

### Example

Let's say you have a local tool:

```bash
# 1. Create your archive
tar czf mytool-1.0.0.tar.gz mytool

# 2. Create metadata.toml
cat > metadata.toml << EOF
name = "mytool"
version = "1.0.0"
binaries = ["mytool"]
description = "My custom tool"
EOF

# 3. Install
ora install mytool --local ./mytool-1.0.0.tar.gz --metadata ./metadata.toml
```

The binary will be installed to `~/.local/bin/mytool` (by default).

### Use Cases

- Installing proprietary software
- Testing packages before publishing
- Internal tools not suitable for public registries
- Air-gapped environments

---

## Directory Structure

Ora uses standard XDG directories:

```
~/.config/ora/          # Configuration
├── config.toml         # Global config (registries)
├── security.toml       # Security policies
└── installed.toml      # Package database

~/.local/share/ora/     # Data
└── registries/         # Cloned registry repositories
    ├── my-registry/
    └── official/

~/.cache/ora/           # Cache
└── downloads/          # Temporary downloads

~/.local/bin/           # Installed binaries (default)
├── rg                  # ripgrep
├── fd                  # fd-find
└── ...
```

### Override Directories

Use environment variables:

```bash
export ORA_CONFIG_DIR=~/custom/config
export ORA_DATA_DIR=~/custom/data
export ORA_CACHE_DIR=~/custom/cache
export ORA_BIN_DIR=~/custom/bin

ora install ripgrep
```

---

## Creating Your Own Registry

Want to create a registry for your organization?

### 1. Create a Git Repository

```bash
mkdir ora-packages
cd ora-packages
git init
```

### 2. Create the ora-registry/ Directory

Ora requires an `ora-registry/` directory:

```bash
mkdir ora-registry
```

### 3. Add .repo Files

Create `.repo` files in the `ora-registry/` directory. See [Creating .repo Files](CREATING_REPO_FILES.md).

Example `ora-registry/ripgrep.repo`:

```toml
name = "ripgrep"

[source]
type = "github-releases"
repo = "BurntSushi/ripgrep"

[source.download]
url = "https://github.com/BurntSushi/ripgrep/releases/download/{version}/ripgrep-{version}-{platform}.tar.gz"

[source.platforms.linux_x86_64]
url = "https://github.com/BurntSushi/ripgrep/releases/download/{version}/ripgrep-{version}-x86_64-unknown-linux-musl.tar.gz"

[source.checksums]
type = "sha256-single-file"
url = "https://github.com/BurntSushi/ripgrep/releases/download/{version}/ripgrep-{version}-x86_64-unknown-linux-musl.tar.gz.sha256"

[install]
binaries = ["rg"]
```

### 4. Commit and Push

```bash
git add .
git commit -m "Add ripgrep package"
git push origin main
```

### 5. Use Your Registry

```bash
ora registry add my-packages https://github.com/you/ora-packages.git
ora registry sync
ora install ripgrep
```

---

## Security Best Practices

Ora is designed with security in mind. By default:

✅ **Checksums required** - All downloads must have checksums
✅ **HTTPS only** - Git URLs must use HTTPS (configurable)
✅ **SSRF protection** - Blocks private IPs and cloud metadata endpoints
✅ **Path traversal protection** - Prevents malicious archive extraction
✅ **Zip bomb protection** - Limits extraction size
✅ **Size limits** - Configurable max download sizes

### Allow Insecure Package (Not Recommended)

Some packages don't provide checksums. To install them:

```toml
# In package.repo file
[source.checksums]
type = "none"
allow_insecure = true  # Required to bypass checksum verification
```

**Warning**: Only use `allow_insecure = true` for trusted sources!

### Configure Security

Edit `~/.config/ora/security.toml`:

```toml
[network]
max_download_size = 1073741824  # 1GB
request_timeout = 120
max_redirects = 5

[network.git]
https_only = true  # Enforce HTTPS for git URLs
allowed_schemes = ["https", "http"]

[validation]
max_archive_size = 1073741824  # 1GB
```

See [Security Configuration](SECURITY_CONFIGURATION.md) for all options.

---

## Common Workflows

### Installing Multiple Packages

```bash
# Method 1: One by one
ora install ripgrep
ora install fd
ora install jq

# Method 2: From a list
cat packages.txt | xargs -n1 ora install
```

### Updating All Packages

```bash
# Update all installed packages
ora update

# Update command will check and install newer versions
```

### Backup Your Installation

```bash
# Backup package database
cp ~/.config/ora/installed.toml ~/ora-backup.toml

# Restore
cp ~/ora-backup.toml ~/.config/ora/installed.toml
```

---

## Troubleshooting

### Package Not Found

```
Error: Package 'xyz' not found in any registry
```

**Solution**:
1. Check package name: `ora search xyz`
2. Sync registries: `ora registry sync`
3. Add more registries: `ora registry add ...`

### Checksum Verification Failed

```
Error: Checksum verification failed
```

**Solution**:
1. This is a security feature - the download may be corrupted or tampered
2. Check if the package maintainer updated checksums
3. Report to package maintainer if persistent

### Network Errors

```
Error: Failed to download
```

**Solution**:
1. Check internet connection
2. Check if URL is accessible: `curl -I <url>`
3. Check proxy settings
4. Increase timeout in `~/.config/ora/security.toml`

### Permission Errors

```
Error: Permission denied
```

**Solution**:
1. Don't use `sudo` with Ora
2. Install to user directory: Default is `~/.local/bin`
3. Ensure `~/.local/bin` exists: `mkdir -p ~/.local/bin`
4. Add to PATH: `export PATH="$HOME/.local/bin:$PATH"`

### Binary Not in PATH

After installation, the binary isn't found:

```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export PATH="$HOME/.local/bin:$PATH"

# Reload shell
source ~/.bashrc
```

---

## Next Steps

Now that you're familiar with the basics:

1. **Read the docs**:
   - [Creating .repo Files](CREATING_REPO_FILES.md) - Create package definitions
   - [.repo Schema Reference](REPO_SCHEMA.md) - Complete schema
   - [Security Configuration](SECURITY_CONFIGURATION.md) - Security options

2. **Create your own registry** with packages for your organization

3. **Contribute** to the project:
   - Report bugs: [GitHub Issues](https://github.com/Altagen/Ora/issues)
   - Suggest features: [GitHub Discussions](https://github.com/Altagen/Ora/discussions)
   - Submit PRs: [Contributing Guide](../README.md#contributing)

4. **Share** your experience and help others!

---

## Quick Reference

```bash
# Registry Management
ora registry add <name> <git-url>     # Add registry
ora registry list                     # List registries
ora registry sync                     # Update registries
ora registry remove <name>            # Remove registry

# Package Installation
ora search <query>                    # Search packages
ora info <package>                    # Package info
ora install <package>                 # Install package
ora install --repo <file>             # Install from .repo file
ora install <pkg> --version <ver>     # Install specific version

# Package Management
ora list                              # List installed
ora update                            # Update all packages
ora update <package>                  # Update one package
ora uninstall <package>               # Uninstall package

# Configuration
ora config show                       # Show configuration
ora config verify                     # Verify config files
ora config init                       # Initialize config

# Help
ora --help                            # General help
ora <command> --help                  # Command help
```

---

Happy package managing! 🎉
