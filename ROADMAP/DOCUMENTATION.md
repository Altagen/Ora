# Documentation Roadmap

This document outlines documentation improvements and additions planned for Ora.

---

## User Documentation

### Getting Started Guide

**Status**: Partial (exists as `docs/GETTING_STARTED.md`)
**Priority**: High

**Improvements Needed**:
- Quick start (5-minute setup)
- Common use cases with examples
- Troubleshooting section
- Platform-specific instructions

**Structure**:
```markdown
# Getting Started

## Installation
- Linux (various distros)
- macOS
- Windows

## Quick Start
- Installing your first package
- Searching for packages
- Updating packages

## Common Tasks
- Working with registries
- Using .repo files
- Managing installed packages

## Troubleshooting
- Common errors and solutions
- Debug mode
- Getting help
```

---

### Security Best Practices Guide

**Status**: Planned
**Priority**: High

Documentation of security features and recommendations.

**Topics**:
- Understanding `allow_insecure` flag
- Checksum verification
- GPG signature verification (when implemented)
- Post-install script safety
- Registry trust management
- Security configuration options
- Threat model and limitations

**Example Content**:
```markdown
## Security Best Practices

### Never Use --allow-insecure in Production
The `--allow-insecure` flag bypasses checksum and signature verification.
Only use it for:
- Packages you've built yourself
- Testing and development
- Packages from trusted sources with verified checksums

### Verify Registry Sources
Always verify the source of registries:
```bash
ora registry list  # Check configured registries
```

### Review Post-Install Scripts
Post-install scripts run with your user privileges:
```bash
# Always review before accepting
cat package.repo | grep -A 20 "post_install"
```
```

---

### Security Limitations Document

**Status**: Planned
**Priority**: Critical

Clear documentation of current security gaps and limitations.

**Purpose**: Transparency about what Ora does and doesn't protect against.

**Topics**:
- Unimplemented security features (from [SECURITY.md](SECURITY.md))
- Known vulnerabilities
- Trust boundaries
- Attack scenarios
- Mitigation strategies

**Example Structure**:
```markdown
# Security Limitations

## Current Limitations

### ⚠️ No GPG Signature Verification (Yet)
**Impact**: Cannot verify package authenticity
**Status**: Planned (see ROADMAP/SECURITY.md)
**Mitigation**:
- Use `checksum_url` in .repo files
- Only use trusted registries
- Review packages manually

### ⚠️ Post-Install Scripts Run Unsandboxed
**Impact**: Scripts have full user privileges
**Status**: Sandboxing planned
**Mitigation**:
- Always review script content
- Only install from trusted sources
- Run Ora in a VM/container for untrusted packages
```

---

### Provider API Documentation

**Status**: Planned
**Priority**: Medium

Documentation for implementing custom providers.

**Audience**: Developers who want to add support for new package sources.

**Topics**:
- Provider trait overview
- Implementing a custom provider
- Testing providers
- Provider best practices
- Security considerations

**Example Guide**:
```markdown
# Creating Custom Providers

## Provider Trait

All providers implement the `Provider` trait:

```rust
pub trait Provider {
    fn discover_versions(&self, config: &RepoConfig) -> Result<Vec<String>>;
    fn get_download_url(&self, config: &RepoConfig, version: &str) -> Result<String>;
    fn get_checksum(&self, config: &RepoConfig, version: &str) -> Result<Option<String>>;
}
```

## Example: S3 Provider

[Step-by-step implementation example]
```

---

### Registry Setup Guide

**Status**: Planned
**Priority**: Medium

Guide for setting up and managing package registries.

**Topics**:

**1. Public Registry Setup**:
- Creating a Git repository
- Organizing .repo files
- Repository structure
- Best practices

**2. Private Registry Setup**:
- Authentication configuration
- Access control
- Mirroring public registries

**3. Registry Management**:
- Adding new packages
- Updating package definitions
- Deprecating packages
- Security policies

**Example Structure**:
```markdown
# Setting Up a Package Registry

## Quick Start

Create a Git repository with this structure:
```
my-registry/
├── packages/
│   ├── kubectl.repo
│   ├── terraform.repo
│   └── prometheus.repo
├── README.md
└── .gitignore
```

## Adding Packages

Create a .repo file for each package:
```toml
[package]
name = "kubectl"
description = "Kubernetes command-line tool"

[source.version]
discovery_type = "github-releases"
owner = "kubernetes"
repo = "kubernetes"
```

## Publishing

Push to GitHub and add to Ora:
```bash
ora registry add my-registry https://github.com/myorg/my-registry.git
```
```

---

### Migration Guide

**Status**: Planned
**Priority**: Low

Help users migrate from other package managers to Ora.

**Supported Migrations**:
- From Homebrew
- From apt/dnf/pacman
- From manual installations
- From language-specific managers (npm, cargo, pip)

**Example Content**:
```markdown
# Migrating from Homebrew

## Finding Equivalent Packages

Many Homebrew packages have Ora equivalents:

| Homebrew | Ora Equivalent |
|----------|---------------|
| `brew install kubectl` | `ora install kubectl` |
| `brew install terraform` | `ora install terraform` |
| `brew install prometheus` | `ora install prometheus` |

## Inventory Current Packages

List your Homebrew packages:
```bash
brew list > homebrew-packages.txt
```

Search for Ora equivalents:
```bash
cat homebrew-packages.txt | xargs -I {} ora search {}
```

## Migration Strategy

1. Keep Homebrew installed during migration
2. Install Ora alongside Homebrew
3. Migrate packages incrementally
4. Verify each package works with Ora
5. Uninstall from Homebrew after verification
```

