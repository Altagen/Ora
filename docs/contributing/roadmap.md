# Ora — Roadmap

This roadmap is not a commitment or timeline. Items represent areas that could be improved if contributors are interested. Security fixes always take priority over new features.

---

## Release Philosophy

Ora follows [Semantic Versioning](https://semver.org/):

- **Major** (x.0.0) — Breaking changes
- **Minor** (0.x.0) — New features, backward compatible
- **Patch** (0.0.x) — Bug fixes, security updates

Security fixes are released immediately as patch versions with a clear advisory.

---

## 🔴 Critical — Security Hardening

### GPG Signature Verification
Full implementation of package signature verification using GPG/PGP.
- Detached signature (`.sig`) support
- GPG keyring management and key trust verification
- Signature validation before extraction

**Impact**: Prevents installation of tampered packages.

---

### Script Sandboxing
Isolate post-installation scripts to prevent arbitrary code execution.
- **Linux**: bubblewrap (namespaces) + seccomp (syscall filtering)
- Read-only filesystem outside install directory
- No network access by default, timeout enforcement

**Impact**: Limits damage from malicious post-install scripts.

---

### HTTP Redirect Enforcement
The `allow_redirects` config option exists but is not enforced in the HTTP client.
- Implement redirect validation in `src/utils/http.rs`
- Log redirect attempts, fail on unexpected redirects

**Impact**: Prevents redirect-based attacks.

---

### Template Injection Fixes
Unsafe `resolve_template()` usage in provider implementations.
- `src/providers/gitlab.rs` — template resolution without escaping
- `src/providers/custom_api.rs` — user-controlled variables

**Impact**: Prevents command injection and path traversal.

---

### Environment Variable Filtering
Filter sensitive environment variables from post-install scripts.
- SSH keys, API tokens, AWS credentials, database credentials
- Whitelist of safe variables, configurable exceptions

**Impact**: Prevents credential leakage to post-install scripts.

---

## 🟡 High Priority

### Registry Signing
GPG signatures for registry repositories.
- Sign registry commits, verify on clone/pull
- Trusted keyring management, reject unsigned registries

### SLSA Provenance
Support for [SLSA](https://slsa.dev/) provenance attestations.
- Parse and validate in-toto attestations
- Verify build platform and parameters, display to users

### Better Error Messages *(ongoing)*
- Contextual suggestions for common failures
- Links to relevant documentation
- Structured JSON output for scripting

### TLS Certificate Pinning
The certificate pinning config exists but validation is not implemented.
- SHA-256 fingerprint validation with pin rotation

### TOCTOU Race Conditions
Fix time-of-check-time-of-use issues in `src/installer/deployer.rs` and `src/installer/extractor.rs`.
- Atomic operations, file locking during critical sections, re-verification

### Binary Signature Verification
Platform-specific binary signature verification.
- macOS: codesign, Windows: Authenticode, Linux: ELF signatures

### Package Shadowing Prevention
Enforce `fail_on_ambiguous_package` — currently configured but not checked.

### Audit Log Integrity
HMAC-SHA256 signatures on audit log entries to detect tampering.

---

## 🟢 Medium Priority

### Offline Mode & Lockfile

Support for air-gapped and network-restricted environments.

**Lockfile** (`ora.lock`):
```bash
ora lock                    # Generate lockfile from installed packages
ora install --locked        # Install exact versions from lockfile
```

**Offline install**:
```bash
ora download windsurf prometheus   # Pre-fetch packages
ora install --offline windsurf     # Install from cache only
ora install --prefer-cache kubectl # Prefer cache, fall back to network
```

**Cache management**:
```bash
ora cache list / stats / clean
ora cache export --output cache.tar.gz
ora cache import cache.tar.gz
```

**Cache structure**:
```
~/.cache/ora/
├── downloads/        # Cached archives
├── metadata/         # Cached .repo files
└── index.db          # SQLite index
```

---

### Version Pinning
Lockfile mechanism (`ora.lock`) for reproducible installations — exact versions and checksums pinned per package.

### `ora lint` — Repository Validation
Validate `.repo` files before publishing.
```bash
ora lint package.repo           # Validate single file
ora lint packages/*.repo        # Validate directory
ora lint package.repo --format json  # CI/CD output
```
Checks: TOML syntax, required fields, URL template variables, security config, platform mappings.

### Private Registry Auth
- SSH keys for git-based registries
- HTTP Basic Auth and token-based auth (GitHub, GitLab)
- Secure per-registry credential storage

### Progress Indicators
Download progress bars with speed and ETA, parallel operation tracking, quiet mode for CI/CD.

### Shell Completions
```bash
ora completions bash > /etc/bash_completion.d/ora
ora completions zsh > /usr/share/zsh/site-functions/_ora
ora completions fish > ~/.config/fish/completions/ora.fish
```

### Git Force Checkout
Respect `allow_force_checkout` config in Git provider — currently ignored.

### JSON/TOML Size Limits
Enforce configured size limits in HTTP client to prevent DoS via oversized config files.

### Content-Type Validation
Validate HTTP response content types match expectations (JSON, binary, etc.).

### Checksum Parsing
Improve filename matching in `src/installer/verifier.rs` — current `ends_with()` may match wrong files.

---

## ⚪ Low Priority

### Self-Update
`ora self-update` — download latest Ora release, verify integrity, replace binary atomically with rollback on failure.

### Download Resume
HTTP Range requests for resuming interrupted downloads with exponential backoff.

### Dry Run Mode
```bash
ora install --dry-run package    # Preview without executing
ora update --dry-run --all
```

### Package Aliasing
```toml
# ~/.config/ora/config.toml
[aliases]
k = "kubectl"
tf = "terraform"
```

### Regex Execution Timeout
Hard timeout for regex operations to prevent ReDoS attacks.

### Archive Magic Bytes
Detect archive type by content (magic bytes), not just file extension.

### Antivirus Integration
Optional ClamAV (Linux/macOS) or Windows Defender integration — scan archives before extraction.

---

## Contributing

- Pick any item from this roadmap
- Open an issue to discuss your approach before implementing
- Submit a PR with your implementation
- Security fixes are always welcome and will be prioritized

**References**: [OWASP Top 10](https://owasp.org/www-project-top-ten/) — [SLSA Framework](https://slsa.dev/) — [Sigstore](https://www.sigstore.dev/)
