# Architecture Roadmap

This document outlines architectural improvements and refactoring planned for Ora.

---

## Privilege Separation

**Status**: Planned
**Priority**: High

Separate components with different privilege levels to minimize security impact.

### Current Architecture

Currently, Ora runs as a single process with user privileges. All operations (download, verification, extraction, installation) run with the same privileges.

### Proposed Architecture

```
┌─────────────────────────────────────────┐
│  ora (main process, user privileges)    │
│  - CLI parsing                           │
│  - Configuration management              │
│  - Orchestration                         │
└──────────────┬──────────────────────────┘
               │
       ┌───────┴────────┐
       │                │
       ▼                ▼
┌─────────────┐   ┌──────────────────┐
│ ora-fetch   │   │ ora-install      │
│ (network)   │   │ (filesystem)     │
│             │   │                  │
│ - Download  │   │ - Verification   │
│ - Scraping  │   │ - Extraction     │
│ - Checksum  │   │ - Deployment     │
└─────────────┘   └──────────────────┘
```

### Benefits

- **Security**: Limit blast radius of vulnerabilities
- **Isolation**: Network operations isolated from filesystem
- **Sandboxing**: Easier to sandbox individual components
- **Testing**: Individual components easier to test

### Implementation

**Phase 1**: Separate into modules
- Refactor code into distinct crates
- Define clear interfaces between components
- Maintain single-process architecture

**Phase 2**: Multi-process architecture
- Spawn separate processes for fetch/install
- IPC via Unix sockets or pipes
- Privilege dropping where possible

---

## Modular Provider System

**Status**: Planned
**Priority**: Medium

Plugin system for custom package sources.

### Current State

Providers are hardcoded in `src/providers/`:
- `github.rs`
- `gitlab.rs`
- `direct_url.rs`
- `custom_api.rs`
- `webpage_scraping.rs`

### Proposed System

**Provider Plugin Interface**:
```rust
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn discover_versions(&self, config: &RepoConfig) -> Result<Vec<Version>>;
    fn get_download_url(&self, config: &RepoConfig, version: &str) -> Result<String>;
    fn verify(&self, config: &RepoConfig, file: &Path) -> Result<()>;
}
```

**Dynamic Loading**:
```toml
# ~/.config/ora/providers.toml
[[provider]]
name = "custom-s3"
type = "plugin"
path = "/usr/local/lib/ora/providers/s3.so"

[[provider]]
name = "artifactory"
type = "plugin"
path = "/usr/local/lib/ora/providers/artifactory.so"
```

### Benefits

- **Extensibility**: Users can add custom providers
- **Separation**: Provider logic separated from core
- **Maintenance**: Providers can be updated independently

### Implementation

- Define stable provider trait
- Support dynamic library loading (`.so`, `.dylib`, `.dll`)
- Sandboxing for untrusted providers
- Provider discovery and registration

---

## Configuration Validation

**Status**: Planned
**Priority**: High

Enforce minimum security baseline and validate all configuration.

### Configuration Layers

```
System Config     →  /etc/ora/config.toml (admin-defined baseline)
        ↓
User Config       →  ~/.config/ora/config.toml (user preferences)
        ↓
Repo Config       →  package.repo (package-specific)
        ↓
CLI Flags         →  --allow-insecure, etc. (runtime overrides)
```

### Validation Rules

**System-Level Enforcement**:
- Minimum TLS version (no TLS 1.0/1.1)
- Required security checks (checksum verification)
- Prohibited operations (e.g., no `allow_insecure`)

**Example System Config**:
```toml
# /etc/ora/config.toml (admin-defined, user cannot override)
[security.enforce]
require_checksums = true
require_tls_1_2 = true
prohibit_insecure = true
max_file_size = "5GB"
```

**Validation on Load**:
- Parse all config files
- Merge configurations with precedence
- Validate against system policy
- Fail if policy violations detected

### Benefits

- **Compliance**: Enforce organizational security policies
- **Safety**: Prevent dangerous configurations
- **Clarity**: Clear error messages on policy violations

---

## Offline Mode Architecture

**Status**: Planned (see [OFFLINE.md](OFFLINE.md))
**Priority**: Medium

Better support for air-gapped environments.

### Key Components

**1. Cache Manager**:
- Index cached packages (SQLite)
- Validate cache integrity
- LRU eviction policy
- Export/import functionality

**2. Registry Manager**:
- Clone registries locally
- Offline search capabilities
- Update detection (when online)

**3. Bundle System**:
- Self-contained package bundles
- Bundle creation and verification
- Installation from bundles

**4. Lockfile Support**:
- Generate deterministic lockfiles
- Install from lockfile
- Verify lockfile integrity

See [OFFLINE.md](OFFLINE.md) for detailed architecture.

---

## Async/Concurrency Improvements

**Status**: Planned
**Priority**: Medium

Better utilization of async Rust and parallel operations.

### Current State

- Basic async/await usage with Tokio
- Sequential operations in many places
- Limited parallelism

### Improvements

**1. Parallel Downloads**:
```rust
// Download multiple packages concurrently
let downloads = packages.iter()
    .map(|pkg| download_package(pkg))
    .collect::<Vec<_>>();

let results = futures::future::join_all(downloads).await;
```

