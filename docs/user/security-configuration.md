# Ora Security Configuration - Complete Guide

**Version**: 0.2.0

This guide documents Ora's security configuration system.

---

## Quick Reference

Complete list of all security configuration variables:

### Network Security (`[network]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `https_only` | bool | `false` | Allow only HTTPS downloads |
| `allow_redirects` | bool | `true` | Follow HTTP redirects |
| `max_redirects` | usize | `5` | Maximum redirects to follow |
| `block_private_ips` | bool | `true` | Block RFC 1918 private IPs |
| `block_localhost` | bool | `true` | Block localhost/127.0.0.1 |
| `block_link_local` | bool | `true` | Block 169.254.x.x addresses |
| `block_metadata_endpoints` | bool | `true` | Block cloud metadata IPs |
| `allowed_schemes` | Vec<String> | `["https", "http"]` | Allowed URL schemes |
| `max_download_size` | u64 | `2147483648` | Max download size (2 GB) |
| `timeout_seconds` | u64 | `120` | Network timeout |
| `validate_dns_resolution` | bool | `false` | Validate DNS before requests |

### Git Security (`[network.git]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `https_only` | bool | `true` | Force HTTPS for git operations |
| `allowed_schemes` | Vec<String> | `["https"]` | Allowed git URL schemes |
| `max_repo_size` | u64 | `104857600` | Max repo size (100 MB) |
| `timeout_seconds` | u64 | `300` | Git operation timeout |
| `allow_force_checkout` | bool | `false` | Allow git force checkout |

### Security Settings (`[security]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `require_checksums` | bool | `true` | Require checksum verification |
| `require_signatures` | bool | `false` | Require GPG signatures |
| `max_git_size_mb` | u64 | `1024` | Max git repository size (1 GB) - Git bomb protection |

### Extraction Security (`[extraction]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `max_file_size` | u64 | `1073741824` | Max single file size (1 GB) |
| `max_total_size` | u64 | `5368709120` | Max total extracted size (5 GB) |
| `max_file_count` | usize | `100000` | Max files in archive |
| `max_directory_depth` | usize | `50` | Max directory nesting |
| `max_path_length` | usize | `4096` | Max path length in bytes |
| `compression_ratio_warning` | u64 | `100` | Compression ratio threshold |
| `block_symlinks` | bool | `true` | Block symbolic links |
| `block_hardlinks` | bool | `true` | Block hard links |
| `block_device_files` | bool | `true` | Block device files |
| `strip_setuid_bits` | bool | `true` | Strip SUID/SGID bits |

### Script Security (`[scripts]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `require_confirmation` | bool | `true` | Ask before running scripts |
| `enabled` | bool | `true` | Enable post-install scripts |
| `timeout_seconds` | u64 | `300` | Script execution timeout |
| `show_script_content` | bool | `true` | Show script before running |
| `static_analysis` | bool | `true` | Analyze scripts for dangers |
| `block_public_registry_scripts` | bool | `true` | Block scripts from public registries |
| `allowed_interpreters` | Vec<String> | `["sh", "bash"]` | Allowed script interpreters |
| `filter_sensitive_env_vars` | bool | `true` | Filter sensitive environment variables |

### Registry Security (`[registries]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `enforce_trust_levels` | bool | `false` | Enforce public/private trust levels |
| `require_checksums_public` | bool | `true` | Require checksums from public registries |
| `require_checksums_private` | bool | `false` | Require checksums from private registries |
| `require_gpg_signatures` | bool | `false` | Require GPG signatures |
| `allow_package_shadowing` | bool | `true` | Allow same package in multiple registries |
| `fail_on_ambiguous_package` | bool | `false` | Fail if package found in multiple registries |
| `max_registry_size` | u64 | `104857600` | Max registry size (100 MB) |
| `sync_timeout_seconds` | u64 | `300` | Registry sync timeout |

### Validation Security (`[validation]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `max_toml_size` | u64 | `1048576` | Max TOML file size (1 MB) |
| `max_json_size` | u64 | `10485760` | Max JSON response size (10 MB) |

### Regex Validation (`[validation.regex]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `max_compiled_size` | usize | `1048576` | Max compiled regex size (1 MB) |
| `max_dfa_size` | usize | `1048576` | Max DFA size (1 MB) |
| `max_capture_groups` | usize | `10` | Max capture groups |
| `max_pattern_length` | usize | `1000` | Max pattern length |
| `max_matches` | usize | `1000` | Max matches to process |

