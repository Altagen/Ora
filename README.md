<div align="center">
  <img src="assets/ora-icon.png" alt="Ora Logo" width="200"/>

  # Ora - Omni Repository for Archives

  A secure, decentralized package manager for pre-compiled binaries

  [![CI](https://github.com/Altagen/Ora/workflows/CI/badge.svg)](https://github.com/Altagen/Ora/actions)
  [![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
  [![Version](https://img.shields.io/badge/version-0.2.0-green.svg)](https://github.com/Altagen/Ora/releases)
</div>

---

## What is Ora

Ora is a package manager for installing and managing pre-compiled binaries from diverse sources including GitHub Releases, GitLab, custom APIs, webpage scraping, and direct URLs. It runs in userland by default (no root required), supports both decentralized git-based registries and direct HTTPS registries, and enforces comprehensive security configurations to protect against supply chain attacks.

Key features:

- **Multi-source support**: GitHub Releases, GitLab, custom APIs, webpage scraping, direct URLs
- **Dual registry modes**: Git-based collections or direct HTTPS endpoints
- **Security-focused**: Checksum verification, GPG signatures, SSRF prevention, git bomb protection
- **Zero root required**: Installs to `~/.local` by default
- **Cross-platform**: Automatic OS/architecture detection
- **Flexible**: Webpage scraping for software without APIs

---

## Quick Start

```bash
# 1. Install Ora (requires Rust)
cargo install --git https://github.com/Altagen/Ora

# 2. Add a registry (git repository with package definitions)
ora registry add my-packages https://github.com/user/ora-packages.git

# 3. Sync the registry
ora registry sync

# 4. Search for packages
ora search ripgrep

# 5. Install a package
ora install ripgrep
```

**New to Ora?** See the [Getting Started Guide](docs/GETTING_STARTED.md) for a complete walkthrough.

**Creating packages?** See [Creating .repo Files](docs/CREATING_REPO_FILES.md) for a complete guide.

---

## Installation

### From Source

```bash
git clone https://github.com/Altagen/Ora
cd ora
cargo build --release
sudo cp target/release/ora /usr/local/bin/
```

### Build with GPG Support

Install libclang first:

```bash
# Arch Linux
sudo pacman -S clang

# Ubuntu/Debian
sudo apt-get install libclang-dev

# macOS
brew install llvm
```

Then build:

```bash
cargo build --release --features gpg
sudo cp target/release/ora /usr/local/bin/
```

### Using Cargo

```bash
# Latest release
cargo install ora

# From git repository
cargo install --git https://github.com/Altagen/Ora
```

---

## Basic Usage

### Registry Management

Ora supports two types of registries:

- **Git Registry**: A git repository with multiple `.repo` files in `/ora-registry/` directory
- **Direct URL Registry**: A single HTTPS endpoint serving one `.repo` file

```bash
# Add a Git registry (collection of packages)
ora registry add my-registry https://github.com/user/ora-packages.git

# Add a Direct URL registry (single package)
ora registry add windsurf https://example.com/packages/windsurf.repo

# List configured registries
ora registry list

# Sync registries to get latest packages (Git registries only)
ora registry sync

# Remove a registry
ora registry remove my-registry

# Verify a registry structure
ora registry verify my-registry
```

### Installing Packages

```bash
# Search for packages in registries
ora search ripgrep

# Get package information
ora info ripgrep

# Install from registry
ora install ripgrep

# Install specific version
ora install ripgrep --version 14.1.0

# Install from a .repo file
ora install --repo ./package.repo
```

### Managing Packages

```bash
# List installed packages
ora list

# Update all packages
ora update

# Update specific package
ora update ripgrep

# Uninstall a package
ora uninstall ripgrep
```

### Configuration

```bash
# Show configuration and paths
ora config show

# Verify configuration files
ora config verify

# Initialize configuration files
ora config init
```

For complete usage examples, see the [Getting Started Guide](docs/GETTING_STARTED.md).

---

## Configuration

Ora uses TOML configuration files in XDG locations:

- **Global config**: `~/.config/ora/config.toml`
- **Security policies**: `~/.config/ora/security.toml`
- **Package database**: `~/.config/ora/installed.toml`

Environment variables can override default paths:

- `ORA_CONFIG_DIR` - Configuration directory (default: `~/.config/ora`)
- `ORA_DATA_DIR` - Data directory (default: `~/.local/share/ora`)
- `ORA_CACHE_DIR` - Cache directory (default: `~/.cache/ora`)

For detailed security configuration options, see [Security Configuration](docs/SECURITY_CONFIGURATION.md).

---

## Security

Security is Ora's top priority. All downloads are verified with checksums, and the system includes protections against path traversal, zip bombs, SSRF attacks, and more.

For complete security details, see [Security Configuration](docs/SECURITY_CONFIGURATION.md).

### Reporting Security Issues

**DO NOT** open public issues for security vulnerabilities.

Please report security issues via [GitHub Security Advisories](https://github.com/Altagen/Ora/security/advisories/new).

---

## Creating Package Definitions

Packages are defined using `.repo` files. See:

- **[Quick Start Guide](docs/QUICK_START_REPO.md)** - Create your first .repo file in 5 minutes ⚡
- [Creating .repo Files](docs/CREATING_REPO_FILES.md) - Complete guide with examples
- [.repo Schema Reference](docs/REPO_SCHEMA.md) - Full schema documentation

---

## Documentation

### For Users
- **[Getting Started](docs/GETTING_STARTED.md)** - Installation, first steps, daily usage, troubleshooting
- [Security Configuration](docs/SECURITY_CONFIGURATION.md) - Security policies and configuration options

### For Package Creators
- **[Quick Start: Create a .repo File](docs/QUICK_START_REPO.md)** - 5-minute practical guide ⚡
- [Creating .repo Files](docs/CREATING_REPO_FILES.md) - How to package software for Ora
- [.repo Schema Reference](docs/REPO_SCHEMA.md) - Complete schema documentation

### For Contributors
- [Testing Guide](tests/README.md) - Running tests and development
- [Roadmap](ROADMAP/README.md) - Planned features and improvements

---

## Contributing

Contributions are welcome! Quick steps:

1. Fork the repository
2. Create a feature branch
3. Make your changes with conventional commits (feat:, fix:, docs:)
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Submit a pull request

---

## License

Ora is dual-licensed under your choice of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE](LICENSE))

---

## Community & Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/Altagen/Ora/issues)
- **GitHub Discussions**: [Ask questions and share ideas](https://github.com/Altagen/Ora/discussions)
