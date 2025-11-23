# Security Roadmap

This document outlines security improvements and hardening planned for Ora.

---

## Critical Priority

### GPG Signature Verification

**Status**: Planned
**Priority**: Critical

Full implementation of package signature verification using GPG/PGP.

**Requirements**:
- Support for detached signatures (`.sig` files)
- GPG keyring management
- Key trust verification
- Signature validation before extraction
- Clear error messages on signature mismatch

**Security Impact**: Prevents installation of tampered packages.

---

### Script Sandboxing

**Status**: Planned
**Priority**: Critical

Isolate post-installation scripts to prevent arbitrary code execution.

**Technologies**:
- **bubblewrap**: Linux namespace sandboxing
- **seccomp**: System call filtering
- **containers**: Optional container-based isolation

**Restrictions**:
- Limited filesystem access (read-only outside install directory)
- No network access by default
- Restricted system call access
- Timeout enforcement

**Security Impact**: Limits damage from malicious post-install scripts.

---

### HTTP Redirect Enforcement

**Status**: Planned
**Priority**: Critical

Actually enforce the `allow_redirects` configuration option.

**Current State**: Configuration exists but is not enforced in HTTP client.

**Required Changes**:
- Implement redirect validation in `src/utils/http.rs`
- Respect `allow_redirects` from security configuration
- Log redirect attempts
- Fail installation on unexpected redirects

**Security Impact**: Prevents redirect-based attacks.

**Files to modify**: `src/utils/http.rs`

---

### Template Injection Fixes

**Status**: Planned
**Priority**: Critical

Replace unsafe `resolve_template()` usage in provider implementations.

**Vulnerable Code**:
- GitLab provider: Template resolution without proper escaping
- CustomAPI provider: User-controlled template variables

**Required Changes**:
- Input validation and sanitization
- Safe template engine or manual escaping
- Whitelist allowed template variables

**Security Impact**: Prevents command injection and path traversal.

**Files to modify**:
- `src/providers/gitlab.rs`
- `src/providers/custom_api.rs`

---

### Environment Variable Filtering

**Status**: Planned
**Priority**: Critical

Filter sensitive environment variables from post-install scripts.

