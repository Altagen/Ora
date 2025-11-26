# Ora Roadmap

This document provides an overview of Ora's development roadmap. For detailed information, see the [ROADMAP/](ROADMAP/) directory.

---

## Quick Links

- **[Recently Completed](ROADMAP/COMPLETED.md)** - Features and improvements completed recently
- **[Security](ROADMAP/SECURITY.md)** - Security improvements and hardening (Critical Priority)
- **[Features](ROADMAP/FEATURES.md)** - New features and enhancements
- **[Testing](ROADMAP/TESTING.md)** - Test coverage improvements and missing tests
- **[Offline Mode](ROADMAP/OFFLINE.md)** - Support for air-gapped and offline environments
- **[Architecture](ROADMAP/ARCHITECTURE.md)** - Architectural improvements and refactoring
- **[Documentation](ROADMAP/DOCUMENTATION.md)** - Documentation improvements
- **[Dependencies](ROADMAP/DEPENDENCIES.md)** - Dependency management and updates

---

## Recent Highlights ✅

### Update Flow for --repo Packages
Fixed `ora update` to work with packages installed from local .repo files by using the `registry_source` field.

### Automatic Directory Cleanup
Uninstall now removes empty parent directories automatically after package removal.

### Webpage Scraping Provider
Full support for discovering package versions by scraping HTML pages with regex-based URL and version extraction.

### Windsurf Integration
Complete support for Electron-based applications with proper permission handling for all required executables.

See [ROADMAP/COMPLETED.md](ROADMAP/COMPLETED.md) for full details.

---

## Priority Areas

### 🔴 Critical: Security Hardening

Security is the highest priority for Ora. Key areas:

- **GPG Signature Verification** - Verify package authenticity
- **Script Sandboxing** - Isolate post-install scripts
- **HTTP Redirect Enforcement** - Prevent redirect-based attacks
- **Template Injection Fixes** - Secure template resolution
- **Environment Variable Filtering** - Protect credentials

See [ROADMAP/SECURITY.md](ROADMAP/SECURITY.md) for complete security roadmap.

---

### 🟡 High: Core Features

Essential features for a robust package manager:

- **Offline Mode** - Support for air-gapped environments
- **Version Pinning** - Lockfile mechanism for reproducibility
- **SLSA Provenance** - Supply chain security attestations
- **Registry Signing** - Cryptographic verification of registries
- **Better Error Messages** - More actionable error feedback

See [ROADMAP/FEATURES.md](ROADMAP/FEATURES.md) and [ROADMAP/OFFLINE.md](ROADMAP/OFFLINE.md) for details.

---

### 🟢 Medium: Architecture & Developer Experience

Improvements to code quality and developer experience:

- **Privilege Separation** - Separate components with different privilege levels
- **Modular Providers** - Plugin system for custom package sources
- **Progress Indicators** - Better download/install progress feedback
- **Async/Concurrency** - Better parallelism and performance

See [ROADMAP/ARCHITECTURE.md](ROADMAP/ARCHITECTURE.md) for details.

---

## Documentation

Comprehensive documentation is essential for adoption:

- **Security Best Practices** - Guide for secure Ora usage
- **Security Limitations** - Transparent disclosure of current gaps
- **Provider API** - Documentation for custom provider development
- **Registry Setup** - Guide for creating package registries
- **Migration Guides** - Help users migrate from other package managers

See [ROADMAP/DOCUMENTATION.md](ROADMAP/DOCUMENTATION.md) for details.

---

## Contributing

This roadmap is not a commitment or timeline - items represent areas that could be enhanced if contributors are interested.

**Interested in contributing?**
- Pick any item from the roadmap
- Open an issue to discuss your approach
- Submit a PR with your implementation
- Security fixes are always welcome and will be prioritized

**Priority Guidelines**:
- Security fixes take priority over features
- Items may be added, removed, or reprioritized
- Community input is valued and welcome

---

## Roadmap Organization

The roadmap is organized into thematic documents in the [ROADMAP/](ROADMAP/) directory:

```
ROADMAP/
├── README.md           # Overview and organization
├── COMPLETED.md        # Recently completed work
├── SECURITY.md         # Security improvements (Critical/High/Medium/Low)
├── FEATURES.md         # New features and enhancements
├── TESTING.md          # Test coverage improvements and missing tests
├── OFFLINE.md          # Offline mode architecture and design
├── ARCHITECTURE.md     # Architectural improvements
├── DOCUMENTATION.md    # Documentation improvements
└── DEPENDENCIES.md     # Dependency management
```

This organization allows for:
- **Focused discussions** on specific areas
- **Independent evolution** of different roadmap areas
- **Easier tracking** of progress by theme
- **Better discoverability** of related work

---

## Release Philosophy

Ora follows [Semantic Versioning](https://semver.org/):
- **Major** (x.0.0): Breaking changes
- **Minor** (0.x.0): New features, backward compatible
- **Patch** (0.0.x): Bug fixes, security updates

**Security Updates**:
- Security fixes are released immediately
- Patch versions for critical vulnerabilities
- Clear security advisories published

**Feature Releases**:
- Regular minor version releases
- Deprecation warnings before breaking changes
- Migration guides for major versions

---

## Community & Feedback

- **Issues**: [github.com/ora-pm/ora/issues](https://github.com/ora-pm/ora/issues)
- **Discussions**: [github.com/ora-pm/ora/discussions](https://github.com/ora-pm/ora/discussions)
- **Security**: security@ora-pm.dev (for security vulnerabilities)

---

## License

Ora is open source software. Contributions are welcome under the project's license terms.