**2. Parallel Extraction**:
```rust
// Extract multiple archives in parallel
tokio::task::spawn_blocking(|| extract_archive(path));
```

**3. Concurrent Registry Updates**:
```rust
// Update all registries in parallel
let updates = registries.iter()
    .map(|reg| reg.update())
    .collect::<Vec<_>>();

futures::future::join_all(updates).await;
```

### Benefits

- **Performance**: Faster installations and updates
- **Responsiveness**: Non-blocking UI updates
- **Efficiency**: Better resource utilization

---

## Error Handling Refactor

**Status**: Planned
**Priority**: Medium

Improve error handling throughout the codebase.

### Current State

- Mix of `anyhow::Result` and custom errors
- Generic error messages
- Limited error context

### Proposed System

**Structured Errors**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum OraError {
    #[error("Package '{package}' not found in registry '{registry}'")]
    PackageNotFound {
        package: String,
        registry: String,
    },

    #[error("Checksum mismatch for {file}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        file: PathBuf,
        expected: String,
        actual: String,
    },

    #[error("Network error: {source}")]
    Network {
        #[from]
        source: reqwest::Error,
    },

    // ... more variants
}
```

**Error Context**:
```rust
.context(format!("Failed to download {} from {}", package, url))
.with_suggestion("Try checking your network connection")
```

### Benefits

- **Clarity**: Clear, actionable error messages
- **Debugging**: Rich error context for troubleshooting
- **User Experience**: Better suggestions and help

---

## Database Abstraction

**Status**: Planned
**Priority**: Low

Abstract database operations for better testing and flexibility.

### Current State

- Direct TOML file reading/writing
- No transaction support
- Limited querying capabilities

### Proposed System

**Database Trait**:
```rust
pub trait Database: Send + Sync {
    async fn get_installed(&self, name: &str) -> Result<Option<InstalledPackage>>;
    async fn list_installed(&self) -> Result<Vec<InstalledPackage>>;
    async fn save_installed(&self, pkg: &InstalledPackage) -> Result<()>;
    async fn remove_installed(&self, name: &str) -> Result<()>;
}
```

**Implementations**:
- `TomlDatabase`: Current TOML-based storage
- `SqliteDatabase`: SQLite for better querying
- `MemoryDatabase`: For testing

### Benefits

- **Testing**: Easy to mock database
- **Flexibility**: Swap storage backends
- **Performance**: Better query capabilities with SQLite

---

## Testing Infrastructure

**Status**: Planned
**Priority**: High

Improve testing coverage and infrastructure.

### Test Categories

**1. Unit Tests**:
- Individual function testing
- Mock external dependencies
- Fast execution

**2. Integration Tests**:
- Multi-component interactions
- Real filesystem/network (isolated)
- E2E workflows

**3. Property Tests**:
- QuickCheck-style property testing
- Fuzzing for parsers
- Edge case discovery

**4. Performance Tests**:
- Benchmark critical paths
- Regression detection
- Memory usage tracking

### Test Infrastructure

**Mock Provider**:
```rust
struct MockProvider {
    versions: Vec<String>,
    download_url: String,
}

impl Provider for MockProvider {
    // ... controlled responses for testing
}
```

**Test Fixtures**:
```
tests/fixtures/
├── repos/           # Sample .repo files
├── archives/        # Test archives
├── registries/      # Mock registries
└── configs/         # Test configurations
```

**CI/CD Integration**:
- Run tests on every PR
- Test multiple platforms (Linux, macOS, Windows)
- Security scanning (cargo audit)
- Coverage reporting

---

## Documentation Generation

**Status**: Planned
**Priority**: Medium

Automatic generation of API documentation and guides.

### Components

**1. API Documentation**:
```bash
# Generate Rust API docs
cargo doc --no-deps --open
```

**2. CLI Documentation**:
```bash
# Generate man pages
ora man --output /usr/share/man/man1/

# Generate markdown docs
ora docs --format markdown --output docs/
```

**3. Configuration Schema**:
```bash
# Generate JSON schema for configs
ora schema --output schema/config.json
```

### Publishing

- Host docs at docs.ora-pm.dev
- Include in releases
- Package with installers

---

## Metrics and Telemetry (Optional)

**Status**: Planned
**Priority**: Low

Optional, privacy-respecting telemetry for understanding usage.

### Principles

- **Opt-in only**: Never enabled by default
- **Privacy-first**: No PII collection
- **Transparent**: Show exactly what is sent
- **Local-first**: Store locally, sync optionally

### Collected Metrics

- Command usage (install, update, etc.)
- Provider usage (GitHub, GitLab, etc.)
- Installation times (performance tracking)
- Error rates (reliability tracking)

### Implementation

```toml
# ~/.config/ora/config.toml
[telemetry]
enabled = false      # Opt-in
endpoint = "https://telemetry.ora-pm.dev"
anonymous_id = "uuid"
```

---

## References

- [Cargo Architecture](https://doc.rust-lang.org/cargo/contrib/architecture/)
- [Rustup Architecture](https://rust-lang.github.io/rustup/dev-guide/)
- [Privilege Separation](https://en.wikipedia.org/wiki/Privilege_separation)
- [Plugin Systems in Rust](https://adventures.michaelfbryan.com/posts/plugins-in-rust/)
