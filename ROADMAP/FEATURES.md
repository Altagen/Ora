# Features Roadmap

This document outlines planned features and enhancements for Ora.

---

## Package Management

### Reproducible Builds

**Status**: Planned
**Priority**: High

Support for verifying reproducible builds.

**Features**:
- Verify build reproducibility attestations
- Compare checksums from multiple build sources
- Support for [reproducible-builds.org](https://reproducible-builds.org/) metadata

**Use Cases**:
- Verify binaries match source code
- Detect supply chain tampering
- Increase trust in package integrity

---

### SLSA Provenance

**Status**: Planned
**Priority**: High

Support for [SLSA](https://slsa.dev/) provenance attestations.

**Features**:
- Parse and validate SLSA provenance
- Verify build platform and parameters
- Display provenance information to users
- Support for different SLSA levels

**Format Support**:
- in-toto attestations
- SLSA provenance format v0.2+

---

### Version Pinning

**Status**: Planned
**Priority**: Medium

Lockfile mechanism for exact version and checksum pinning.

**Features**:
- Generate lockfile (`ora.lock`) on install
- Pin exact versions and checksums
- Reproducible installations from lockfile
- Support for updating specific packages in lockfile

**Commands**:
```bash
# Generate lockfile
ora lock

# Install from lockfile
ora install --locked

# Update specific package in lockfile
ora lock update <package>
```

---

### Self-Update

**Status**: Planned
**Priority**: Medium

Secure update mechanism for Ora itself.

**Features**:
- `ora self-update` command
- Checksum verification of Ora binaries
- Signature verification (GPG)
- Rollback on failure

**Implementation**:
- Detect current installation method (cargo, binary, package manager)
- Download latest release from GitHub
- Verify integrity
- Replace binary atomically

---

### Download Resume

**Status**: Planned
**Priority**: Low

Support resuming interrupted downloads.

**Features**:
- HTTP Range requests
- Partial download tracking
- Resume from last checkpoint
- Automatic retry with exponential backoff

**Use Cases**:
- Large packages on unstable connections
- Bandwidth-limited environments

---

## Registry

### Registry Signing

**Status**: Planned
**Priority**: High

GPG signatures for registry repositories.

**Features**:
- Sign registry commits with GPG
- Verify registry signatures on clone/pull
- Trusted keyring management
- Reject unsigned or untrusted registries

**Security Impact**: Prevents registry tampering.

---

### Multiple Registries

**Status**: Planned
**Priority**: Medium

Better handling of package conflicts across registries.

**Current State**: Basic multi-registry support exists.

**Improvements**:
- Clear conflict resolution
- Registry priority/ordering
- Per-package registry pinning
- Better error messages for conflicts

---

### Private Registry Auth

**Status**: Planned
**Priority**: Medium

Authentication support for private registries.

**Authentication Methods**:
- SSH keys (for Git-based registries)
- HTTP Basic Auth
- Token-based auth (GitHub tokens, etc.)
- OAuth2 (future)

**Credential Management**:
- Secure credential storage
- Per-registry credentials
- Credential helpers (similar to git-credential)

---

## Developer Experience

### Antivirus Integration

**Status**: Planned
**Priority**: Low

Optional integration with antivirus software.

**Supported Solutions**:
- ClamAV (Linux, macOS)
- Windows Defender (Windows)
- Custom scanner integration

**Features**:
- Scan downloaded archives before extraction
- Scan extracted files before deployment
- Configurable scan policies
- Clear reporting of detections

---

### Better Error Messages

**Status**: Ongoing
**Priority**: High

More actionable error messages throughout Ora.

**Improvements**:
- Contextual error messages
- Suggestions for common fixes
- Link to relevant documentation
- Structured error output (JSON mode for scripting)

**Example**:
```
❌ Package 'foo' not found

Suggestions:
├─ Update registries: ora registry update
├─ Search for similar packages: ora search foo
└─ Check package name spelling
```

---

### Repository Validation (ora lint)

**Status**: Planned
**Priority**: Medium

Validate .repo files before publishing to registries.

**Features**:
- Validate TOML syntax
- Check required vs optional fields
- Verify URL templates and variables
- Detect common mistakes (binary vs binaries, type typos)
- Security configuration validation
- Platform mapping verification

**Commands**:
```bash
# Validate a single .repo file
ora lint package.repo

# Validate all .repo files in a directory
ora lint packages/*.repo

# Show verbose validation output
ora lint package.repo --verbose

# Output as JSON for CI/CD
ora lint package.repo --format json
```

**Example Output**:
```
✅ Checking windman.repo...

✅ Required fields:
  ✓ name: "windman"
  ✓ [source.type]: "github-releases"
  ✓ [source.download.url]: valid template

⚠️  Optional fields missing:
  - description (recommended)
  - [security.checksum] (highly recommended for security)

❌ Errors:
  × [install.binaries]: Field is required but missing
  × [source.download.url]: Invalid template variable {versoin}
    Did you mean: {version}?

💡 Suggestions:
  - Add SHA256 checksums for better security
  - Consider adding GPG signature verification
```

**Use Cases**:
- Registry maintainers validating packages before merging
- CI/CD pipelines ensuring .repo quality
- Package authors debugging parsing errors
- Reducing trial-and-error when creating .repo files

**Related Documentation**:
- See `docs/REPO_SCHEMA.md` for required/optional fields reference
- See "Common Parsing Errors" section for error explanations

---

### Progress Indicators

**Status**: Planned
**Priority**: Medium

Better download and install progress feedback.

**Features**:
- Download progress bars with speed and ETA
- Installation stage indicators
- Parallel operation tracking
- Quiet mode for CI/CD

**Libraries**:
- `indicatif` for progress bars
- Structured logging for machine parsing

---

### Verbose Logging

**Status**: Planned
**Priority**: Medium

Structured logging with different verbosity levels.

**Levels**:
- ERROR: Only errors
- WARN: Warnings and errors
- INFO: General information (default)
- DEBUG: Detailed debug information
- TRACE: Very verbose tracing

**Features**:
- Environment variable control (`ORA_LOG`)
- Flag control (`-v`, `-vv`, `-vvv`)
- JSON output for structured logging
- Log file support

---

## Quality of Life

### Shell Completions

**Status**: Planned
**Priority**: Medium

Auto-completion for popular shells.

**Shells**:
- Bash
- Zsh
- Fish
- PowerShell

**Installation**:
```bash
# Generate completions
ora completions bash > /etc/bash_completion.d/ora
ora completions zsh > /usr/share/zsh/site-functions/_ora
```

---

### Package Aliasing

**Status**: Planned
**Priority**: Low

Allow users to define package aliases.

**Use Cases**:
- Short names for frequently used packages
- Transition from deprecated package names
- Personal naming preferences

**Configuration**:
```toml
# ~/.config/ora/config.toml
[aliases]
k = "kubectl"
tf = "terraform"
```

---

### Dry Run Mode

**Status**: Planned
**Priority**: Low

Preview operations without executing them.

**Commands**:
```bash
ora install --dry-run package
ora update --dry-run --all
ora uninstall --dry-run package
```

**Output**:
- What would be downloaded
- What would be installed/uninstalled
- Disk space impact
- Dependency changes

---

## Platform Support

### Windows Support

**Status**: Partial
**Priority**: High

Full Windows support with platform-specific features.

**Improvements Needed**:
- Windows-specific path handling
- Authenticode signature verification
- Windows Defender integration
- PowerShell completions
- Windows service integration

---

### macOS Support

**Status**: Partial
**Priority**: High

Full macOS support with platform-specific features.

**Improvements Needed**:
- Codesign verification
- Keychain integration for credentials
- macOS-specific binary formats (app bundles)
- Homebrew-like cask support

---

### ARM Support

**Status**: Partial
**Priority**: Medium

Better support for ARM architectures.

**Platforms**:
- ARM64 (aarch64) Linux
- Apple Silicon (M1/M2/M3)
- ARM Windows
- Raspberry Pi

**Improvements**:
- Better architecture detection
- ARM-specific binary selection
- Cross-architecture handling

---

## References

- [Homebrew](https://brew.sh/) - Package manager inspiration
- [Cargo](https://doc.rust-lang.org/cargo/) - Rust package manager
- [npm](https://www.npmjs.com/) - Node package manager
- [SLSA](https://slsa.dev/) - Supply chain security framework