---

### .repo File Reference

**Status**: Partial (exists as `docs/CREATING_REPO_FILES.md`)
**Priority**: High

Complete reference for all .repo file options.

**Improvements**:
- Exhaustive field documentation
- All provider types documented
- Examples for each provider
- Advanced configuration scenarios
- Schema validation

**Structure**:
```markdown
# .repo File Format Reference

## Package Section
[Complete reference with all fields]

## Source Section
[All discovery types with examples]

### GitHub Releases
[Complete field reference]

### GitLab Releases
[Complete field reference]

### Webpage Scraping
[Complete field reference]

### Direct URL
[Complete field reference]

### Custom API
[Complete field reference]

## Install Section
[Binary patterns, post-install scripts, etc.]

## Security Section
[Security options reference]

## Examples
[Real-world examples for common scenarios]
```

---

## Developer Documentation

### Architecture Guide

**Status**: Planned
**Priority**: Medium

Overview of Ora's architecture for contributors.

**Topics**:
- Codebase structure
- Module responsibilities
- Data flow
- Key abstractions
- Design decisions

**Diagram Example**:
```
┌─────────────┐
│ CLI (args)  │
└──────┬──────┘
       │
       ▼
┌──────────────────┐
│ Command Handler  │
└──────┬───────────┘
       │
   ┌───┴────┐
   │        │
   ▼        ▼
┌─────┐  ┌──────────┐
│ Reg │  │ Provider │
│ Mgr │  │          │
└─────┘  └────┬─────┘
              │
              ▼
       ┌──────────────┐
       │  Installer   │
       │  Pipeline    │
       └──────────────┘
```

---

### Contributing Guide

**Status**: Planned
**Priority**: High

Guide for potential contributors.

**Topics**:
- Setting up development environment
- Building from source
- Running tests
- Code style and conventions
- Submitting PRs
- Issue triage process

---

### Release Process

**Status**: Planned
**Priority**: Medium

Documentation of release procedures.

**Topics**:
- Version numbering (SemVer)
- Changelog generation
- Building releases
- Publishing to registries
- Announcing releases

---

## API Documentation

### Rust API Docs

**Status**: Partial
**Priority**: Medium

Comprehensive rustdoc documentation.

**Improvements**:
- Document all public APIs
- Examples for common operations
- Link to guides
- Architecture documentation

**Generation**:
```bash
cargo doc --no-deps --open
```

---

### Man Pages

**Status**: Planned
**Priority**: Low

Traditional man pages for Unix systems.

**Pages**:
- `ora(1)`: Main command
- `ora-install(1)`: Install subcommand
- `ora-update(1)`: Update subcommand
- `ora-registry(1)`: Registry management
- `ora.repo(5)`: Repo file format

**Installation**:
```bash
# Install to system
ora man --install

# Manual installation
cp docs/man/*.1 /usr/share/man/man1/
```

---

## Interactive Documentation

### Examples Repository

**Status**: Planned
**Priority**: Low

Repository of real-world .repo files and examples.

**Structure**:
```
ora-examples/
├── simple/
│   ├── kubectl.repo
│   ├── terraform.repo
│   └── README.md
├── advanced/
│   ├── electron-app.repo
│   ├── custom-api.repo
│   └── README.md
└── security/
    ├── signed-package.repo
    ├── checksum-verification.repo
    └── README.md
```

---

### Tutorial Videos

**Status**: Planned
**Priority**: Low

Short tutorial videos for common tasks.

**Topics**:
- Getting started (5 min)
- Creating a .repo file (10 min)
- Setting up a registry (15 min)
- Security best practices (10 min)

---

### FAQ

**Status**: Planned
**Priority**: Medium

Frequently asked questions.

**Categories**:
- General questions
- Installation issues
- Security concerns
- Performance questions
- Comparison with other tools

**Example Questions**:
- **Q**: How is Ora different from Homebrew?
- **Q**: Can I use Ora to install system packages?
- **Q**: Is it safe to use `--allow-insecure`?
- **Q**: How do I create my own registry?

---

## Documentation Infrastructure

### Documentation Website

**Status**: Planned
**Priority**: Medium

Dedicated documentation website (e.g., docs.ora-pm.dev).

**Features**:
- Searchable documentation
- Version-specific docs
- Dark/light theme
- Mobile-friendly
- Generated from markdown

**Technology Options**:
- mdBook (Rust ecosystem)
- Docusaurus
- GitBook

---

### Documentation Testing

**Status**: Planned
**Priority**: Medium

Ensure documentation stays up-to-date.

**Methods**:
- Test code examples in docs
- Validate command outputs
- Check links (no 404s)
- Spell checking

**Tools**:
- `cargo test --doc` for Rust examples
- `markdownlint` for markdown validation
- Link checkers

---

### Changelog Automation

**Status**: Partial (git-cliff configured)
**Priority**: High

Automatic changelog generation from commits.

**Current State**: git-cliff configured

**Improvements**:
- Consistent commit message format
- Automatic categorization
- Link to issues/PRs
- Generate on release

---

## Localization (i18n)

**Status**: Planned
**Priority**: Low

Support for multiple languages.

**Languages** (priority order):
1. English (native)
2. French
3. German
4. Spanish
5. Japanese
6. Chinese

**Scope**:
- CLI messages and errors
- Documentation
- Website content

---

## References

- [Rust API Documentation Guidelines](https://rust-lang.github.io/api-guidelines/documentation.html)
- [Write the Docs](https://www.writethedocs.org/)
- [Divio Documentation System](https://documentation.divio.com/)
- [mdBook](https://rust-lang.github.io/mdBook/)
