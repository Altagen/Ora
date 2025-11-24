# GitHub Releases Provider

This provider fetches packages from GitHub releases.

## How It Works

1. Discovers available versions from GitHub releases API
2. Downloads binaries from release assets
3. Optionally verifies checksums from release assets

## Example: Prometheus

```toml
[package]
name = "prometheus"
description = "Prometheus monitoring and alerting toolkit"

[source]
type = "github-releases"
repo = "prometheus/prometheus"

[source.download]
url = "https://github.com/prometheus/prometheus/releases/download/v{version}/prometheus-{version}.{os}-{arch}.tar.gz"

[source.os_mapping]
linux = "linux"
macos = "darwin"

[source.arch_mapping]
x86_64 = "amd64"
aarch64 = "arm64"

[install]
binary_path = "prometheus-*/prometheus"

[security.checksum]
url = "https://github.com/prometheus/prometheus/releases/download/v{version}/sha256sums.txt"
algorithm = "sha256"
single_hash = false
```

## Template Variables

- `{version}` - The version number (e.g., "2.45.0")
- `{os}` - Operating system after mapping (e.g., "linux", "darwin")
- `{arch}` - Architecture after mapping (e.g., "amd64", "arm64")

## Platform Mappings

Map Rust's platform names to the ones used by the project:

```toml
[source.os_mapping]
linux = "linux"
macos = "darwin"
windows = "windows"

[source.arch_mapping]
x86_64 = "amd64"
aarch64 = "arm64"
```

## Security: Checksums

Many GitHub projects provide checksum files (SHA256SUMS, checksums.txt, etc.):

```toml
[security.checksum]
url = "https://github.com/{owner}/{repo}/releases/download/v{version}/sha256sums.txt"
algorithm = "sha256"
single_hash = false  # File contains multiple checksums
```

If the checksum file contains only one hash:
```toml
single_hash = true
```

## Examples in This Directory

- **prometheus.repo** - Multi-binary package with checksums
- **ripgrep.repo** - Simple single binary with checksum file

## Usage

```bash
# Install from local .repo file
ora install prometheus --repo ./prometheus.repo

# Or add to a registry
cp prometheus.repo ~/my-registry/ora-registry/
```
