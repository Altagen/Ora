# Ora Testing Framework

Comprehensive test suite for the Ora package manager.

## Test Structure

```
tests/
├── e2e_test.sh              # End-to-end shell script tests
├── integration/             # Rust integration tests
│   ├── helpers/
│   │   ├── mock_registry.rs  # Mock git registry
│   │   └── test_env.rs       # Isolated test environment
│   ├── test_install.rs       # Installation tests
│   ├── test_registry.rs      # Registry management tests
│   └── test_uninstall.rs     # Uninstallation tests
└── fixtures/
    └── repo_files/          # Test .repo files
        ├── windman.repo      # Your project
        ├── windsurf.repo     # Custom provider
        ├── prometheus.repo   # GitHub with checksums
        ├── ripgrep.repo      # GitHub standard
        ├── jq.repo           # Simple binary
        └── fd.repo           # Alternative find
```

## Running Tests

### Unit Tests (Fast)

```bash
# Run all unit tests
cargo test --lib

# Run specific module tests
cargo test --lib config
cargo test --lib security
```

### Integration Tests (Requires Network)

```bash
# Run all integration tests
cargo test --test '*'

# Run specific test file
cargo test --test test_registry

# Run with output
cargo test --test test_install -- --nocapture

# Run ignored tests (requires network, downloads real binaries)
cargo test --test test_install -- --ignored --nocapture
```

### End-to-End Tests (Full Workflow)

```bash
# Run ALL e2e tests (recommended)
./tests/run_all_e2e.sh

# Or run individual test suites:
./tests/e2e_test.sh              # Basic E2E tests (17 tests)
./tests/e2e_advanced_test.sh     # Advanced E2E tests (14 tests)

# Specify custom binary location
ORA_BINARY=/path/to/ora ./tests/run_all_e2e.sh
```

The complete test suite includes:
- **Basic tests** (17): Core functionality, all commands, error handling
- **Advanced tests** (14): Multi-registry, persistence, stress testing
- **Total**: 31 end-to-end test cases

All tests:
1. Create isolated test environments (no impact on your system)
2. Test with fake/mock data (no network required)
3. Verify error handling and edge cases
4. Clean up automatically

## Test Packages

### 1. Windman (Your Project)
- **Type**: GitHub releases
- **Source**: https://github.com/Altagen/windman
- **Platform**: Linux x86_64 only
- **Security**: SHA256 checksum
- **Size**: Small (~MB)
- **Purpose**: Test GitHub provider with checksums

### 2. Windsurf (Proprietary)
- **Type**: Direct URL
- **Source**: https://windsurf.com
- **Platform**: Linux x64, arm64
- **Security**: No checksums (allow_insecure)
- **Size**: Large (~200MB)
- **Purpose**: Test direct-url provider

### 3. Prometheus
- **Type**: GitHub releases
- **Source**: https://github.com/prometheus/prometheus
- **Platform**: Multi-platform
- **Security**: SHA256 checksums
- **Purpose**: Test multi-binary installation

### 4. Ripgrep
- **Type**: GitHub releases
- **Source**: https://github.com/BurntSushi/ripgrep
- **Platform**: Linux x86_64, aarch64
- **Security**: Single-hash SHA256
- **Purpose**: Test standard GitHub releases

### 5. jq
- **Type**: GitHub releases
- **Source**: https://github.com/jqlang/jq
- **Platform**: Linux amd64, arm64
- **Security**: SHA256 multi-hash file
- **Purpose**: Test single binary installation

### 6. fd
- **Type**: GitHub releases
- **Source**: https://github.com/sharkdp/fd
- **Platform**: Linux x86_64
- **Security**: Single-hash SHA256
- **Purpose**: Test additional binaries (autocomplete)

## Mock Registry

The `MockRegistry` helper creates a temporary git repository with test .repo files:

```rust
use tests::integration::helpers::MockRegistry;

let registry = MockRegistry::new()?;
let url = registry.url(); // file:///tmp/...
let packages = registry.list_packages()?; // ["fd", "jq", "prometheus", ...]
```

