# Ora .repo File Schema

## Quick Reference: Required vs Optional Fields

### ✅ Required Fields (Must be present)

```toml
name = "package-name"           # REQUIRED: Package identifier
description = "..."             # REQUIRED: Short description

[source]
type = "github-releases"        # REQUIRED: Provider type

[install]
# At least one of these must define how to install:
# - binaries = ["binary"]
# - files = [...]
# - Or direct_binary = true (for standalone binaries)

[security]
# Section must exist (can be empty for defaults)
```

### ⚠️ Conditionally Required

**For `github-releases` or `gitlab-releases`:**
```toml
[source]
repo = "owner/repository"       # REQUIRED for GitHub/GitLab providers
```

**For `custom-api`, `webpage-scraping`, or `direct-url`:**
```toml
[source.download]
url = "..."                     # REQUIRED: Download URL template
# OR
urls = { ... }                  # REQUIRED: Platform-specific URLs (for direct-url)
```

**For security (highly recommended):**
```toml
[security.checksum]             # RECOMMENDED: Checksum verification
url = "..."
algorithm = "sha256"
```

### 📋 Optional Fields

```toml
homepage = "https://..."        # Optional: Project homepage

[platform]                      # Optional: Platform mappings
[platform.os_map]
linux = "linux"

[platform.arch_map]
x86_64 = "amd64"

[install]
mode = "userland"              # Optional: Defaults to userland
post_install = "script..."     # Optional: Post-install script

[security]
allow_insecure = false         # Optional: Defaults to false
[security.gpg]                 # Optional: GPG signature verification
signature_url = "..."

[metadata]                     # Optional: Package metadata
license = "MIT"
authors = ["..."]
tags = ["..."]
```

---

## Philosophy

1. **90% case**: Simple, works with GitHub/GitLab API automatically
2. **9% case**: Custom API with minimal config
3. **1% case**: Hardcoded URLs with explicit warnings

## Basic Schema (90% of packages)

```toml
name = "package-name"
description = "Short description"
homepage = "https://..."            # Optional

[source]
type = "github-releases"            # or "gitlab-releases"
repo = "owner/repository"           # GitHub/GitLab repo

# Ora automatically:
# - Fetches releases via API
# - Extracts versions
# - Builds download URLs

[source.download]
# Template for download URL (uses API data)
url = "https://github.com/{repo}/releases/download/v{version}/{name}-{version}-{os}-{arch}.tar.gz"

# Optional: Asset name pattern if different from URL
asset_pattern = "{name}-{version}-{os}-{arch}.tar.gz"

[platform]
# Optional: OS/arch mapping
[platform.os_map]
linux = "linux"
darwin = "macos"

[platform.arch_map]
x86_64 = "amd64"
aarch64 = "arm64"

[install]
mode = "userland"                   # or "system"
binaries = ["binary-name"]          # Main executable(s)

# Optional: Additional files
[install.extras]
completions = ["completions/*"]
man_pages = ["man/*"]

[security.checksum]
algorithm = "sha256"

# Option 1: Separate checksum file (recommended)
url = "https://github.com/{repo}/releases/download/v{version}/checksums.txt"
format = "multi-hash"               # or "single-hash"

# Option 2: Inline checksum in asset list (GitHub provides this)
# Ora will automatically use GitHub's checksums if available

# Option 3: No checksum (requires explicit allow)
# (omit checksum section)
```

## Custom API (9% of packages)

```toml
name = "package-name"

[source]
type = "custom-api"

[source.version]
# How to discover versions
discovery_url = "https://api.example.com/releases"
discovery_type = "json"
json_path = "$.releases[*].version"  # JSONPath to extract versions

[source.download]
url = "https://cdn.example.com/{version}/{name}-{os}-{arch}.tar.gz"

[install]
mode = "userland"
binaries = ["binary"]

[security.checksum]
algorithm = "sha256"
url = "https://cdn.example.com/{version}/checksums.txt"
format = "multi-hash"
```

## Direct URLs (1% of packages - with warnings)