**Variables to Filter**:
- SSH keys and tokens (`SSH_AUTH_SOCK`, `SSH_AGENT_PID`)
- API tokens (`GITHUB_TOKEN`, `GITLAB_TOKEN`, etc.)
- AWS credentials (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`)
- Database credentials
- User session tokens

**Implementation**:
- Whitelist of safe variables
- Clear logging of filtered variables
- Configuration option to allow specific variables

**Security Impact**: Prevents credential leakage to post-install scripts.

---

## High Priority

### TLS Certificate Pinning

**Status**: Planned
**Priority**: High

Implement the configured certificate pinning validation.

**Current State**: Configuration exists but validation not implemented.

**Requirements**:
- SHA-256 fingerprint validation
- Support for certificate chains
- Pin rotation mechanism
- Clear error messages on mismatch

**Security Impact**: Prevents MITM attacks via certificate substitution.

---

### TOCTOU Race Conditions

**Status**: Planned
**Priority**: High

Fix time-of-check-time-of-use issues in deployer and extractor.

**Vulnerable Scenarios**:
- File verification then extraction (file could be swapped)
- Checksum validation then deployment (archive could be modified)

**Required Changes**:
- Atomic operations where possible
- File locking during critical sections
- Re-verification after operations

**Security Impact**: Prevents file swap attacks during installation.

**Files to modify**:
- `src/installer/deployer.rs`
- `src/installer/extractor.rs`

---

### Audit Log Integrity

**Status**: Planned
**Priority**: High

Add HMAC or signatures to audit logs to prevent tampering.

**Requirements**:
- HMAC-SHA256 signatures for each log entry
- Secret key management
- Log verification command (`ora audit verify`)
- Tamper detection

**Security Impact**: Ensures audit trail cannot be modified by attackers.

**Files to modify**: `src/security/mod.rs` (audit logger)

---

### Binary Signature Verification

**Status**: Planned
**Priority**: High

Verify platform-specific binary signatures.

**Platform Support**:
- **macOS**: codesign verification
- **Windows**: Authenticode signature verification
- **Linux**: ELF signatures (if available)

**Implementation**:
- Platform-specific verification modules
- Optional enforcement (configurable)
- Clear reporting of unsigned binaries

**Security Impact**: Validates binary authenticity from official publishers.

---

### Package Shadowing Prevention

**Status**: Planned
**Priority**: High

Enforce `fail_on_ambiguous_package` configuration.

**Current State**: Configuration exists but not enforced.

**Scenario**:
Multiple registries provide a package with the same name.

**Required Changes**:
- Detect package conflicts across registries
- Fail or warn based on configuration
- Provide clear resolution steps

**Security Impact**: Prevents malicious packages from shadowing legitimate ones.

---

## Medium Priority

### Git Force Checkout

**Status**: Planned
**Priority**: Medium

Respect `allow_force_checkout` configuration in Git provider.

**Current State**: Git operations may use force checkout regardless of config.

**Required Changes**:
- Check configuration before git operations
- Fail safely if conflicts exist and force is disallowed

**Files to modify**: `src/providers/github.rs`, `src/providers/gitlab.rs`

---

### JSON/TOML Size Limits

**Status**: Planned
**Priority**: Medium

Enforce configured size limits in HTTP client.

**Purpose**: Prevent DoS via oversized configuration files.

**Implementation**:
- Check `Content-Length` header
- Stream parsing with size limits
- Configurable limits in security.toml

**Files to modify**: `src/utils/http.rs`

---

### Content-Type Validation

**Status**: Planned
**Priority**: Medium

Validate HTTP response content types match expectations.

**Checks**:
- Expecting JSON? Verify `application/json`
- Expecting binary? Verify `application/octet-stream`
- Reject unexpected content types

**Security Impact**: Prevents content-type confusion attacks.

---

### Rate Limiting

**Status**: Planned
**Priority**: Medium

Add network request rate limiting to prevent abuse.

**Features**:
- Configurable requests per second
- Per-domain rate limiting
- Exponential backoff on failures

**Security Impact**: Prevents abuse and reduces risk of being blocked by providers.

---

### Checksum Parsing

**Status**: Planned
**Priority**: Medium

Improve filename matching in checksum files.

**Current Issue**: Uses `ends_with()` which may match wrong files.

**Required Changes**:
- Exact filename matching
- Support for common checksum file formats (SHA256SUMS, etc.)
- Better error messages on mismatch

**Files to modify**: `src/installer/verifier.rs`

---

## Low Priority

### Regex Execution Timeout

**Status**: Planned
**Priority**: Low

Hard timeout for regex operations to prevent ReDoS.

**Implementation**:
- Timeout wrapper around regex matching
- Configurable timeout duration
- Fail safely on timeout

**Security Impact**: Prevents regex denial-of-service attacks.

---

### Archive Magic Bytes

**Status**: Planned
**Priority**: Low

Detect archive type by content (magic bytes), not just extension.

**Purpose**: Prevent extension-based attacks.

**Implementation**:
- Read file header
- Detect format (zip, tar.gz, tar.xz, etc.)
- Validate matches expected format

**Files to modify**: `src/installer/extractor.rs`

---

### File Permissions Validation

**Status**: Planned
**Priority**: Low

Validate deployed binary permissions are appropriate.

**Checks**:
- Binaries should be executable (755 or similar)
- Config files should not be executable
- Setuid/setgid detection and warning

**Security Impact**: Prevents permission-based vulnerabilities.

---

## References

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [SLSA Framework](https://slsa.dev/)
- [Sigstore](https://www.sigstore.dev/)
