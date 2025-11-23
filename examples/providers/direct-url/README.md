# Direct URL Provider

This provider uses hard-coded download URLs for each platform/architecture combination.

## When to Use

- Simple tools with predictable download URLs
- Internal company tools with direct download links
- Quick prototyping and testing
- No version discovery needed (manual updates)

## How It Works

1. You provide direct URLs for each platform
2. Ora selects the appropriate URL based on current platform
3. Downloads and installs the binary

**Note:** This provider does NOT auto-discover new versions. You must manually update URLs when new versions are released.

## Example: Simple Tool

```toml
[package]
name = "simple-tool"
description = "Example of a simple tool with direct download URLs"

[source]
type = "direct-url"

[source.download.urls]
linux_x86_64 = "https://example.com/downloads/simple-tool-linux-amd64.tar.gz"
linux_aarch64 = "https://example.com/downloads/simple-tool-linux-arm64.tar.gz"
macos_x86_64 = "https://example.com/downloads/simple-tool-darwin-amd64.tar.gz"
macos_aarch64 = "https://example.com/downloads/simple-tool-darwin-arm64.tar.gz"

[install]
binary_path = "simple-tool"
```

## Platform Keys

Available platform/architecture combinations:

| Key | Platform | Architecture |
|-----|----------|--------------|
| `linux_x86_64` | Linux | x86_64 (64-bit Intel/AMD) |
| `linux_aarch64` | Linux | ARM64 |
| `macos_x86_64` | macOS | x86_64 (Intel Macs) |
| `macos_aarch64` | macOS | ARM64 (Apple Silicon M1/M2/M3) |
| `windows_x86_64` | Windows | x86_64 |
| `windows_aarch64` | Windows | ARM64 |

## No Version Discovery

Unlike other providers, direct-url does not discover versions automatically. The URL IS the version.

To update to a new version:
1. Update the URLs in the .repo file
2. Users run `ora update <package>`

## Security: Checksums

You can provide checksums for each URL:

```toml
[security]
allow_insecure = false

[[security.checksums]]
platform = "linux_x86_64"
sha256 = "abc123..."

[[security.checksums]]
platform = "macos_x86_64"
sha256 = "def456..."
```

## Use Case: Internal Tools

Perfect for company-internal tools:

```toml
[package]
name = "internal-cli"
description = "Internal company CLI tool"

[source]
type = "direct-url"

[source.download.urls]
linux_x86_64 = "https://artifacts.company.com/internal-cli/latest/linux-x64.tar.gz"
macos_x86_64 = "https://artifacts.company.com/internal-cli/latest/darwin-x64.tar.gz"

[security]
allow_insecure = true  # Internal network, checksums not needed
```

## Examples in This Directory

- **simple-tool.repo** - Basic example with platform-specific URLs

## Usage

```bash
# Install from local .repo file
ora install simple-tool --repo ./simple-tool.repo

# Or serve via HTTPS
ora registry add my-tool https://company.com/tools/my-tool.repo
```

## Limitations

- No automatic version discovery
- Manual URL updates required
- Best for stable tools with infrequent updates
- Consider using github-releases provider for better version management
