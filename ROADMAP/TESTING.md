# Testing Improvements Roadmap

This document tracks testing improvements and missing test coverage for future Ora releases.

---

## Current Status (0.2.2)

**Total Tests**: 150 ✅
- Unit tests: 49
- Integration tests: 101
- All passing

**Test Coverage**:
- ✅ Config migration (0.0 → 0.1)
- ✅ Package schema migration (0.0 → 0.1)
- ✅ Downgrade detection (basic)
- ✅ Version comparison
- ✅ Auto-migration on database load
- ✅ Security configuration
- ✅ Installation, uninstall, update flows

---

## 🔴 Critical: Missing Test Coverage (0.2.3)

### 1. Invalid/Corrupted Version Strings

**Priority**: Critical
**Risk**: Data corruption, crashes

```rust
// Test cases needed:
schema_version = "abc"       // Non-numeric
schema_version = "1.x"       // Partial numeric
schema_version = "1.2.3.4"   // Too many parts
schema_version = ""          // Empty string
schema_version = "999.999"   // Unreasonably high
```

**Expected Behavior**:
- Invalid versions should be treated as "0.0" (oldest)
- Clear error messages or auto-migration
- No crashes or panics

**Test File**: `src/config/migrations.rs`

---

### 2. Mixed Database (Multiple Schema Versions)

**Priority**: Critical
**Risk**: Partial migration failures

```rust
// Scenario: Database with packages at different schema versions
[packages.package-a]
schema_version = "0.0"  # Old package

[packages.package-b]
schema_version = "0.1"  # Current package

[packages.package-c]
schema_version = "0.0"  # Another old package
```

**Test Coverage Needed**:
- ✅ Verify all old packages are migrated
- ✅ Verify current packages are unchanged
- ✅ Verify migration count is accurate
- ✅ Verify database saved correctly after partial migration

**Test File**: `tests/integration_update.rs` or `src/config/migrations.rs`

---

### 3. Downgrade Detection Integration Test

**Priority**: Critical
**Risk**: User confusion, data loss

```rust
// Test: User tries to use old Ora binary with new config
// Scenario: config_version = "0.2", CURRENT_CONFIG_VERSION = "0.1"
```

**Test Coverage Needed**:
- ✅ Verify command fails gracefully
- ✅ Verify error message suggests "ora self-update"
- ✅ Verify error message is user-friendly
- ✅ Verify no data corruption occurs

**Test File**: `tests/integration_update.rs`

---

### 4. Error Message Quality Tests

**Priority**: High
**Risk**: Poor user experience

```rust
// Verify error messages contain:
// - "ora self-update" suggestion
// - Clear explanation of the problem
// - No technical jargon
// - Actionable next steps
```

**Test File**: `src/config/migrations.rs`

---

## 🟡 High: Update Command Coverage (0.2.3)

### 5. Preserve allow_insecure During Update

**Priority**: High
**Risk**: Security flag silently changed

**Current Code** (`src/cli/commands/update.rs:117`):
```rust
let allow_insecure = installed.allow_insecure || repo_config.security.allow_insecure;
```

**Test Cases Needed**:
- ✅ Package installed with `--allow-insecure` → update → still has flag
- ✅ Package installed without flag → update → flag not added
- ✅ Repo config changes allow_insecure → verify precedence

**Test File**: `tests/integration_update.rs`

---

### 6. Update From Different Sources

**Priority**: Medium
**Risk**: Update failures

**Test Cases**:
- ✅ Update package installed from `file:` source
- ✅ Update package installed from `registry:` source
- ✅ Update package with missing .repo file (graceful failure)
- ✅ Update package with registry no longer available

**Test File**: `tests/integration_update.rs`

---

## 🟢 Medium: Edge Cases (0.2.4+)

### 7. Concurrent Access

**Priority**: Medium
**Risk**: Data corruption, race conditions

**Scenarios**:
- Two `ora install` commands running simultaneously
- One `ora install` + one `ora uninstall` at the same time
- Config modification during command execution

**Mitigation**:
- File locking mechanisms
- Atomic writes
- Transaction-like behavior

---

### 8. Migration Failure Recovery

**Priority**: Medium
**Risk**: Corrupted config files

**Scenarios**:
- Disk full during migration save
- Permission denied during migration save
- Process killed mid-migration

**Test Coverage Needed**:
- ✅ Original file preserved on failure
- ✅ Clear error messages
- ✅ No partial/corrupted writes

---

### 9. End-to-End Migration Scenarios

**Priority**: Low
**Risk**: Integration bugs

**Full User Journey Tests**:
- Fresh install → migrate config → install package → update → uninstall
- Legacy config (no version) → auto-migrate → all operations work
- Downgrade attempt → clear error → upgrade Ora → success

---

## 🔵 Low: Performance & Stress Tests (0.3.0+)

### 10. Large Database Performance

**Scenarios**:
- Database with 1000+ packages
- Migration performance with many packages
- Database load/save performance

---

### 11. Version Comparison Edge Cases

**Test Cases**:
- Very large version numbers (overflow protection)
- Negative version numbers
- Special characters in version strings
- Unicode in version strings

---

## Implementation Priority

**For 0.2.3 (Next Release)**:
1. Invalid version strings test
2. Mixed database test
3. Downgrade integration test
4. Error message quality test

**For 0.2.4**:
- Preserve allow_insecure test
- Update from different sources tests

**For 0.3.0+**:
- Concurrent access tests
- Migration failure recovery
- Performance tests

---

## Test Metrics Goals

**Current**: 150 tests
**0.2.3 Target**: 160+ tests (critical coverage)
**0.2.4 Target**: 170+ tests (update coverage)
**0.3.0 Target**: 200+ tests (full coverage)

**Coverage Goals**:
- Line coverage: 80%+
- Branch coverage: 70%+
- Integration coverage: All critical user paths

---

## Related Documentation

- [ROADMAP.md](../ROADMAP.md) - Overall project roadmap
- [SECURITY.md](SECURITY.md) - Security testing requirements
- [FEATURES.md](FEATURES.md) - Feature testing requirements

---

## Contributing

**Want to help with testing?**

1. Pick any test from the 🔴 Critical section
2. Open an issue: "Test: [Test Name]"
3. Implement the test
4. Submit a PR

**Test Guidelines**:
- Clear test names describing what is being tested
- Good error messages when tests fail
- Independent tests (no shared state)
- Fast execution (< 100ms per test ideal)

---

## Notes

- Test priorities may change based on bug reports
- Security-related tests take priority
- All tests must pass before release
- Integration tests may be marked `#[ignore]` if they require network/external resources