### Template Validation (`[validation.templates]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `url_encode_variables` | bool | `true` | URL-encode template variables |
| `block_path_traversal` | bool | `true` | Block ../ in variables |
| `block_null_bytes` | bool | `true` | Block null bytes |
| `block_newlines` | bool | `true` | Block newlines |
| `max_variable_length` | usize | `1000` | Max variable value length |

### Resource Limits (`[resources]`)

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `enabled` | bool | `true` | Enable resource limits |
| `max_concurrent_downloads` | usize | `3` | Max parallel downloads |
| `max_memory_bytes` | u64 | `0` | Max memory (0 = unlimited) |
| `max_cache_size_bytes` | u64 | `0` | Max cache size (0 = unlimited) |

---

## 📋 OVERVIEW

Ora now has a **comprehensive security configuration system** that allows administrators to customize all aspects of security according to their threat model.

### Design Principles

✅ **Everything is configurable** - No hardcoded values
✅ **Secure defaults** - Default configuration protects against known attacks
✅ **Production-ready** - Designed for enterprise deployments
✅ **Complete documentation** - Every option is documented
✅ **Runtime validation** - Configurations are validated at load time

---

## 🚀 QUICK START

### Initialize Configuration

```bash
# Create default configuration file
ora security init

# Display current configuration
ora security show

# Reset to default values
ora security reset
```

### Configuration File

The file is automatically created at:

- **Linux**: `~/.config/ora/security.toml`
- **macOS**: `~/Library/Application Support/ora/security.toml`
- **Windows**: `%APPDATA%\ora\security.toml`

---

## 📖 CONFIGURATION CATEGORIES

### 1. 🌐 Network Security (`[network]`)

Controls all network operations.

```toml
[network]
# Force HTTPS only (recommended for production)
https_only = false  # false for compatibility, true for maximum security

# Follow HTTP redirects
allow_redirects = false  # false = more secure

# Maximum number of redirects
max_redirects = 3

# SSRF protection
block_private_ips = true      # Block 10.x, 192.168.x, 172.16-31.x
block_localhost = true         # Block 127.0.0.1, ::1
block_link_local = true        # Block 169.254.x.x
block_metadata_endpoints = true # Block AWS/GCP/Azure metadata

# Allowed URL schemes
allowed_schemes = ["https", "http"]

# Download limits
max_download_size = 2147483648  # 2 GB
timeout_seconds = 300           # 5 minutes

# DNS validation (prevents DNS rebinding)
validate_dns_resolution = true  # IMPORTANT!
```

**Use cases**:

- **Secure enterprise**: `https_only = true`, `allow_redirects = false`
- **Development environment**: Default values
- **Restricted network**: Adjust `allowed_schemes`

---

### 2. 🔒 Git Security (`[network.git]`)

Controls Git operations for registries.

```toml
[network.git]
# Only allow HTTPS for Git (HIGHLY RECOMMENDED)
https_only = true

# Allowed Git schemes
allowed_schemes = ["https"]

# Maximum repository size
max_repo_size = 104857600  # 100 MB

# Timeout for clone/fetch
timeout_seconds = 300

# Allow git checkout --force (DANGEROUS)
allow_force_checkout = false
```

**🚨 WARNING**: Never set `https_only = false` in production!

**Git Bomb Protection** (New in 0.2.0):

Ora now includes comprehensive protection against git bomb attacks:

```toml
[security]
# Maximum git repository size in MB (default: 1024 MB = 1 GB)
max_git_size_mb = 1024
```

**How it works**:
1. **Shallow Clone**: Uses `git clone --depth=1` to only fetch the latest commit
2. **Size Checking**: Measures `.git` directory size after cloning
3. **Automatic Cleanup**: Removes repository if it exceeds the limit
4. **Configurable Limit**: Adjust `max_git_size_mb` for legitimate large repositories

**Example attack scenarios prevented**:

- ✅ **History Bomb**: Millions of commits with large binary files (blocked by shallow clone)
- ✅ **Large Objects Bomb**: Gigantic blobs in git history (blocked by shallow clone)
- ✅ **Packed Bomb**: Malicious git pack files that expand to gigabytes (blocked by size check)

**Configuration example**:

```toml
# For most use cases (default)
[security]
max_git_size_mb = 1024  # 1 GB

# For large legitimate repositories (e.g., with assets)
[security]
max_git_size_mb = 5120  # 5 GB

# For strict security environments
[security]
max_git_size_mb = 512  # 512 MB
```

