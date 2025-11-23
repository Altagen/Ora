# Offline Mode Support

This document outlines the plan for offline mode support in Ora, enabling usage in air-gapped and network-restricted environments.

---

## Overview

**Offline mode** enables Ora to function without Internet access by leveraging local caches, pre-downloaded packages, and locally cloned registries. This is crucial for:

- **Air-gapped environments**: Military, finance, critical infrastructure
- **Development machines** without Internet access
- **Isolated CI/CD** environments
- **Network resilience**: Continue working during outages
- **Performance**: Faster installations from local cache
- **Bandwidth economy**: Avoid re-downloading packages

---

## Use Cases

### 1. Air-Gapped Server Deployment

```bash
# On a machine WITH Internet (prepare cache)
ora download windsurf prometheus kubectl terraform
ora registry update --clone-local
ora cache export --output /mnt/usb/ora-offline-cache.tar.gz

# Transfer file to air-gapped machine

# On the machine WITHOUT Internet
ora cache import /mnt/usb/ora-offline-cache.tar.gz
ora install --offline windsurf
ora install --offline prometheus
```

### 2. CI/CD Pipelines

```bash
# Cache packages for CI/CD
ora download --lock-file ora.lock
ora install --offline --locked
```

### 3. Development Environments

```bash
# Work offline with cached packages
ora install --prefer-cache kubectl
ora update --offline --cached-only
```

---

## Proposed Commands

### Install from Cache

```bash
# Install only if available in cache (strict mode)
ora install --offline <package>

# Prefer cache, fall back to download
ora install --prefer-cache <package>
```

### Download Without Installing

```bash
# Download specific packages
ora download windsurf prometheus kubectl

# Download from lockfile
ora download --lock-file ora.lock

# Download specific version
ora download windsurf@1.12.35
```

### Cache Management

```bash
# List cached packages
ora cache list

# Show cache statistics
ora cache stats

# Export cache for transfer
ora cache export --output cache.tar.gz

# Import cache from archive
ora cache import cache.tar.gz

# Clean old/unused cache entries
ora cache clean
```

### Registry Operations

```bash
# Clone registry for offline use
ora registry add --clone-local official https://github.com/ora-pm/registry.git

# Update local registry clones (requires Internet)
ora registry update

# Search in local registries only
ora search --offline <query>
```

### Bundle Creation

```bash
# Create self-contained bundle
ora bundle create --package windsurf --output windsurf-bundle.tar.gz

# Include dependencies in bundle
ora bundle create --package kubectl --with-dependencies --output kubectl-bundle.tar.gz

# Install from bundle
ora install --from-bundle windsurf-bundle.tar.gz
```

---

## Architecture

### 1. Intelligent Download Cache

**Current State**: Ora already has `~/.cache/ora/downloads/`

**Improvements Needed**:

- **Cache Index**: SQLite database tracking available packages/versions
- **Metadata Storage**: Store `.repo` files alongside downloaded archives
- **Cache Validation**: Verify cached files haven't been corrupted
- **Automatic Cleanup**: LRU eviction when cache exceeds size limit

**Cache Structure**:
```
~/.cache/ora/
├── downloads/
│   ├── windsurf-1.12.35.tar.gz
│   ├── prometheus-2.45.0.tar.gz
│   └── ...
├── metadata/
│   ├── windsurf.repo
│   ├── prometheus.repo
│   └── ...
└── index.db (SQLite)
```

### 2. Local Registries

**Features**:

- Clone Git-based registries to local filesystem
- Use local clones without `git pull` in offline mode
- Index local registry packages for fast searching

**Configuration**:
```toml
# ~/.config/ora/registries.toml
[[registry]]
name = "official"
type = "git"
url = "https://github.com/ora-pm/registry.git"
local_clone = "/home/user/.local/share/ora/registries/official"
offline_mode = true  # Don't pull in offline mode
```

### 3. Package Bundles

**Bundle Format** (tar.gz containing):
```
bundle/
├── package.tar.gz           # Original archive
├── package.repo             # Repo configuration
├── checksums.txt            # Checksums
├── metadata.json            # Package metadata
└── signature.asc (optional) # GPG signature
```

**Bundle Operations**:
- Create: Package archive + metadata into bundle
- Verify: Check integrity before installation
- Install: Extract and install without network access

### 4. Lockfile Support

**Lockfile Format** (`ora.lock`):
```toml
version = 1

[[package]]
name = "windsurf"
version = "1.12.35"
checksum = "sha256:abcd1234..."
source = "https://windsurf.com/..."
registry = "custom"

[[package]]
name = "prometheus"
version = "2.45.0"
checksum = "sha256:5678efgh..."
source = "https://github.com/..."
registry = "official"
```

**Usage**:
```bash
# Generate lockfile
ora lock

# Install from lockfile
ora install --locked
```