```toml
name = "proprietary-tool"

[source]
type = "direct-url"

[source.download.urls]
# Platform-specific hardcoded URLs
linux_x86_64 = "https://cdn.example.com/stable/abc123/tool-linux-x64.tar.gz"
linux_aarch64 = "https://cdn.example.com/stable/abc123/tool-linux-arm64.tar.gz"
darwin_x86_64 = "https://cdn.example.com/stable/abc123/tool-macos-x64.tar.gz"

[install]
mode = "userland"
binaries = ["tool"]

[security]
allow_insecure = true

# ⚠️ Ora will automatically warn:
# "This package uses hardcoded URLs. Version cannot be determined automatically.
#  Updates require manual .repo file changes. Use --allow-insecure to install."
```

## Available Variables

All templates support these variables:

- `{name}` - Package name
- `{version}` - Version being installed
- `{os}` - Detected OS (or mapped via platform.os_map)
- `{arch}` - Detected architecture (or mapped via platform.arch_map)
- `{repo}` - Repository path (for GitHub/GitLab)
- `{commit}` - Git commit hash (if available from API)

## Security Options

```toml
[security.checksum]
algorithm = "sha256" | "sha512"

# Format 1: Multi-hash file (one hash per file)
# Example: SHA256SUMS.txt
# abc123...  package-1.0.0-linux-x64.tar.gz
# def456...  package-1.0.0-macos-x64.tar.gz
format = "multi-hash"
url = "https://..."

# Format 2: Single hash (one file = one hash)
# Example: package-1.0.0.tar.gz.sha256
format = "single-hash"
url = "https://..."

# Format 3: Inline hash
hash = "abc123..."

[security.gpg]
# Optional: GPG signature verification
signature_url = "https://.../package.tar.gz.asc"
key_url = "https://.../public.key"
# or
key_id = "ABCD1234"
keyserver = "keys.openpgp.org"

[security]
allow_insecure = false  # Set to true to skip checksum/signature verification
```

## Post-Install Scripts

```toml
[install.post_install]
script = """
#!/bin/bash
echo "Setting up {name}..."
# Can use {install_dir}, {config_dir} variables
"""

# Or external script from package
script_file = "scripts/post-install.sh"
```

## Registry Structure

Simple flat structure (recommended):

```txt
registry/
├── packages/
│   ├── package1.repo
│   ├── package2.repo
│   └── package3.repo
└── README.md
```

Complex structure (only if needed):

```txt
registry/
├── packages/
│   ├── simple-package.repo           # Most packages
│   └── complex-package/              # Only if frequent changes
│       ├── metadata.toml             # Optional: versioning metadata
│       └── package.repo
└── registry.toml                     # Optional: registry policies
```

## Example: Complete GitHub Package

```toml
name = "ripgrep"
description = "A line-oriented search tool that recursively searches the current directory for a regex pattern"
homepage = "https://github.com/BurntSushi/ripgrep"

[source]
type = "github-releases"
repo = "BurntSushi/ripgrep"

[source.download]
url = "https://github.com/{repo}/releases/download/{version}/ripgrep-{version}-{arch}-unknown-linux-gnu.tar.gz"
asset_pattern = "ripgrep-{version}-{arch}-unknown-linux-gnu.tar.gz"

[platform]
[platform.arch_map]
x86_64 = "x86_64"
aarch64 = "aarch64"

[install]
mode = "userland"
binaries = ["rg"]

[install.extras]
completions = ["complete/*"]
man_pages = ["doc/rg.1"]

[security.checksum]
algorithm = "sha256"
url = "https://github.com/{repo}/releases/download/{version}/ripgrep-{version}-{arch}-unknown-linux-gnu.tar.gz.sha256"
format = "single-hash"
```

## Common Parsing Errors and How to Fix Them

### ❌ Error: "missing field `name`"
**Cause:** Required field `name` is not present in the .repo file.

**Fix:**
```toml
name = "your-package-name"  # Add this at the top
```

---

### ❌ Error: "missing field `description`"
**Cause:** Required field `description` is not present.

**Fix:**
```toml
description = "Brief description of what this package does"
```

---

### ❌ Error: "missing field `type`" in [source]
**Cause:** The `type` field in `[source]` section is required.

