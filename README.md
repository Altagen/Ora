<div align="center">
  <img src="assets/ora-icon.png" alt="Ora Logo" width="200"/>

  <h1>Ora — Omni Repository for Archives</h1>

  <p>A secure, decentralized package manager for pre-compiled binaries.</p>

  <p>
    <a href="https://github.com/Altagen/Ora/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/Altagen/Ora/ci.yml?label=CI" alt="CI" /></a>
    <a href="https://github.com/Altagen/Ora/releases/latest"><img src="https://img.shields.io/github/v/release/Altagen/Ora" alt="Latest Release" /></a>
    <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg" alt="License" /></a>
  </p>
</div>

---

## What is Ora

Ora is a package manager for installing and managing pre-compiled binaries from diverse sources including GitHub Releases, GitLab, custom APIs, webpage scraping, and direct URLs. It runs in userland by default (no root required), supports both decentralized git-based registries and direct HTTPS registries, and enforces comprehensive security configurations to protect against supply chain attacks.

**Key features:**

- **Multi-source support** — GitHub Releases, GitLab, custom APIs, webpage scraping, direct URLs
- **Dual registry modes** — Git-based collections or direct HTTPS endpoints
- **Security-focused** — Checksum verification, GPG signatures, SSRF prevention, zip bomb protection
- **Zero root required** — Installs to `~/.local` by default
- **Cross-platform** — Automatic OS/architecture detection
- **Flexible** — Webpage scraping for software without APIs

---

## Tech Stack

- **Language**: Rust 2021 edition
- **CLI**: clap 4.x
- **Async Runtime**: tokio
- **HTTP Client**: reqwest
- **Git**: git2
- **GPG**: sequoia-openpgp (optional feature)
- **Checksums**: sha2
- **Build System**: Task (go-task)

## Platform Support

| Platform | Status | Build |
|----------|--------|-------|
| Linux x86_64 | ✅ Supported | Native |
| Linux ARM64 | ✅ Supported | Cross-compiled |
| macOS Intel | ✅ Supported | Native |
| macOS Apple Silicon | ✅ Supported | Native |
| Windows | ❌ Not planned | — |

---

## Quick Start

```bash
# 1. Install Ora (requires Rust)
cargo install --git https://github.com/Altagen/Ora

# 2. Add a registry
ora registry add my-packages https://github.com/user/ora-packages.git

# 3. Sync the registry
ora registry sync

# 4. Install a package
ora install ripgrep
```

**New to Ora?** See the [Getting Started Guide](docs/GETTING_STARTED.md) for a complete walkthrough.

---

## Installation

### From Releases

Download the pre-compiled binary for your platform from the [Releases page](https://github.com/Altagen/Ora/releases/latest):

```bash
# Linux x86_64
curl -fsSL https://github.com/Altagen/Ora/releases/latest/download/ora-<version>-linux-amd64.tar.gz | tar xz
sudo install -m 755 ora-<version>-linux-amd64 /usr/local/bin/ora

# macOS (Apple Silicon)
curl -fsSL https://github.com/Altagen/Ora/releases/latest/download/ora-<version>-macos-arm64.tar.gz | tar xz
sudo install -m 755 ora-<version>-macos-arm64 /usr/local/bin/ora
```

### From Source

```bash
git clone https://github.com/Altagen/Ora
cd Ora
cargo build --release
sudo cp target/release/ora /usr/local/bin/
```

### With GPG Support

```bash
# Install libclang first (Arch: clang, Ubuntu: libclang-dev, macOS: llvm)
cargo build --release --features gpg
```

---

## Basic Usage

### Registry Management

```bash
# Add a Git registry (collection of packages)
ora registry add my-registry https://github.com/user/ora-packages.git

# Add a Direct URL registry (single package)
ora registry add windsurf https://example.com/packages/windsurf.repo

# List, sync, remove registries
ora registry list
ora registry sync
ora registry remove my-registry
```

### Installing Packages

```bash
ora search ripgrep
ora info ripgrep
ora install ripgrep
ora install ripgrep --version 14.1.0
ora install --repo ./package.repo
```

### Managing Packages

```bash
ora list
ora update
ora uninstall ripgrep
```

### Configuration

```bash
ora config show
ora config verify
ora config init
```

---

## Configuration

Ora uses TOML configuration files in XDG locations:

- **Global config**: `~/.config/ora/config.toml`
- **Security policies**: `~/.config/ora/security.toml`
- **Package database**: `~/.config/ora/installed.toml`

| Variable | Default |
|----------|---------|
| `ORA_CONFIG_DIR` | `~/.config/ora` |
| `ORA_DATA_DIR` | `~/.local/share/ora` |
| `ORA_CACHE_DIR` | `~/.cache/ora` |

---

## Security

Security is Ora's top priority. All downloads are verified with checksums, and the system includes protections against path traversal, zip bombs, SSRF attacks, and more.

For complete security details, see [Security Configuration](docs/SECURITY_CONFIGURATION.md).

**Reporting security vulnerabilities:** Please use [GitHub Security Advisories](https://github.com/Altagen/Ora/security/advisories/new) — do not open public issues.

---

## Project Structure

```
ora/
├── src/
│   ├── main.rs             # CLI entry point
│   ├── commands/           # CLI subcommands
│   ├── registry/           # Registry management
│   ├── package/            # Package installation
│   └── security/           # Checksum & GPG verification
├── docs/
│   ├── GETTING_STARTED.md
│   ├── CREATING_REPO_FILES.md
│   └── SECURITY_CONFIGURATION.md
├── Cargo.toml
└── Taskfile.yaml
```

## Development

```bash
git clone https://github.com/Altagen/Ora
cd Ora

# Build
cargo build

# Run tests
cargo test

# Lint
cargo clippy --all-targets --all-features -- -D warnings
```

---

## Documentation

### For Users
- [Getting Started](docs/GETTING_STARTED.md) — Installation, first steps, daily usage
- [Security Configuration](docs/SECURITY_CONFIGURATION.md) — Security policies and options

### For Package Creators
- [Quick Start: Create a .repo File](docs/QUICK_START_REPO.md) — 5-minute guide
- [Creating .repo Files](docs/CREATING_REPO_FILES.md) — Complete packaging guide
- [.repo Schema Reference](docs/REPO_SCHEMA.md) — Full schema documentation

---

## Contributing

Issues and bug reports are welcome on [GitHub](https://github.com/Altagen/Ora/issues).

---

## License

Ora is dual-licensed under your choice of:

- MIT License — [LICENSE-MIT](LICENSE-MIT)
- Apache License 2.0 — [LICENSE](LICENSE)
