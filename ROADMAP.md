# Ora Roadmap

This document outlines potential improvements organized by category. Items listed here are not commitments or promises - they represent areas that could be enhanced if contributors are interested.

---

## Security

### Critical

- **GPG Signature Verification**: Full implementation of package signature verification
- **Script Sandboxing**: Isolate post-installation scripts (bubblewrap, seccomp, containers)
- **HTTP Redirect Enforcement**: Actually enforce `allow_redirects` configuration
- **Template Injection Fixes**: Replace unsafe `resolve_template()` in GitLab and CustomAPI providers
- **Environment Variable Filtering**: Filter sensitive env vars from post-install scripts

### High Priority

- **TLS Certificate Pinning**: Implement the configured certificate pinning validation
- **TOCTOU Race Conditions**: Fix time-of-check-time-of-use issues in deployer/extractor
- **Audit Log Integrity**: Add HMAC or signatures to prevent tampering
- **Binary Signature Verification**: Verify codesign (macOS), Authenticode (Windows), ELF signatures (Linux)
- **Package Shadowing Prevention**: Enforce `fail_on_ambiguous_package` configuration

### Medium Priority

- **Git Force Checkout**: Respect `allow_force_checkout` configuration
- **JSON/TOML Size Limits**: Enforce configured size limits in HTTP client
- **Content-Type Validation**: Validate HTTP response content types
- **Rate Limiting**: Add network request rate limiting
- **Checksum Parsing**: Improve filename matching (exact match, not ends_with)

### Low Priority

- **Regex Execution Timeout**: Hard timeout for regex operations
- **Archive Magic Bytes**: Detect archive type by content, not just extension
- **File Permissions Validation**: Validate deployed binary permissions are appropriate

---

## Features

### Package Management

- **Reproducible Builds**: Support for verifying reproducible builds
- **SLSA Provenance**: Support for provenance attestations
- **Version Pinning**: Lockfile mechanism for exact version/checksum pinning
- **Self-Update**: Secure update mechanism for Ora itself (`ora self-update`)
- **Download Resume**: Support resuming interrupted downloads

### Registry

- **Registry Signing**: GPG signatures for registry repositories
- **Multiple Registries**: Better handling of package conflicts across registries
- **Private Registry Auth**: Authentication support for private registries

### Developer Experience

- **Antivirus Integration**: Optional integration with ClamAV, Windows Defender
- **Better Error Messages**: More actionable error messages
- **Progress Indicators**: Better download/install progress feedback
- **Verbose Logging**: Structured logging with different verbosity levels

---

## Architecture

- **Privilege Separation**: Separate components with different privilege levels
- **Modular Providers**: Plugin system for custom package sources
- **Configuration Validation**: Enforce minimum security baseline
- **Offline Mode**: Better support for air-gapped environments

---

## Documentation

- **Security Limitations**: Clear documentation of current security gaps
- **Provider API**: Documentation for implementing custom providers
- **Registry Setup**: Guide for setting up private registries
- **Migration Guide**: Help users migrate from other package managers

---

## Dependencies

- **Update Dependencies**: Keep dependencies up to date
  - Consider upgrading `zip` crate (0.6 → 0.7+)
  - Monitor security advisories for `tar`, `git2`, `reqwest`
- **Reduce Dependencies**: Evaluate if all dependencies are necessary
- **Audit Trail**: Regular `cargo audit` runs

---

## Notes

- This roadmap is not a commitment timeline
- Items may be added, removed, or reprioritized
- Community contributions welcome for any item
- Security fixes take priority over features