**Blocked vulnerabilities**:

- ✅ VULN-001: Command injection via `git://`, `ssh://`, `file://`
- ✅ SSRF via alternative Git protocols
- ✅ DoS via oversized repositories
- ✅ **NEW**: Git bomb attacks via malicious history
- ✅ **NEW**: Resource exhaustion via large pack files

---

### 3. 📦 Extraction Security (`[extraction]`)

Protects against zip bombs and path traversal.

```toml
[extraction]
# Anti-zip-bomb limits
max_file_size = 1073741824      # 1 GB per file
max_total_size = 5368709120     # 5 GB total
max_file_count = 100000         # 100k files maximum
max_directory_depth = 50        # 50 levels maximum
max_path_length = 4096          # 4096 characters

# Compression ratio warning threshold
compression_ratio_warning = 100  # 100:1 ratio

# Block dangerous file types
block_symlinks = true       # Block symlinks in archives
block_hardlinks = true      # Block hardlinks
block_device_files = true   # Block /dev/* files
strip_setuid_bits = true    # Strip SUID/SGID bits
```

**Blocked attack scenarios**:

- ✅ Zip bombs (1MB → 10TB)
- ✅ Billion laughs (millions of files)
- ✅ Path traversal (`../../etc/passwd`)
- ✅ Symlink to system files
- ✅ SUID binary injection

---

### 4. 📜 Scripts Security (`[scripts]`)

Controls post-installation script execution.

```toml
[scripts]
# Enable post-install scripts
enabled = true

# Require user confirmation (CRITICAL)
require_confirmation = true

# Display script content before execution
show_script_content = true

# Static analysis for dangerous patterns
static_analysis = true

# Block scripts from public registries by default
block_public_registry_scripts = true

# Execution timeout
timeout_seconds = 300  # 5 minutes

# Allowed interpreters
allowed_interpreters = ["sh", "bash"]

# Filter sensitive environment variables from logs
filter_sensitive_env_vars = true
```

**Security levels**:

| Level | Configuration | Usage |
|--------|--------------|-------|
| **Paranoid** | `enabled = false` | Critical production |
| **Secure** | `require_confirmation = true` (default) | General use |
| **Permissive** | `require_confirmation = false` | Dev/CI |

**⚠️ Known limitations**:

- Scripts execute with full user permissions (no sandbox)
- No CPU/memory limits (TODO: VULN-016)
- Network access not restricted

---

### 5. 🗃️ Registries Security (`[registries]`)

Manages registry trust and validation.

```toml
[registries]
# Apply trust levels (Public/Private)
enforce_trust_levels = true

# Require checksums for public registries
require_checksums_public = true

# Require checksums for private registries
require_checksums_private = false

# Require GPG signatures (not implemented)
require_gpg_signatures = false

# Allow multiple registries with same package
allow_package_shadowing = false

# Fail if package found in multiple registries
fail_on_ambiguous_package = true

# Synchronization limits
max_registry_size = 104857600  # 100 MB
sync_timeout_seconds = 300
```

**Recommended trust policy**:

```toml
# For enterprise registries
[registries]
enforce_trust_levels = true
require_checksums_public = true   # External registries
require_checksums_private = false # Trusted internal registries
fail_on_ambiguous_package = true  # Avoid substitution
```

**Blocked vulnerabilities**:

- ✅ VULN-014: Confusion attack substitution
- ✅ VULN-013: Registry trust level bypass
- ✅ VULN-017: Registry DoS by excessive size

---

### 6. ✅ Input Validation (`[validation]`)

Limits injection attacks and data bombs.

```toml
[validation]
# File size limits
max_toml_size = 1048576    # 1 MB
max_json_size = 10485760   # 10 MB

[validation.regex]
# ReDoS protection
max_compiled_size = 1048576   # 1 MB
max_dfa_size = 1048576        # 1 MB
max_capture_groups = 50
max_pattern_length = 1000
max_matches = 1000

[validation.templates]
# Template injection protection
url_encode_variables = true
block_path_traversal = true   # Block ../
block_null_bytes = true       # Block \0
block_newlines = true         # Block \n
max_variable_length = 1024
```

**Blocked attacks**:

- ✅ VULN-009: ReDoS (regex catastrophic backtracking)
- ✅ VULN-008: Template injection
- ✅ VULN-010: TOML bomb
- ✅ VULN-011: JSON bomb

---

