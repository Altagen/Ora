# Recently Completed Features

This document tracks recently completed features and improvements.

---

## Update Flow for --repo Packages

**Status**: ✅ Completed

Fixed `ora update` to work correctly with packages installed from local .repo files. The update command now:
- Parses the `registry_source` field from installed.toml
- Distinguishes between `file:` and `registry:` sources
- Reloads .repo configuration from the appropriate source
- Respects the `allow_insecure` flag from the repo configuration
- Passes all necessary parameters to the install command

**Impact**: Users can now update packages installed via `--repo` flag without manually uninstalling and reinstalling.

**Files modified**: `src/cli/commands/update.rs`

---

## Automatic Directory Cleanup

**Status**: ✅ Completed

Uninstall now automatically removes empty parent directories after removing a package. This prevents accumulation of empty version directories in the package storage.

**Features**:
- Checks if parent directory is empty after uninstall
- Automatically removes empty directories
- Non-critical operation (failures don't abort uninstall)
- Logged for transparency

**Impact**: Cleaner package directory structure without manual cleanup.

**Files modified**: `src/cli/commands/uninstall.rs`

---

## Webpage Scraping Provider

**Status**: ✅ Completed

Full support for scraping download URLs from HTML pages with regex-based version extraction. This enables Ora to work with packages that don't have traditional package registries (GitHub releases, GitLab, etc.).

**Features**:
- HTML page scraping with configurable URL patterns
- Regex-based version extraction
- Semantic version sorting
- Cache support (15-minute TTL)
- Platform/architecture detection in URLs
- Security configuration (allow_redirects, certificate validation)

**Use cases**:
- Applications hosted on custom websites (Windsurf, etc.)
- Projects without GitHub/GitLab releases
- Proprietary software with web-based downloads

**Files modified**:
- `src/providers/webpage_scraping.rs`
- `src/config/repo.rs`
- `docs/CREATING_REPO_FILES.md`

---

## Windsurf Integration

**Status**: ✅ Completed

Complete support for Electron-based applications with comprehensive post-install permission fixes. Windsurf (and similar Electron apps) require multiple executable permissions beyond the main binary.

**Features**:
- Correct binary pattern detection (`Windsurf/bin/windsurf`)
- Post-install script for setting permissions on:
  - Main binary (`windsurf`)
  - Crash handler (`chrome_crashpad_handler`)
  - Chrome sandbox (`chrome-sandbox`)
  - All shared libraries (`*.so`)
  - All executables in `bin/` directory
- Release channel filtering (stable vs next/beta)

**Impact**: Full support for complex Electron applications that require multiple executables and libraries.

**Files modified**:
- `windsurf.repo`
- `docs/CREATING_REPO_FILES.md`

---

## Notes

All features listed here have been:
- Implemented and tested
- Documented
- Integrated into the main codebase
- Verified to work in production scenarios