Features:
- ✅ Creates real git repository in `/tmp`
- ✅ Commits all .repo files
- ✅ Provides `file://` URL for cloning
- ✅ Supports adding new packages dynamically
- ✅ Auto-cleanup on drop

## Test Environment

The `TestEnvironment` creates an isolated Ora environment:

```rust
use tests::integration::helpers::TestEnvironment;

let env = TestEnvironment::new()?;
env.set_env_vars(); // Sets ORA_* env vars

// Test operations...

env.cleanup(); // Cleanup (automatic on drop)
```

Features:
- ✅ Isolated config directory
- ✅ Isolated data/cache directories
- ✅ Temporary install location
- ✅ Environment variable management
- ✅ Helper functions for assertions

## Writing New Tests

### Add a New .repo File

1. Create `.repo` file in `tests/fixtures/repo_files/`
2. Follow the schema from examples
3. The MockRegistry will automatically include it

### Add Integration Test

```rust
use crate::helpers::{MockRegistry, TestEnvironment};
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_my_feature() {
    let env = TestEnvironment::new().unwrap();
    env.set_env_vars();

    let mut cmd = Command::cargo_bin("ora").unwrap();
    cmd.arg("my-command");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("expected output"));

    env.cleanup();
}
```

### Add E2E Test

Add to `tests/e2e_test.sh`:

```bash
run_test "My test description" \
    "$ORA_BIN my-command | grep -q 'expected'"
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Test
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run unit tests
        run: cargo test --lib
      - name: Run integration tests
        run: cargo test --test '*'
      - name: Run e2e tests
        run: ./tests/e2e_test.sh
```

## Test Coverage

### Rust Integration Tests
- ✅ Registry management (add, list, remove, update)
- ✅ Package search
- ✅ Package info
- ✅ Installation (from .repo file and registry)
- ✅ Uninstallation
- ✅ Error handling
- ✅ Platform detection

### E2E Binary Tests (31 total)

**Basic Tests (17)**:
- ✅ Help and version commands
- ✅ Config management (show, verify, init)
- ✅ Registry operations (add, remove, list, sync)
- ✅ Package search with no registries
- ✅ Package list (empty state)
- ✅ Package uninstall (non-existent)
- ✅ Invalid URL rejection
- ✅ Update command (no packages)
- ✅ Info command
- ✅ Config output format validation
- ✅ Error handling for all commands

**Advanced Tests (14)**:
- ✅ Mock registry file creation
- ✅ Multiple registry management
- ✅ Selective registry removal
- ✅ Config file persistence
- ✅ Security config initialization
- ✅ Search with no results
- ✅ Rapid add/remove operations (stress test)
- ✅ Config verification
- ✅ Corrupted config handling
- ✅ Concurrent-like operations
- ✅ Update with no packages
- ✅ List output format
- ✅ Complete cleanup verification
- ✅ Clean state verification

### Not Yet Covered
- ⏳ GPG verification (not implemented yet)
- ⏳ TLS certificate pinning (not implemented yet)
- ⏳ Actual package installation (E2E tests use mocks)
- ⏳ Network-dependent registry sync

## Troubleshooting

### Tests Fail Due to Network

Some tests require network access. Run without ignored tests:

```bash
cargo test --test '*' -- --skip ignored
```

### Tests Fail Due to Missing Binaries

Ensure you've built ora first:

```bash
cargo build --release
```

### Mock Registry Issues

If git operations fail, check git configuration:

```bash
git config --global user.email "test@example.com"
git config --global user.name "Test User"
```

### Permission Issues

E2E tests create files in `~/Downloads/ora-test-*`. Ensure write permissions.

## Performance

Test execution times (approximate):
- Unit tests: ~5s
- Integration tests (no network): ~10s
- Integration tests (with network): ~1-5min (depends on downloads)
- E2E test suite: ~2-10min (includes real installations)

## Next Steps

- [ ] Add GPG verification tests (when implemented)
- [ ] Add TLS certificate pinning tests
- [ ] Add update command comprehensive tests
- [ ] Add concurrent installation tests
- [ ] Add failure recovery tests
- [ ] Add checksum mismatch tests
- [ ] Measure code coverage with tarpaulin
