# Quick Start: Creating Your First .repo File

This guide will help you create a `.repo` file in 5-10 minutes, using real examples.

## Table of Contents

- [Choose Your Package Type](#choose-your-package-type)
- [Recipe 1: Simple GitHub Release (5 minutes)](#recipe-1-simple-github-release-5-minutes)
- [Recipe 2: GitHub Release with Platform-Specific URLs (10 minutes)](#recipe-2-github-release-with-platform-specific-urls-10-minutes)
- [Recipe 3: Direct Binary Download (5 minutes)](#recipe-3-direct-binary-download-5-minutes)
- [Testing Your .repo File](#testing-your-repo-file)
- [Common Issues and Quick Fixes](#common-issues-and-quick-fixes)

---

## Choose Your Package Type

Before starting, identify which type your package is:

| Type | Example | Complexity |
|------|---------|------------|
| **GitHub Release with archives** | Prometheus, ripgrep | ⭐⭐ Easy |
| **GitHub Release, different platforms** | jq, fd | ⭐⭐⭐ Medium |
| **Direct binary (no archive)** | Single executables | ⭐ Very Easy |
| **Custom API** | Non-GitHub sources | ⭐⭐⭐⭐ Advanced |

---

## Recipe 1: Simple GitHub Release (5 minutes)

**Use this when**: Your package has standard GitHub releases with `.tar.gz` archives for each platform.

### Example: Prometheus

#### Step 1: Find the release page

Visit: `https://github.com/prometheus/prometheus/releases/latest`

#### Step 2: Look at one asset name

Example: `prometheus-2.45.0.linux-amd64.tar.gz`

Break it down:
- `prometheus` = package name
- `2.45.0` = version
- `linux` = OS
- `amd64` = architecture
- `.tar.gz` = archive format

#### Step 3: Check the archive contents

Download one archive and peek inside:

```bash
curl -L https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz | tar -tzf - | head -5
```

Output:
```
prometheus-2.45.0.linux-amd64/
prometheus-2.45.0.linux-amd64/prometheus
prometheus-2.45.0.linux-amd64/promtool
```

**Key info**: The binaries are in a subfolder `prometheus-2.45.0.linux-amd64/`

#### Step 4: Find the checksum file

Look for files like:
- `sha256sums.txt`
- `checksums.txt`
- `SHA256SUMS`

Download it to see the format:
```bash
curl -L https://github.com/prometheus/prometheus/releases/download/v2.45.0/sha256sums.txt | head -2
```

Output:
```
abc123def456...  prometheus-2.45.0.linux-amd64.tar.gz
def456abc123...  prometheus-2.45.0.darwin-amd64.tar.gz
```

**Format**: `hash filename` = "multi-hash" format

#### Step 5: Write the .repo file

Create `ora-registry/prometheus.repo`:

```toml
name = "prometheus"
description = "Monitoring system and time series database"
homepage = "https://prometheus.io"

[source]
type = "github-releases"
repo = "prometheus/prometheus"

[source.download]
# Pattern: {version} is WITHOUT the 'v' prefix (2.45.0, not v2.45.0)
url = "https://github.com/prometheus/prometheus/releases/download/v{version}/prometheus-{version}.{os}-{arch}.tar.gz"

[platform.os_map]
linux = "linux"
macos = "darwin"

[platform.arch_map]
x86_64 = "amd64"
aarch64 = "arm64"

[install]
mode = "userland"
# Use glob pattern because binaries are in a versioned subfolder
binaries = ["prometheus-*/prometheus", "prometheus-*/promtool"]

[security]
allow_insecure = false

[security.checksum]
algorithm = "sha256"
url = "https://github.com/prometheus/prometheus/releases/download/v{version}/sha256sums.txt"
format = "multi-hash"

[metadata]
license = "Apache-2.0"
tags = ["monitoring", "metrics"]
```

#### Step 6: Test it

```bash
ora install --repo ora-registry/prometheus.repo
```

**Done!** Your binaries should now be in `~/.local/bin/`

---

## Recipe 2: GitHub Release with Platform-Specific URLs (10 minutes)

**Use this when**: Each platform has a completely different URL pattern (common with Rust projects).

### Example: ripgrep

#### Step 1: Check release assets

Visit: `https://github.com/BurntSushi/ripgrep/releases/latest`

Assets look like:
```
ripgrep-14.1.0-x86_64-unknown-linux-musl.tar.gz
ripgrep-14.1.0-aarch64-unknown-linux-gnu.tar.gz
ripgrep-14.1.0-x86_64-apple-darwin.tar.gz
ripgrep-14.1.0-aarch64-apple-darwin.tar.gz
```

**Notice**: Each platform uses a different "triple" (e.g., `x86_64-unknown-linux-musl`)

#### Step 2: Map each platform explicitly

Create `ora-registry/ripgrep.repo`:

```toml
name = "ripgrep"
description = "Fast line-oriented search tool"
homepage = "https://github.com/BurntSushi/ripgrep"

[source]
type = "github-releases"
repo = "BurntSushi/ripgrep"

# Use platform-specific URLs instead of a template
[source.download.urls]
linux_x86_64 = "https://github.com/BurntSushi/ripgrep/releases/download/{version}/ripgrep-{version}-x86_64-unknown-linux-musl.tar.gz"
linux_aarch64 = "https://github.com/BurntSushi/ripgrep/releases/download/{version}/ripgrep-{version}-aarch64-unknown-linux-gnu.tar.gz"
macos_x86_64 = "https://github.com/BurntSushi/ripgrep/releases/download/{version}/ripgrep-{version}-x86_64-apple-darwin.tar.gz"
macos_aarch64 = "https://github.com/BurntSushi/ripgrep/releases/download/{version}/ripgrep-{version}-aarch64-apple-darwin.tar.gz"

[install]
mode = "userland"
# The binary is usually in a subfolder like ripgrep-14.1.0-x86_64.../
binaries = ["ripgrep-*/rg"]

[security]
allow_insecure = false

[security.checksum]
algorithm = "sha256"
# Each archive has its own .sha256 file
url = "https://github.com/BurntSushi/ripgrep/releases/download/{version}/ripgrep-{version}-{platform_triple}.tar.gz.sha256"
format = "single-hash"

# Need to define platform triples for checksum URLs
[security.checksum.platform_map]
linux_x86_64 = "x86_64-unknown-linux-musl"
linux_aarch64 = "aarch64-unknown-linux-gnu"
macos_x86_64 = "x86_64-apple-darwin"
macos_aarch64 = "aarch64-apple-darwin"
```

**Pro tip**: When URLs differ significantly per platform, use `[source.download.urls]` instead of a single template.

---

## Recipe 3: Direct Binary Download (5 minutes)

**Use this when**: The package is a single binary, no archive to extract.

### Example: jq

#### Step 1: Check what's available

Visit: `https://github.com/jqlang/jq/releases/latest`

Assets:
```
jq-linux-amd64
jq-linux-arm64
jq-macos-amd64
jq-macos-arm64
```

These are direct binaries, not archives.

#### Step 2: Write the .repo file

Create `ora-registry/jq.repo`:

```toml
name = "jq"
description = "Command-line JSON processor"
homepage = "https://jqlang.github.io/jq/"

[source]
type = "github-releases"
repo = "jqlang/jq"

[source.download.urls]
linux_x86_64 = "https://github.com/jqlang/jq/releases/download/jq-{version}/jq-linux-amd64"
linux_aarch64 = "https://github.com/jqlang/jq/releases/download/jq-{version}/jq-linux-arm64"
macos_x86_64 = "https://github.com/jqlang/jq/releases/download/jq-{version}/jq-macos-amd64"
macos_aarch64 = "https://github.com/jqlang/jq/releases/download/jq-{version}/jq-macos-arm64"

[install]
mode = "userland"
# For direct binaries, specify the downloaded filename
binaries = ["jq-*"]
direct_binary = true

# Optional: rename to remove platform suffix
[install.rename]
"jq-linux-amd64" = "jq"
"jq-linux-arm64" = "jq"
"jq-macos-amd64" = "jq"
"jq-macos-arm64" = "jq"

# If no checksums are available
[security]
allow_insecure = true  # ⚠️ Only if no checksums exist
```

**Security note**: If the project provides checksums, always use them! Set `allow_insecure = true` only as a last resort.

---

## Testing Your .repo File

### Test locally before publishing

```bash
# 1. Validate the syntax
ora validate ora-registry/mypackage.repo

# 2. Try installing it
ora install --repo ora-registry/mypackage.repo

# 3. Verify the binary works
mypackage --version

# 4. Clean up for re-testing
ora uninstall mypackage
```

### Test in a registry

```bash
# 1. Create a test registry
mkdir -p test-registry/ora-registry
cp ora-registry/mypackage.repo test-registry/ora-registry/

cd test-registry
git init
git add .
git commit -m "Add mypackage"

# 2. Add your local registry
ora registry add test file://$PWD

# 3. Sync and install
ora registry sync test
ora install mypackage

# 4. Verify
mypackage --version
```

---

## Common Issues and Quick Fixes

### ❌ Issue: "Download URL returned 404"

**Cause**: URL template doesn't match actual release assets

**Fix**:
1. Go to the GitHub releases page
2. Right-click on an asset → "Copy link"
3. Compare with your URL template
4. Adjust `{os}`, `{arch}` mappings or use platform-specific URLs

**Example**:
```toml
# Your template generates:
url = ".../{version}/app-{version}-{os}-{arch}.tar.gz"
# → app-2.0.0-linux-amd64.tar.gz

# But actual asset is:
# → app-2.0.0-linux-x86_64.tar.gz

# Fix: Update arch_map
[platform.arch_map]
x86_64 = "x86_64"  # Instead of "amd64"
```

---

### ❌ Issue: "Binary not found after installation"

**Cause**: Binary is in a nested folder in the archive

**Fix**: Check the archive structure, then use glob patterns

```bash
# Check what's in the archive
curl -L <archive-url> | tar -tzf -

# If you see:
#   package-1.0.0/
#   package-1.0.0/bin/
#   package-1.0.0/bin/mybinary

# Use this pattern:
[install]
binaries = ["package-*/bin/mybinary"]
# or more general:
binaries = ["**/mybinary"]
```

---

### ❌ Issue: "Checksum verification failed"

**Cause**: Checksum file format doesn't match your configuration

**Fix**: Download the checksum file and identify the format

```bash
curl -L <checksum-url>
```

**Format examples**:

1. **multi-hash** (one file, multiple hashes):
```
abc123...  package-1.0.0-linux-amd64.tar.gz
def456...  package-1.0.0-macos-arm64.tar.gz
```
→ Use `format = "multi-hash"`

2. **single-hash** (one hash per file):
```
abc123def456...
```
→ Use `format = "single-hash"`

3. **sha256sum** (standard format with two spaces):
```
abc123...  package-1.0.0.tar.gz
```
→ Use `format = "sha256sum"`

---

### ❌ Issue: "Version not found"

**Cause**: GitHub tag format doesn't match expectations

**Fix**: Check actual tags

```bash
# List all tags
curl -s https://api.github.com/repos/owner/repo/releases | jq '.[].tag_name'

# Common formats:
# - "v1.0.0"  → Ora strips 'v' automatically
# - "1.0.0"   → Use as-is
# - "release-1.0.0" → Need custom version extraction
```

For non-standard tags, you may need `custom-api` type (see advanced docs).

---

## Next Steps

1. **Read the full guide**: [CREATING_REPO_FILES.md](CREATING_REPO_FILES.md)
2. **Schema reference**: [REPO_SCHEMA.md](REPO_SCHEMA.md)
3. **Share your .repo files**: Create a registry and contribute to the ecosystem!

---

## Quick Reference Card

```toml
# Minimal .repo file for GitHub releases
name = "mypackage"

[source]
type = "github-releases"
repo = "owner/repository"

[source.download]
url = "https://github.com/owner/repo/releases/download/v{version}/{name}-{version}-{os}-{arch}.tar.gz"

[install]
binaries = ["mybinary"]

[security.checksum]
algorithm = "sha256"
url = "https://github.com/owner/repo/releases/download/v{version}/checksums.txt"
format = "multi-hash"
```

**Variables**:
- `{version}` = version without 'v' prefix (e.g., `2.45.0`)
- `{os}` = `linux` or `macos` (customize with `platform.os_map`)
- `{arch}` = `x86_64` or `aarch64` (customize with `platform.arch_map`)

**Platform mappings** (most common):
```toml
[platform.os_map]
linux = "linux"     # or "Linux", depends on upstream
macos = "darwin"    # or "macos", "Darwin"

[platform.arch_map]
x86_64 = "amd64"    # or "x86_64", "x64"
aarch64 = "arm64"   # or "aarch64", "arm64"
```

Happy packaging! 🎉
