# Webpage Scraping Provider

This provider scrapes download URLs and version information from HTML pages.

## How It Works

1. Fetches an HTML page (e.g., releases page)
2. Extracts download URLs using regex patterns
3. Extracts version numbers from URLs or page content
4. Sorts versions semantically to find the latest

## Use Cases

- Applications without GitHub/GitLab releases
- Proprietary software with web-based downloads
- Projects with custom release pages
- Electron apps (Windsurf, etc.)

## Example: Windsurf

```toml
[package]
name = "windsurf"
description = "Windsurf - AI-powered code editor"

[source.version]
discovery_type = "html-scraping"
discovery_url = "https://windsurf.com/editor/releases"
url_pattern = "https://windsurf-stable\\.codeiumdata\\.com/[^/]+/stable/[^\"\\s]+?\\.(tar\\.gz|zip)"
version_pattern = "([0-9]+\\.[0-9]+\\.[0-9]+)"

[install]
mode = "userland"
binaries = ["Windsurf/bin/windsurf"]

post_install = '''
#!/bin/bash
chmod +x "$INSTALL_DIR"/Windsurf/windsurf
chmod +x "$INSTALL_DIR"/Windsurf/chrome_crashpad_handler
chmod +x "$INSTALL_DIR"/Windsurf/*.so 2>/dev/null || true
'''
```

## Configuration Fields

### discovery_url
The URL of the page to scrape:
```toml
discovery_url = "https://example.com/downloads"
```

### url_pattern
Regex pattern to extract download URLs from the page:
```toml
# Match tar.gz or zip files from specific domain
url_pattern = "https://example\\.com/releases/[^\"]+?\\.(tar\\.gz|zip)"
```

**Tips:**
- Escape dots: `\.` instead of `.`
- Use non-greedy matching: `+?` instead of `+`
- Match file extensions: `\\.(tar\\.gz|zip)`

### version_pattern
Regex pattern to extract version numbers:
```toml
# Match semantic versions like 1.2.3
version_pattern = "([0-9]+\\.[0-9]+\\.[0-9]+)"
```

The pattern must have a capturing group `()` for the version.

## Release Channels

For projects with multiple release channels (stable/beta/next), use different URL patterns:

**Stable:**
```toml
url_pattern = "https://example\\.com/[^/]+/stable/[^\"]+?\\.tar\\.gz"
```

**Beta:**
```toml
url_pattern = "https://example\\.com/[^/]+/next/[^\"]+?\\.tar\\.gz"
```

## Platform Detection

The provider automatically detects platform and architecture from URLs:

```
https://windsurf.com/linux-x64/stable/Windsurf-1.12.35.tar.gz
                  ^      ^
                  |      |
              platform  arch
```

Supported patterns:
- `linux-x64`, `linux-amd64` → Linux x86_64
- `linux-arm64`, `linux-aarch64` → Linux ARM64
- `darwin-x64`, `macos-x64` → macOS x86_64
- `darwin-arm64`, `macos-arm64` → macOS ARM64

## Post-Install Scripts

Electron apps often need permission fixes:

```toml
post_install = '''
#!/bin/bash
set -e

# Main binary
chmod +x "$INSTALL_DIR"/AppName/bin/appname

# Electron helpers
chmod +x "$INSTALL_DIR"/AppName/chrome_crashpad_handler
chmod +x "$INSTALL_DIR"/AppName/chrome-sandbox

# Shared libraries
chmod +x "$INSTALL_DIR"/AppName/*.so 2>/dev/null || true
'''
```

## Caching

Scraped URLs are cached for 15 minutes to avoid repeated requests:
```
~/.cache/ora/scrapers/{hash}.json
```

## Examples in This Directory

- **windsurf.repo** - Windsurf stable channel
- **windsurf-next.repo** - Windsurf beta channel

## Usage

```bash
# Install from local .repo file
ora install windsurf --repo ./windsurf.repo --allow-insecure

# Or serve via HTTPS
ora registry add windsurf https://windsurf.com/ora/windsurf.repo
ora install windsurf
```

## Security Note

Webpage scraping cannot automatically verify checksums unless the page provides them in a structured format. Consider using `allow_insecure = true` with caution, and only for trusted sources.
