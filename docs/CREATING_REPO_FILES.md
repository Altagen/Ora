# Creating .repo Files - Complete Guide

This guide explains how to create `.repo` files for different types of software packages.

## Table of Contents

- [Basic Concepts](#basic-concepts)
- [GitHub Releases Example](#github-releases-example)
- [Step-by-Step: Prometheus](#step-by-step-prometheus)
- [Common Patterns](#common-patterns)
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
type = "github-releases"  # or "gitlab-releases", "custom-api", "direct-url"
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

## Where to Create .repo Files

When creating a registry, organize your `.repo` files like this:

```
my-registry/                # Git repository root
├── README.md              # Optional: describe your registry
└── packages/              # REQUIRED: .repo files go here
    ├── ripgrep.repo
    ├── fd.repo
    ├── prometheus.repo
    └── ...
```

**Important**: The `packages/` directory is required. Ora looks for `.repo` files specifically in this directory.

After creating your registry:

```bash
# 1. Create git repository
mkdir my-registry && cd my-registry
git init

# 2. Create packages directory
mkdir packages

# 3. Add .repo files in packages/
cat > packages/ripgrep.repo << 'EOF'
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
