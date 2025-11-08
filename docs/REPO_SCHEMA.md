# Ora .repo File Schema

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

## Validation

Use `ora validate-repo` to check a .repo file:

```bash
ora validate-repo package.repo

# Checks:
# ✓ Valid TOML syntax
# ✓ Required fields present
# ✓ repo_version supported
# ✓ URL templates valid
# ✓ Security config valid
# ⚠ Warnings for allow_insecure
```

## Philosophy Summary

- ✅ **Prefer API discovery** over hardcoded URLs
- ✅ **Keep it simple** for 90% of packages
- ✅ **Explicit warnings** for insecure configurations
- ✅ **Git is the source of truth** - no central server needed
- ✅ **CI validates** .repo files automatically
- ✅ **User-friendly** errors with actionable messages