### 7. ⚙️ Resource Limits (`[resources]`)

Prevents resource exhaustion.

```toml
[resources]
# Enable resource limits
enabled = true

# Maximum concurrent downloads
max_concurrent_downloads = 3

# Maximum memory (0 = unlimited)
max_memory_bytes = 0

# Maximum cache size
max_cache_size_bytes = 10737418240  # 10 GB
```

---

## 🎯 RECOMMENDED CONFIGURATION PROFILES

### Profile 1: Enterprise Production (Maximum Security)

```toml
[network]
https_only = true
allow_redirects = false
validate_dns_resolution = true

[network.git]
https_only = true

[extraction]
block_symlinks = true
strip_setuid_bits = true

[scripts]
enabled = false  # Disable completely

[registries]
enforce_trust_levels = true
require_checksums_public = true
fail_on_ambiguous_package = true

[validation.templates]
url_encode_variables = true
block_path_traversal = true
```

---

### Profile 2: Development Environment (Balanced)

```toml
# Use default values
# They are already well-balanced
```

---

### Profile 3: CI/CD Pipeline (Automated)

```toml
[scripts]
require_confirmation = false  # No interaction
enabled = true

[registries]
require_checksums_public = true
fail_on_ambiguous_package = true

[resources]
max_concurrent_downloads = 5  # Faster
```

**⚠️ Use `--insecure` flag cautiously in pipelines!**

---

## 🛠️ ADVANCED CUSTOMIZATION

### Environment Variables

All configurations can be overridden via environment variables:

```bash
# Format: ORA_SECURITY_<SECTION>_<KEY>
export ORA_SECURITY_NETWORK_HTTPS_ONLY=true
export ORA_SECURITY_SCRIPTS_REQUIRE_CONFIRMATION=false
export ORA_SECURITY_EXTRACTION_MAX_FILE_SIZE=2147483648
```

**Priority**: Env vars > security.toml > Defaults

---

### Programmatic Configuration

```rust
use ora::config::SecurityConfig;

// Load configuration
let mut config = SecurityConfig::load()?;

// Modify
config.network.https_only = true;
config.scripts.timeout_seconds = 600;

// Save
config.save()?;
```

---

## 📊 AUDIT AND MONITORING

### Check Current Configuration

```bash
ora security show
```

Displays:

- ✅ All enabled options
- ⚠️ Disabled options (with warnings)
- 📁 Configuration file path

### Validate Configuration

```bash
# Configuration is automatically validated at load time
# Errors are displayed clearly
```

### Security Logs

Security decisions are logged at different levels:

```txt
INFO  - Configuration loaded from ~/.config/ora/security.toml
WARN  - HTTPS-only mode disabled, HTTP downloads allowed
WARN  - DNS validation disabled, vulnerable to rebinding attacks
ERROR - Git URL 'git://evil.com/repo' blocked (HTTPS only mode)
```

---

## 🔐 COMPATIBILITY MATRIX

| Feature | Security Level | Compatibility | Performance |
|---------|-------------------|---------------|-------------|
| HTTPS Only | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| DNS Validation | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| Block Redirects | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Block Symlinks | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Script Confirmation | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| Disable Scripts | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ |

---

## 📚 REFERENCES

### Vulnerabilities Fixed by Configuration

- **VULN-001**: Git command injection → `network.git.https_only`
- **VULN-008**: Template injection → `validation.templates.*`
- **VULN-009**: ReDoS → `validation.regex.*`
- **VULN-010/011**: TOML/JSON bombs → `validation.max_*_size`
- **VULN-013**: Trust bypass → `registries.enforce_trust_levels`
- **VULN-014**: Substitution → `registries.fail_on_ambiguous_package`
- **VULN-016**: Script timeout → `scripts.timeout_seconds`

---

## 🚨 IMPORTANT NOTES

### Current Limitations

1. **GPG not implemented**: `require_gpg_signatures` does nothing currently (VULN-012)
2. **No script sandbox**: Scripts execute with full permissions
3. **No CPU/memory limits**: Scripts can consume all resources

---

## 📞 SUPPORT

### Configuration Issues

```bash
# Check current configuration
ora security show

# Reset if corrupted
ora security reset

# Generate documented example
ora security init
```

### Reporting Security Issues

If you discover a vulnerability:

1. **DO NOT** create a public issue
2. Report via [GitHub Security Advisories](https://github.com/Altagen/Ora/security/advisories/new)
3. Include: version, configuration, steps to reproduce