**Fix:**
```toml
[source]
type = "github-releases"  # or "gitlab-releases", "custom-api", etc.
```

---

### ❌ Error: "missing field `repo`" for github-releases
**Cause:** `repo` is required when using `type = "github-releases"` or `gitlab-releases`.

**Fix:**
```toml
[source]
type = "github-releases"
repo = "owner/repository"  # e.g., "BurntSushi/ripgrep"
```

---

### ❌ Error: "unknown variant `github`" for type
**Cause:** Invalid provider type. Must use kebab-case.

**Fix:**
```toml
# ❌ Wrong:
type = "github"
type = "GithubReleases"

# ✅ Correct:
type = "github-releases"
```

Valid types: `github-releases`, `gitlab-releases`, `custom-api`, `direct-url`, `webpage-scraping`

---

### ❌ Error: "unknown field `binary`"
**Cause:** Field name is wrong. The correct field is `binaries` (plural).

**Fix:**
```toml
# ❌ Wrong:
[install]
binary = ["myapp"]

# ✅ Correct:
[install]
binaries = ["myapp"]
```

---

### ❌ Error: "invalid type: string, expected a sequence" for binaries
**Cause:** `binaries` must be an array, not a string.

**Fix:**
```toml
# ❌ Wrong:
binaries = "myapp"

# ✅ Correct:
binaries = ["myapp"]
```

---

### ❌ Error: Package installs but binary not found
**Cause:** No installation instructions provided.

**Fix:** At least one of these must be present:
```toml
[install]
binaries = ["binary-name"]  # For executables

# OR

[install]
files = [
  { src = "path/in/archive", dst = "path/in/install" }
]

# OR (for standalone binaries with no archive)

[install]
direct_binary = true
binaries = ["binary-name"]
```

---

### ❌ Warning: "Package installed without checksum verification"
**Cause:** No `[security.checksum]` section defined.

**Impact:** Security risk - package integrity not verified.

**Fix:** Add checksum verification (highly recommended):
```toml
[security.checksum]
url = "https://example.com/checksums.txt"
algorithm = "sha256"
format = "multi-hash"  # or "single-hash"
```

To bypass (not recommended):
```toml
[security]
allow_insecure = true
```

---

### ❌ Error: "Invalid URL template variable"
**Cause:** Using undefined or misspelled variable in URL template.

**Valid variables:**
- `{name}` - Package name
- `{version}` - Version being installed
- `{os}` - Operating system (e.g., "linux", "darwin")
- `{arch}` - Architecture (e.g., "x86_64", "aarch64")
- `{repo}` - Repository path (GitHub/GitLab only)

**Fix:**
```toml
# ❌ Wrong:
url = "https://example.com/{versoin}/package.tar.gz"  # Typo: "versoin"

# ✅ Correct:
url = "https://example.com/{version}/package.tar.gz"
```

---

### ❌ Error: TOML syntax error
**Cause:** Invalid TOML syntax.

**Common issues:**
```toml
# ❌ Wrong: Missing quotes for strings with special chars
url = https://example.com/path

# ✅ Correct:
url = "https://example.com/path"

# ❌ Wrong: Wrong section syntax
[source][download]

# ✅ Correct:
[source.download]

# ❌ Wrong: Trailing comma in array
binaries = ["bin1", "bin2",]

# ✅ Correct:
binaries = ["bin1", "bin2"]
```

---

### ⚠️ Warning: Package requires --allow-insecure flag
**Cause:** Package has security concerns:
- No checksum verification
- Hardcoded URLs (type = "direct-url")
- allow_insecure = true in config

**Fix (recommended):** Add proper security verification:
```toml
[security.checksum]
url = "https://..."
algorithm = "sha256"
```

**Fix (not recommended):** Install with flag:
```bash
ora install package --allow-insecure
```

---

## Philosophy Summary

- ✅ **Prefer API discovery** over hardcoded URLs
- ✅ **Keep it simple** for 90% of packages
- ✅ **Explicit warnings** for insecure configurations
- ✅ **Git is the source of truth** - no central server needed
- ✅ **CI validates** .repo files automatically
- ✅ **User-friendly** errors with actionable messages
