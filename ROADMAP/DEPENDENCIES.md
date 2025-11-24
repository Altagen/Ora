# Dependencies Roadmap

This document outlines dependency management and maintenance for Ora.

---

## Dependency Updates

### Regular Update Schedule

**Status**: Planned
**Priority**: High

Establish a regular schedule for dependency updates.

**Schedule**:
- **Security updates**: Immediately upon advisory
- **Minor updates**: Monthly
- **Major updates**: Quarterly (with testing period)

**Process**:
1. Check for updates: `cargo outdated`
2. Review changelogs for breaking changes
3. Update dependencies
4. Run full test suite
5. Update documentation if needed
6. Create PR with changes

---

### Critical Dependencies to Monitor

**High Priority** (security-sensitive):

| Dependency | Current | Latest | Notes |
|-----------|---------|--------|-------|
| `reqwest` | - | - | HTTP client, security-critical |
| `tokio` | - | - | Async runtime |
| `rustls` | - | - | TLS implementation |
| `tar` | - | - | Archive extraction |
| `zip` | 0.6.x | 0.7+ | Consider upgrading |
| `git2` | - | - | Git operations |
| `toml` | - | - | Config parsing |
| `serde` | - | - | Serialization |

**Medium Priority** (functionality):

| Dependency | Notes |
|-----------|-------|
| `clap` | CLI parsing |
| `semver` | Version comparison |
| `sha2` | Checksums |
| `regex` | Pattern matching |
| `chrono` | Time handling |

---

### Specific Upgrade Plans

#### Zip Crate (0.6 → 0.7+)

**Status**: Planned
**Priority**: Medium

**Current**: `zip = "0.6"`
**Target**: `zip = "0.7"` or later

**Breaking Changes**:
- API changes in ZipArchive
- Different error handling
- New features for zip64 support

**Migration Steps**:
1. Review zip 0.7 changelog
2. Update code to new API
3. Test with large archives (> 4GB)
4. Verify Windows zip support
5. Update documentation

**Benefits**:
- Better large file support
- Security fixes
- Performance improvements

---

#### Tokio Runtime Updates

**Status**: Ongoing
**Priority**: High

Keep Tokio up-to-date for security and performance.

**Considerations**:
- Breaking changes in major versions
- Runtime flag changes
- New features (io-uring on Linux)

**Testing**:
- Async operations still work correctly
- No deadlocks or race conditions
- Performance hasn't regressed

---

## Dependency Reduction

### Evaluate Necessity of Dependencies

**Status**: Planned
**Priority**: Medium

Review all dependencies to determine if they're truly needed.

**Questions for Each Dependency**:
1. Is this functionality core to Ora?
2. Could we implement this ourselves simply?
3. Is there a lighter alternative?
4. What's the maintenance status of this crate?

**Candidates for Removal/Replacement**:

**1. Consider Custom Implementation**:
- Simple regex patterns (instead of heavy regex engine)
- Basic HTTP client (for simple requests)
- Minimal TOML parser (if we control format)

**2. Lighter Alternatives**:
- `ureq` instead of `reqwest` (lighter HTTP client)
- `rustls` instead of `openssl` (pure Rust TLS)
- `flate2` instead of multiple compression libraries

**3. Feature Flags**:
Use feature flags to make dependencies optional:
```toml
[dependencies]
git2 = { version = "0.18", optional = true }

[features]
git-provider = ["git2"]
```

---

### Dependency Tree Analysis

**Status**: Planned
**Priority**: Medium

Regular analysis of the dependency tree.

**Commands**:
```bash
# View full dependency tree
cargo tree

# Find duplicate dependencies
cargo tree --duplicates

# Identify large dependencies
cargo bloat --release
```

**Goals**:
- Minimize total dependency count
- Eliminate duplicate versions
- Reduce binary size

---

## Security Auditing

### Cargo Audit Integration

**Status**: Planned
**Priority**: Critical

Regular security audits of dependencies.

**Process**:
```bash
# Install cargo-audit
cargo install cargo-audit

# Check for vulnerabilities
cargo audit

# Check in CI/CD
cargo audit --deny warnings
```

**Integration**:
- Run in GitHub Actions on every PR
- Block PRs with security vulnerabilities
- Daily scheduled audits
- Notifications on new advisories

---

### RustSec Advisory Monitoring

**Status**: Planned
**Priority**: High

Monitor RustSec advisory database for Ora's dependencies.

