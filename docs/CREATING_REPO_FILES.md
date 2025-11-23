# Creating .repo Files - Complete Guide

This guide explains how to create `.repo` files for different types of software packages.

> **New to creating .repo files?** Start with the **[Quick Start Guide](QUICK_START_REPO.md)** - create your first .repo file in 5 minutes! ⚡

## Table of Contents

- [Basic Concepts](#basic-concepts)
- [GitHub Releases Example](#github-releases-example)
- [Step-by-Step: Prometheus](#step-by-step-prometheus)
- [Webpage Scraping Provider](#webpage-scraping-provider)
- [Step-by-Step: Windsurf IDE](#step-by-step-windsurf-ide)
- [Common Patterns](#common-patterns)
- [Debugging Your .repo File](#debugging-your-repo-file)
- [Troubleshooting](#troubleshooting)

---

## Basic Concepts

A `.repo` file describes:

- **Where** to find the software (GitHub, GitLab, custom API)
- **How** to download it (URL templates)
- **What** to install (which binaries)
- **How** to verify it (checksums, signatures)

### Minimum Required Fields

```toml
name = "package-name"

[source]
type = "github-releases"  # or "gitlab-releases", "custom-api", "direct-url", "webpage-scraping"
repo = "owner/repository"

[source.download]
url = "https://example.com/download/{version}/{os}-{arch}.tar.gz"

[install]
binaries = ["binary-name"]
```

---

## GitHub Releases Example

### Understanding GitHub Releases

GitHub releases are found at: `https://github.com/OWNER/REPO/releases`

For example, Prometheus releases are at: `https://github.com/prometheus/prometheus/releases`

Each release has:

- **Tag name**: The version tag on GitHub (e.g., `v2.45.0` on GitHub, becomes `2.45.0` in Ora)
- **Assets**: Downloadable files (binaries, tarballs, checksums)

### How Ora Fetches Versions

Ora uses the GitHub API to get all releases:

```txt
GET https://api.github.com/repos/prometheus/prometheus/releases
```

This returns a JSON array of releases with their tag names, which become available versions.

### How Ora Constructs Download URLs

Ora uses the `url` template with variables:

- `{version}`: The version without 'v' prefix (e.g., `2.45.0`)
- `{os}`: Mapped OS name (e.g., `linux`, `darwin`)
- `{arch}`: Mapped architecture (e.g., `amd64`, `arm64`)

**Note**: GitHub tags often use `v2.45.0` format. Ora automatically strips the 'v' prefix, so `{version}` becomes `2.45.0`.

---

## Step-by-Step: Prometheus

Let's create a `.repo` file for Prometheus from scratch.

### Step 1: Find the GitHub Repository

Repository: `https://github.com/prometheus/prometheus`

### Step 2: Examine a Release

Visit: `https://github.com/prometheus/prometheus/releases/latest`

Example assets for version `2.45.0` (GitHub tag is `v2.45.0`):

```txt
prometheus-2.45.0.linux-amd64.tar.gz
prometheus-2.45.0.linux-arm64.tar.gz
prometheus-2.45.0.darwin-amd64.tar.gz
prometheus-2.45.0.darwin-arm64.tar.gz
sha256sums.txt
```

### Step 3: Identify the URL Pattern

Looking at the assets, we see a pattern:

```txt
https://github.com/prometheus/prometheus/releases/download/v{version}/prometheus-{version}.{os}-{arch}.tar.gz
```

Variables:

- `{version}`: `2.45.0` (without 'v' prefix in filename)
- `{os}`: `linux` or `darwin`
- `{arch}`: `amd64` or `arm64`

### Step 4: Map Platform Names

Ora uses standard names, but GitHub releases might use different ones:

```toml
[platform.os_map]
linux = "linux"      # Ora's 'linux' → GitHub's 'linux'
macos = "darwin"     # Ora's 'macos' → GitHub's 'darwin'

[platform.arch_map]
x86_64 = "amd64"     # Ora's 'x86_64' → GitHub's 'amd64'
aarch64 = "arm64"    # Ora's 'aarch64' → GitHub's 'arm64'
```

### Step 5: Identify Binaries

Download and extract the archive to see what's inside:

```bash
tar -tzf prometheus-2.45.0.linux-amd64.tar.gz
```

Output shows:

```txt
prometheus-2.45.0.linux-amd64/
prometheus-2.45.0.linux-amd64/prometheus
prometheus-2.45.0.linux-amd64/promtool
prometheus-2.45.0.linux-amd64/LICENSE
...
```

Binaries to install: `prometheus` and `promtool`

### Step 6: Identify Checksums

The `sha256sums.txt` file contains hashes for all assets:

```txt
https://github.com/prometheus/prometheus/releases/download/v{version}/sha256sums.txt
```

Format example:

```txt
a1b2c3d4... prometheus-2.45.0.linux-amd64.tar.gz
e5f6g7h8... prometheus-2.45.0.darwin-arm64.tar.gz
```

### Step 7: Write the Complete .repo File

```toml
# Prometheus monitoring system
name = "prometheus"
description = "Prometheus monitoring and alerting toolkit"
homepage = "https://prometheus.io"

[source]
type = "github-releases"
repo = "prometheus/prometheus"

[source.download]
url = "https://github.com/prometheus/prometheus/releases/download/v{version}/prometheus-{version}.{os}-{arch}.tar.gz"

[platform]
[platform.os_map]
linux = "linux"
macos = "darwin"

[platform.arch_map]
x86_64 = "amd64"
aarch64 = "arm64"

[install]
mode = "userland"
binaries = ["prometheus-*/prometheus", "prometheus-*/promtool"]

[security]
allow_insecure = false

[security.checksum]
url = "https://github.com/prometheus/prometheus/releases/download/v{version}/sha256sums.txt"
algorithm = "sha256"
format = "multi-hash"

[metadata]
license = "Apache-2.0"
authors = ["Prometheus Authors"]
tags = ["monitoring", "metrics", "alerting"]
```

### Step 8: Test the Installation

```bash
# Install Prometheus
ora install --repo prometheus.repo

# Verify it works
prometheus --version
```

Ora will:

1. Fetch all releases from GitHub API
2. Show available versions
3. Download the latest version (or specified version)
4. Verify SHA256 checksum
5. Extract binaries to `~/.local/bin/`
6. Make them executable

---

## Webpage Scraping Provider

### When to Use Webpage Scraping

The `webpage-scraping` provider is designed for software that:

- **Doesn't use GitHub/GitLab releases**: Instead, downloads are listed on a custom webpage
- **Has dynamic URLs**: Download URLs include commit hashes or other dynamic identifiers
- **Lacks a public API**: No JSON/REST API to query for versions
- **Serves static HTML with download links**: Links are embedded in the HTML (not JavaScript-rendered)

**Example use cases**:
- VS Code derivatives (like Windsurf)
- Projects with custom release pages
- Software distributed via CDNs without API access

### How Webpage Scraping Works

The provider:

1. **Fetches HTML**: Downloads the release page HTML content
2. **Extracts URLs**: Uses regex to find all download URLs
3. **Filters archives**: Keeps only `.zip`, `.tar.gz`, `.tar.xz`, `.tar.bz2`, `.tgz` files
4. **Extracts versions**: Uses regex to extract version numbers from URLs
5. **Detects platforms**: Identifies platform (e.g., `linux-x64`, `darwin-arm64`) from URL
6. **Caches results**: Stores scraped data with TTL to avoid re-scraping

### Caching Behavior

To avoid re-scraping on every operation, the provider caches scraped URLs:

- **Cache location**: `~/.cache/ora/scrapers/{hash}.json`
- **Cache key**: MD5 hash of the discovery URL
- **TTL (Time-To-Live)**: Configurable via global config (default: 1 hour)
- **Cache invalidation**: Automatic after TTL expires

Configure TTL in `~/.config/ora/config.toml`:

```toml
[scraper]
ttl = 3600  # 1 hour in seconds
```

---

## Step-by-Step: Windsurf IDE

Let's create a `.repo` file for Windsurf IDE, which uses a custom releases page.

### Step 1: Examine the Release Page

Visit: `https://windsurf.com/editor/releases`

This page lists downloads for all versions, but:
- URLs are embedded in HTML/JSON
- URLs contain commit hashes that change per version
- No API endpoint available

### Step 2: Analyze the URL Structure

Example URLs from the page:

```txt
https://windsurf-stable.codeiumdata.com/linux-x64/stable/fb9435d9a.../Windsurf-linux-x64-1.12.35.tar.gz
https://windsurf-stable.codeiumdata.com/darwin-arm64/stable/fb9435d9a.../Windsurf-darwin-arm64-1.12.35.zip
https://windsurf-stable.codeiumdata.com/win32-x64-archive/stable/fb9435d9a.../Windsurf-win32-x64-1.12.35.zip
```

Observations:
- Base domain: `windsurf-stable.codeiumdata.com`
- Platform in URL: `linux-x64`, `darwin-arm64`, `win32-x64-archive`
- Commit hash required: `fb9435d9a4d89c681a097318dfac4546738ce05c`
- Version in filename: `1.12.35`
- Archive formats: `.tar.gz` for Linux, `.zip` for macOS/Windows

### Step 3: Create Regex Patterns

We need two regex patterns:

**1. URL Pattern** - Extracts full download URLs:

```regex
https://windsurf-stable\.codeiumdata\.com/[^/]+/stable/[^"\s]+?\.(zip|tar\.gz|tar\.xz|tar\.bz2|tgz)
```

This matches:
- The base URL
- Platform identifier (e.g., `linux-x64`)
- **`/stable/` path** - ensures only stable releases are matched (not `/next/`)
- Any characters (non-greedy) until...
- One of the archive extensions

> **Note**: Windsurf has two release channels:
> - **stable** (`/stable/`): Production-ready releases
> - **next** (`/next/`): Beta/preview releases with `+next.` in filename
>
> By including `/stable/` in the pattern, we only match stable releases.

**2. Version Pattern** - Extracts version from URL:

```regex
([0-9]+\.[0-9]+\.[0-9]+)
```

This captures version numbers like `1.12.35`.

### Step 4: Map Platform Identifiers

Windsurf uses platform names in URLs (`linux-x64`) that differ from system names (`linux_amd64`):

```toml
[platform.url_filters]
linux_amd64 = "linux-x64"
darwin_aarch64 = "darwin-arm64"
darwin_amd64 = "darwin-x64"
windows_amd64 = "win32-x64-archive"
windows_aarch64 = "win32-arm64-archive"
```

Format: `{system_os}_{normalized_arch} = "url_substring"`

The provider searches for URLs containing the filter string (e.g., `"linux-x64"`).

### Step 5: Identify Binaries

Download and extract an archive to see the structure:

```bash
tar -tzf Windsurf-linux-x64-1.12.35.tar.gz | head -20
```

Output shows:

```txt
Windsurf/
Windsurf/windsurf
Windsurf/bin/windsurf
Windsurf/resources/
...
```

Binaries are in: `Windsurf/windsurf` and `Windsurf/bin/windsurf`

### Step 6: Write the Complete .repo File

```toml
name = "windsurf"
description = "Windsurf Editor - AI-powered IDE"
homepage = "https://windsurf.com"

[source]
type = "webpage-scraping"

[source.version]
discovery_type = "html-scraping"
discovery_url = "https://windsurf.com/editor/releases"

# Regex to extract full download URLs from the HTML page
# Only match URLs in the /stable/ path (not /next/)
url_pattern = "https://windsurf-stable\\.codeiumdata\\.com/[^/]+/stable/[^\"\\s]+?\\.(zip|tar\\.gz|tar\\.xz|tar\\.bz2|tgz)"

# Regex to extract version number from URLs (capture group 1)
version_pattern = "([0-9]+\\.[0-9]+\\.[0-9]+)"

[platform]
# Map system platform identifiers to URL platform identifiers
# Format: system_os_normalized_arch = "url_platform_identifier"
[platform.url_filters]
linux_amd64 = "linux-x64"
darwin_aarch64 = "darwin-arm64"
darwin_amd64 = "darwin-x64"
windows_amd64 = "win32-x64-archive"
windows_aarch64 = "win32-arm64-archive"

[install]
mode = "userland"

# Binaries to install from the archive
binaries = ["Windsurf/bin/windsurf"]

# Post-install script to fix permissions on all Electron binaries and libraries
post_install = '''
#!/bin/bash
set -e

# Fix permissions on the Windsurf directory
# Windsurf needs executable permissions on:
# - Main binary (windsurf)
# - Crash handler (chrome_crashpad_handler)
# - Chrome sandbox (chrome-sandbox)
# - All .so libraries (libffmpeg.so, libEGL.so, libGLESv2.so, etc.)

# Make all binaries and libraries executable
chmod +x "$INSTALL_DIR"/Windsurf/windsurf
chmod +x "$INSTALL_DIR"/Windsurf/chrome_crashpad_handler
chmod +x "$INSTALL_DIR"/Windsurf/chrome-sandbox
chmod +x "$INSTALL_DIR"/Windsurf/*.so 2>/dev/null || true
chmod +x "$INSTALL_DIR"/Windsurf/bin/* 2>/dev/null || true

echo "✓ Windsurf installed successfully"
echo "  - Run 'windsurf' to launch"
'''

[security]
# Windsurf doesn't provide checksums, so we must allow insecure installation
allow_insecure = true

[metadata]
license = "Proprietary"
authors = ["Codeium"]
tags = ["ide", "editor", "ai"]
```

### Step 7: Test the Installation

```bash
# Install Windsurf using the .repo file
ora install windsurf --repo ./windsurf.repo --allow-insecure

# Verify it works
windsurf --version

# Launch Windsurf
windsurf
```

Ora will:

1. Scrape the releases page for download URLs
2. Cache the results for 1 hour (configurable)
3. Show available versions
4. Download the archive for your platform
5. Extract binaries to `~/.local/bin/`
6. Make them executable

### Webpage Scraping Configuration Reference

#### Required Fields

```toml
[source]
type = "webpage-scraping"  # REQUIRED

[source.version]
discovery_url = "https://..."  # REQUIRED: URL of releases page
discovery_type = "html-scraping"  # REQUIRED: Always "html-scraping"
url_pattern = "regex"  # REQUIRED: Regex to extract download URLs
version_pattern = "regex"  # REQUIRED: Regex to extract versions (with capture group)

[platform.url_filters]
{os}_{arch} = "substring"  # REQUIRED: Map platforms to URL substrings
```

#### Regex Pattern Tips

**URL Pattern**:
- Must match complete URLs including file extension
- Use non-greedy matching (`+?`) to avoid matching too much
- End with explicit file extensions: `\.(zip|tar\.gz|...)`
- Escape dots in domains: `\.` not `.`

**Version Pattern**:
- Must include a capture group `(...)` for the version
- Common pattern: `([0-9]+\.[0-9]+\.[0-9]+)` for semver
- The captured group becomes the version identifier

**Example patterns**:

```toml
# Match URLs ending with archive extensions
url_pattern = "https://cdn\\.example\\.com/[^\"\\s]+?\\.(zip|tar\\.gz)"

# Match semantic versions (1.2.3)
version_pattern = "([0-9]+\\.[0-9]+\\.[0-9]+)"

# Match versions with optional patch (1.2 or 1.2.3)
version_pattern = "([0-9]+\\.[0-9]+(?:\\.[0-9]+)?)"
```

#### URL Filters

URL filters map system platforms to URL substrings:

```toml
[platform.url_filters]
# Format: {normalized_os}_{normalized_arch} = "url_substring"

# Linux
linux_amd64 = "linux-x64"      # Matches URLs containing "linux-x64"
linux_arm64 = "linux-arm64"

# macOS
darwin_amd64 = "darwin-x64"    # Also matches "mac-x64", "osx-x64"
darwin_arm64 = "darwin-arm64"  # Also matches "mac-arm64", "apple-silicon"

# Windows
windows_amd64 = "win32-x64"
windows_arm64 = "win32-arm64"
```

**How it works**:
1. Ora detects system platform (e.g., `linux` + `amd64`)
2. Looks up the filter: `linux_amd64 = "linux-x64"`
3. Searches scraped URLs for those containing `"linux-x64"`
4. Returns the first matching URL for that version

#### Caching Configuration

Global config (`~/.config/ora/config.toml`):

```toml
[scraper]
# TTL in seconds (default: 3600 = 1 hour)
ttl = 3600

# Set to 0 to disable caching (not recommended)
# ttl = 0

# Increase for stable releases (e.g., 24 hours)
# ttl = 86400
```

Cache files are stored in: `~/.cache/ora/scrapers/{hash}.json`

To force re-scraping:

```bash
# Delete cache file
rm ~/.cache/ora/scrapers/*.json

# Or reduce TTL temporarily
ora install --repo windsurf.repo
```

### Webpage Scraping Limitations

**What it can handle**:
- ✅ Static HTML with embedded links
- ✅ Pages served with curl-friendly user agents
- ✅ URLs in JSON embedded in HTML
- ✅ Multiple versions on one page

**What it cannot handle**:
- ❌ JavaScript-rendered content (requires browser)
- ❌ Login-protected downloads
- ❌ CAPTCHA-protected pages
- ❌ Dynamic content loaded after page load

**If the page uses JavaScript**:
The webpage-scraping provider uses a curl-like user agent (`curl/8.0.0`) which causes many modern sites to serve static HTML instead of JavaScript-heavy SPA versions. This usually works well for release pages.

If it doesn't work:
1. Check if there's an API endpoint (use DevTools Network tab)
2. Consider using `custom-api` provider instead
3. Contact the project to request an API or static release page

---

## Where to Create .repo Files

When creating a registry, organize your `.repo` files like this:

```
my-registry/                # Git repository root
├── README.md              # Optional: describe your registry
└── ora-registry/          # REQUIRED: .repo files go here
    ├── ripgrep.repo
    ├── fd.repo
    ├── prometheus.repo
    └── ...
```

**Important**: The `ora-registry/` directory is required. Ora looks for `.repo` files specifically in this directory.

After creating your registry:

```bash
# 1. Create git repository
mkdir my-registry && cd my-registry
git init

# 2. Create ora-registry directory
mkdir ora-registry

# 3. Add .repo files in ora-registry/
cat > ora-registry/ripgrep.repo << 'EOF'
name = "ripgrep"
...
EOF

# 4. Commit and push
git add .
git commit -m "Add ripgrep package"
git push origin main
```

---

## Common Patterns

### Pattern 1: Version in Tag vs Filename

Some projects use `v` prefix in Git tags but not in filenames:

**GitHub Tag**: `v2.45.0`
**Filename**: `prometheus-2.45.0.linux-amd64.tar.gz`

Solution: In the URL template, `{version}` provides the version without 'v':

```toml
url = "https://.../download/v{version}/app-{version}.{os}-{arch}.tar.gz"
```

The URL becomes: `https://.../download/v2.45.0/app-2.45.0.linux-amd64.tar.gz`

### Pattern 2: Different Archive Names

**Problem**: Archives extract to version-specific folders:

```txt
prometheus-2.45.0.linux-amd64/
  └── prometheus
```

**Solution**: Use glob patterns in binaries:

```toml
binaries = ["prometheus-*/prometheus"]
```

### Pattern 3: No OS in Filename

Some projects don't include OS in filename:

```txt
ripgrep-14.0.0-x86_64-unknown-linux-musl.tar.gz
```

**Solution**: Use more specific mapping or custom URL per platform:

```toml
[source.download.urls]
linux_x86_64 = "https://.../ripgrep-{version}-x86_64-unknown-linux-musl.tar.gz"
linux_aarch64 = "https://.../ripgrep-{version}-aarch64-unknown-linux-gnu.tar.gz"
macos_x86_64 = "https://.../ripgrep-{version}-x86_64-apple-darwin.tar.gz"
```

### Pattern 4: Single Binary (No Archive)

For direct binary downloads:

```toml
[source.download]
url = "https://github.com/owner/repo/releases/download/v{version}/binary-{os}-{arch}"

[install]
binaries = ["binary-{os}-{arch}"]
direct_binary = true
```

---

## Debugging Your .repo File

When creating a `.repo` file, follow these debugging steps to catch issues early:

### Step 1: Validate Syntax

```bash
# Check if the TOML is valid
ora validate ora-registry/mypackage.repo
```

This checks:
- ✓ Valid TOML syntax
- ✓ Required fields present
- ✓ URL templates are well-formed
- ⚠️ Warns about `allow_insecure = true`

### Step 2: Check What Versions Are Available

```bash
# For GitHub releases, manually check what Ora sees
curl -s https://api.github.com/repos/owner/repo/releases | jq '.[].tag_name' | head -10
```

Common issues:
- Tags use `v` prefix (e.g., `v1.0.0`) → Ora strips this automatically
- Releases are marked as "pre-release" → May not be detected
- No releases exist → Check if it's using a different versioning system

### Step 3: Test Download URL Construction

Use `RUST_LOG=debug` to see exactly what URLs Ora generates:

```bash
RUST_LOG=debug ora install --repo ora-registry/mypackage.repo
```

Look for lines like:
```
[DEBUG] Constructed download URL: https://github.com/...
[DEBUG] Platform detected: linux_x86_64
[DEBUG] After mapping: os=linux, arch=amd64
```

### Step 4: Manually Test a Download URL

Copy the URL from debug output and test it:

```bash
# Test if the URL works
curl -I "https://github.com/owner/repo/releases/download/v1.0.0/package-1.0.0-linux-amd64.tar.gz"

# Should return: HTTP/2 302 (redirect) or 200 (OK)
# If 404: Your URL template or mappings are wrong
```

### Step 5: Check Archive Contents

Download an archive and inspect its structure:

```bash
# Download
curl -L -o test.tar.gz "https://..."

# List contents
tar -tzf test.tar.gz | head -20

# Look for:
# - Are binaries in a subfolder? Use glob patterns like "package-*/binary"
# - Are there multiple binaries? List them all in [install.binaries]
# - Are there spaces or special characters? May need escaping
```

### Step 6: Check Checksum File Format

Download the checksum file and verify its format:

```bash
curl -L "https://github.com/owner/repo/releases/download/v1.0.0/checksums.txt"
```

**Format identification**:

1. **multi-hash** - Multiple lines with `hash filename`:
```
abc123...  package-linux-amd64.tar.gz
def456...  package-macos-arm64.tar.gz
```

2. **single-hash** - One hash only (one file per checksum):
```
abc123def456789...
```

3. **sha256sum** - Standard format with two spaces:
```
abc123...  package.tar.gz
```

Update your `.repo` accordingly:
```toml
[security.checksum]
format = "multi-hash"  # or "single-hash" or "sha256sum"
```

### Step 7: Test Installation End-to-End

```bash
# Try installing
ora install --repo ora-registry/mypackage.repo

# Check if binary was installed
ls -lah ~/.local/bin/mybinary

# Test if it runs
mybinary --version

# Check logs for warnings
RUST_LOG=warn ora install --repo ora-registry/mypackage.repo
```

### Common Debug Patterns

**Pattern 1: URL returns 404**
```bash
# Your URL template
url = "https://github.com/{repo}/releases/download/v{version}/{name}-{version}.{os}-{arch}.tar.gz"

# Test with actual values
# Replace variables manually:
# {repo} = "owner/repo"
# {version} = "1.0.0"
# {os} = "linux"
# {arch} = "amd64"

# Result: https://github.com/owner/repo/releases/download/v1.0.0/name-1.0.0.linux-amd64.tar.gz

# Visit GitHub releases page and compare with actual asset names
# Adjust template or add platform mappings
```

**Pattern 2: Binary not found after install**
```bash
# Check extraction worked
ls -R ~/.cache/ora/downloads/

# If binary is there but not in ~/.local/bin:
# → Wrong glob pattern in [install.binaries]

# Fix by using a more general pattern:
binaries = ["**/mybinary"]  # Finds mybinary anywhere in archive
```

**Pattern 3: Checksum mismatch**
```bash
# Download the archive and compute hash manually
curl -L -o test.tar.gz "https://..."
sha256sum test.tar.gz

# Compare with checksum file
curl -L "https://.../checksums.txt"

# If they match → format issue in .repo
# If they don't match → upstream problem or wrong file
```

---

## Troubleshooting

### Issue: Version Not Found

**Symptom**: "Version X.Y.Z not found"

**Cause**: Tag format mismatch

**Solution**: Check actual GitHub tags:

```bash
curl -s https://api.github.com/repos/owner/repo/releases | jq '.[].tag_name'
```

If tags use `v` prefix, ensure your `.repo` handles it correctly.

### Issue: Download URL 404

**Symptom**: "Failed to download package"

**Cause**: Incorrect URL template or platform mapping

**Solution**:

1. Visit GitHub releases page manually
2. Click on an asset to get its real URL
3. Compare with your template
4. Adjust `os_map` and `arch_map` if needed

Example debugging:

```bash
# What Ora generates:
https://github.com/user/repo/releases/download/v1.0.0/app-1.0.0.linux-amd64.tar.gz

# What actually exists:
https://github.com/user/repo/releases/download/v1.0.0/app-1.0.0-linux-x86_64.tar.gz
```

Fix: Update URL template and arch_map.

### Issue: Checksum Verification Failed

**Symptom**: "Checksum mismatch"

**Possible causes**:

1. Wrong checksum file format
2. Filename doesn't match checksum file entry
3. Corrupted download

**Solution**: Download the checksum file manually and check format:

```bash
curl -L https://github.com/owner/repo/releases/download/1.0.0/checksums.txt
```

Formats:

- **Single-hash**: One hash per file (one line per asset)
- **Multi-hash**: Hash followed by filename
- **sha256sum**: Standard format `HASH  filename`

Update `format` field accordingly:

```toml
[security.checksum]
format = "multi-hash"  # or "single-hash" or "sha256sum"
```

### Issue: Binary Not Found After Install

**Symptom**: Binary installed but not in PATH

**Cause**: Binaries extracted to nested directory

**Solution**: Use glob patterns:

```toml
binaries = ["**/actual-binary-name"]
```

Or be more specific:

```toml
binaries = ["package-v*/bin/binary"]
```

---

## Advanced Examples

### Example: jq (Direct Binary Download)

```toml
name = "jq"

[source]
type = "github-releases"
repo = "jqlang/jq"

[source.download.urls]
linux_x86_64 = "https://github.com/jqlang/jq/releases/download/jq-{version}/jq-linux-amd64"
linux_aarch64 = "https://github.com/jqlang/jq/releases/download/jq-{version}/jq-linux-arm64"
macos_x86_64 = "https://github.com/jqlang/jq/releases/download/jq-{version}/jq-macos-amd64"
macos_aarch64 = "https://github.com/jqlang/jq/releases/download/jq-{version}/jq-macos-arm64"

[install]
binaries = ["jq-*"]
direct_binary = true
rename = { "jq-*" = "jq" }
```

### Example: fd (With Deb/RPM Packages)

For projects that provide `.deb` or `.rpm` packages, extract binaries:

```toml
name = "fd"

[source]
type = "github-releases"
repo = "sharkdp/fd"

[source.download]
url = "https://github.com/sharkdp/fd/releases/download/v{version}/fd-v{version}-{arch}-unknown-linux-musl.tar.gz"

[platform.arch_map]
x86_64 = "x86_64"
aarch64 = "aarch64"

[install]
binaries = ["fd-v*/fd"]
```

---

## Best Practices

1. **Always verify checksums**: Set `allow_insecure = false`
2. **Test on all platforms**: Use the 4 supported combinations (linux/macos × x86_64/aarch64)
3. **Use glob patterns**: Archives often extract to version-specific folders
4. **Include metadata**: License, authors, tags help users discover packages
5. **Document homepage**: Users should know where to get help
6. **Keep it simple**: Start with minimal config, add features as needed

---

## Getting Help

- **Schema Reference**: See [REPO_SCHEMA.md](REPO_SCHEMA.md)
- **Examples**: Check `tests/fixtures/repo_files/` for real-world examples
- **GitHub Issues**: Report problems at the Ora repository

---

**Next Steps**:

- Try creating a `.repo` file for your favorite GitHub project
- Test it with `ora install --repo yourfile.repo`
- Share it with the community!