---

## Implementation Phases

### Phase 1: Basic Offline Support

**Features**:
- `--prefer-cache` flag: Use cache if available, download otherwise
- `--offline` flag: Strict mode, fail if not in cache
- Cache index to track available packages
- Basic cache listing command

**Estimated Effort**: 2-3 weeks

### Phase 2: Download & Registry Support

**Features**:
- `ora download` command for pre-fetching packages
- Registry cloning with `--clone-local`
- Offline registry search
- Cache export/import commands

**Estimated Effort**: 3-4 weeks

### Phase 3: Advanced Features

**Features**:
- Bundle creation and installation
- Lockfile generation and installation
- Cache size limits and LRU eviction
- Automatic cache validation

**Estimated Effort**: 3-4 weeks

---

## Configuration

### Global Config

```toml
# ~/.config/ora/config.toml

[cache]
max_size = "10GB"           # Maximum cache size
retention_days = 30         # Keep cached files for 30 days
auto_cleanup = true         # Automatically clean old entries

[offline]
prefer_cache = false        # Default to network mode
strict = false              # Fail if package not in cache
validate_cache = true       # Verify cached file integrity
```

### Security Config

```toml
# ~/.config/ora/security.toml

[offline]
require_signatures = true   # Require signatures even for cached packages
verify_checksums = true     # Always verify checksums
allow_bundles = true        # Allow bundle installation
```

---

## Comparison with Other Package Managers

| Package Manager | Offline Command | Cache Export | Bundles | Lockfile |
|----------------|-----------------|--------------|---------|----------|
| **npm** | `--offline`, `--prefer-offline` | ✅ (tarball) | ❌ | ✅ (package-lock.json) |
| **pip** | `--no-index` + wheelhouse | ✅ (wheelhouse) | ❌ | ✅ (requirements.txt) |
| **cargo** | `--offline` | ✅ (vendor) | ❌ | ✅ (Cargo.lock) |
| **apt** | apt-offline | ✅ (archives) | ❌ | ❌ |
| **nix** | Store-based | ✅ (closures) | ✅ | ✅ (flake.lock) |
| **Ora (planned)** | `--offline` | ✅ | ✅ | ✅ |

---

## Security Considerations

### Cache Integrity

- **Checksums**: Verify cached files haven't been tampered with
- **Signatures**: Validate GPG signatures even for cached packages
- **Timestamps**: Detect if cache has been modified

### Bundle Trust

- **Signature Verification**: Require signed bundles from trusted sources
- **Checksum Validation**: Verify bundle contents
- **Source Tracking**: Record bundle origin in audit log

### Offline Security

- **No Phone Home**: Truly offline, no network requests
- **Audit Logging**: Log all offline installations
- **Cache Permissions**: Protect cache directory (0700)

---

## Example Workflows

### Workflow 1: Prepare Offline Environment

```bash
# Step 1: On Internet-connected machine
ora download windsurf@1.12.35 prometheus@2.45.0 kubectl@1.28.0
ora registry update --clone-local
ora cache export --output /media/usb/ora-cache.tar.gz

# Step 2: Transfer USB drive to offline machine

# Step 3: On offline machine
ora cache import /media/usb/ora-cache.tar.gz
ora install --offline windsurf
ora install --offline prometheus
ora install --offline kubectl
```

### Workflow 2: CI/CD with Lockfile

```bash
# In development
ora install windsurf prometheus kubectl
ora lock  # Generate ora.lock

# In CI/CD pipeline
ora download --lock-file ora.lock
ora install --locked --offline
```

### Workflow 3: Bundle Distribution

```bash
# Create bundle
ora bundle create --package windsurf --output windsurf-1.12.35-bundle.tar.gz

# Distribute bundle (USB, internal network, etc.)

# Install on target machine
ora install --from-bundle windsurf-1.12.35-bundle.tar.gz
```

---

## Benefits

### 1. Security
- Audit cached packages before installation
- Reduce attack surface (no network requests)
- Controlled package sources

### 2. Reproducibility
- Guarantee same binaries everywhere
- Lockfile ensures consistent versions
- Bundle distribution for exact replication

### 3. Performance
- Near-instant installations from cache
- No network latency
- Reduced bandwidth usage

### 4. Reliability
- Independent of Internet availability
- No dependency on upstream servers
- Resilient to network issues

### 5. Compliance
- Meet air-gap requirements
- Satisfy security policies
- Enable restricted environment deployments

---

## References

- [npm offline documentation](https://docs.npmjs.com/cli/v8/commands/npm-install#offline)
- [Cargo offline mode](https://doc.rust-lang.org/cargo/commands/cargo-fetch.html)
- [Air-gapped Kubernetes](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/#air-gapped-environments)
- [apt-offline](https://github.com/rickysarraf/apt-offline)