**Actions on Advisory**:
1. Assess severity and impact
2. Update dependency immediately (critical)
3. Test thoroughly
4. Release patch version
5. Announce security update

**Resources**:
- [RustSec Database](https://rustsec.org/)
- GitHub Security Advisories
- Dependabot alerts

---

### Supply Chain Security

**Status**: Planned
**Priority**: High

Verify dependencies aren't compromised.

**Measures**:

**1. Crate Verification**:
- Check crate popularity and maintenance
- Verify crate authors
- Review source code for suspicious patterns

**2. Checksum Verification**:
- Verify Cargo.lock checksums
- Use `cargo verify-project`

**3. Dependency Pinning**:
- Pin dependencies to specific versions
- Review updates carefully
- Test thoroughly before updating

**4. Minimal Dependencies**:
- Fewer dependencies = smaller attack surface
- Prefer well-known, audited crates

---

## Build Dependencies

### Optimize Build Times

**Status**: Planned
**Priority**: Medium

Reduce build times for better developer experience.

**Strategies**:

**1. Use Workspace Features**:
```toml
[workspace]
resolver = "2"

[profile.dev]
split-debuginfo = "unpacked"  # Faster builds on macOS/Linux
```

**2. Parallel Builds**:
```bash
cargo build --jobs $(nproc)
```

**3. Incremental Compilation**:
```toml
[profile.dev]
incremental = true
```

**4. Pre-built Dependencies**:
- Use `sccache` for shared compilation cache
- Binary caching in CI/CD

---

### Cross-Compilation Support

**Status**: Partial
**Priority**: Medium

Ensure dependencies support cross-compilation.

**Target Platforms**:
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64/arm64)
- Windows (x86_64)

**Problematic Dependencies**:
- Native libraries (openssl, git2)
- Platform-specific code

**Solutions**:
- Use pure Rust alternatives where possible
- Provide pre-built binaries for native deps
- Document cross-compilation process

---

## Documentation Dependencies

### Keep Documentation Up-to-Date

**Status**: Planned
**Priority**: Medium

Ensure dependency documentation reflects current usage.

**Tasks**:
- Update Cargo.toml examples
- Document feature flags
- Explain optional dependencies
- Security implications of dependencies

---

## Dependency Licenses

### License Compliance

**Status**: Planned
**Priority**: High

Ensure all dependencies have compatible licenses.

**Acceptable Licenses**:
- MIT
- Apache-2.0
- BSD-3-Clause
- ISC
- Unlicense

**Incompatible Licenses**:
- GPL (copyleft requirements)
- AGPL
- Proprietary licenses

**Tools**:
```bash
# Install cargo-license
cargo install cargo-license

# Check licenses
cargo license

# Deny incompatible licenses in CI
cargo deny check licenses
```

**Configuration** (`.cargo/deny.toml`):
```toml
[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
    "ISC",
    "Unlicense",
]
deny = [
    "GPL-3.0",
    "AGPL-3.0",
]
```

---

## Dependency Metrics

### Track Dependency Health

**Status**: Planned
**Priority**: Low

Monitor the health of dependencies.

**Metrics**:
- Last update date
- Number of open issues
- Response time to issues
- Number of contributors
- Download count (popularity)
- Test coverage

**Tools**:
- [libs.rs](https://libs.rs/) - Crate quality metrics
- [crates.io](https://crates.io/) - Download stats
- GitHub Insights - Maintenance activity

**Red Flags**:
- No updates in > 1 year
- Many unresolved issues
- Single maintainer (bus factor)
- Low test coverage
- No CI/CD

---

## Future Considerations

### WASM Support

**Status**: Future consideration
**Priority**: Low

Ensure dependencies support WebAssembly compilation.

**Challenges**:
- No filesystem access in WASM
- No network access (without JS glue)
- No threading (yet)

**Potentially Problematic**:
- `tokio` (async runtime)
- `reqwest` (HTTP client)
- Filesystem operations

---

### No-std Support

**Status**: Future consideration
**Priority**: Low

Consider embedded/no-std environments.

**Current State**: Ora requires std

**Benefits of no-std**:
- Smaller binary size
- Embedded systems support
- Bare metal deployment

**Challenges**:
- Most dependencies require std
- Async runtime requires std
- Networking requires std

---

## Dependency Update Log

Track major dependency updates here.

### 2024

| Date | Dependency | From | To | Notes |
|------|-----------|------|----|----|
| - | - | - | - | - |

---

## References

- [Cargo Book - Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [RustSec Advisory Database](https://rustsec.org/)
- [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated)
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)
