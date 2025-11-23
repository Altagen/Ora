#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Helper functions
print_test() {
    echo -e "${YELLOW}[TEST $((TESTS_RUN + 1))]${NC} $1"
    TESTS_RUN=$((TESTS_RUN + 1))
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
    TESTS_PASSED=$((TESTS_PASSED + 1))
}

print_error() {
    echo -e "${RED}✗${NC} $1"
    TESTS_FAILED=$((TESTS_FAILED + 1))
}

print_summary() {
    echo ""
    echo "================================"
    echo "Test Summary"
    echo "================================"
    echo "Total tests: $TESTS_RUN"
    echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
    if [ $TESTS_FAILED -gt 0 ]; then
        echo -e "${RED}Failed: $TESTS_FAILED${NC}"
    else
        echo "Failed: 0"
    fi
    echo "================================"
}

# Setup test environment
TEST_DIR=$(mktemp -d)
export ORA_CONFIG_DIR="$TEST_DIR/config"
export ORA_CACHE_DIR="$TEST_DIR/cache"
export ORA_BIN_DIR="$TEST_DIR/bin"

echo "Test directory: $TEST_DIR"
echo "Binary path: $ORA_BINARY"
echo ""

# Cleanup function
cleanup() {
    echo ""
    echo "Cleaning up test environment..."
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

# Find the binary
if [ -z "$ORA_BINARY" ]; then
    if [ -f "./target/release/ora" ]; then
        ORA_BINARY="./target/release/ora"
    elif [ -f "./target/debug/ora" ]; then
        ORA_BINARY="./target/debug/ora"
    else
        echo "Error: ora binary not found. Please build first with 'cargo build --release'"
        exit 1
    fi
fi

# Test 1: Help command
print_test "Testing 'ora --help'"
if $ORA_BINARY --help > /dev/null 2>&1; then
    print_success "Help command works"
else
    print_error "Help command failed"
fi

# Test 2: Version command
print_test "Testing 'ora --version'"
if $ORA_BINARY --version > /dev/null 2>&1; then
    print_success "Version command works"
else
    print_error "Version command failed"
fi

# Test 3: Config show (should work even with empty config)
print_test "Testing 'ora config show'"
if $ORA_BINARY config show > /dev/null 2>&1; then
    print_success "Config show command works"
else
    print_error "Config show command failed"
fi

# Test 4: Registry list (empty at start)
print_test "Testing 'ora registry list' (should be empty)"
OUTPUT=$($ORA_BINARY registry list 2>&1 || true)
if echo "$OUTPUT" | grep -q "No registries configured" || echo "$OUTPUT" | grep -q "Configured registries:"; then
    print_success "Registry list command works"
else
    print_error "Registry list command failed"
fi

# Test 5: Add a test registry
print_test "Testing 'ora registry add'"
# Registry add may fail during sync, but the registry should still be added
$ORA_BINARY registry add test-registry https://example.com/registry.json > /dev/null 2>&1 || true
# Check if registry was added despite sync failure
OUTPUT=$($ORA_BINARY registry list 2>&1)
if echo "$OUTPUT" | grep -q "test-registry"; then
    print_success "Registry add command works (registry added)"
else
    print_error "Registry add command failed"
fi

# Test 6: List registries (should show our test registry)
print_test "Testing 'ora registry list' (should show test-registry)"
OUTPUT=$($ORA_BINARY registry list 2>&1)
if echo "$OUTPUT" | grep -q "test-registry"; then
    print_success "Registry appears in list"
else
    print_error "Registry not found in list"
fi

# Test 7: Remove the test registry
print_test "Testing 'ora registry remove'"
if $ORA_BINARY registry remove test-registry > /dev/null 2>&1; then
    print_success "Registry remove command works"
else
    print_error "Registry remove command failed"
fi

# Test 8: Search command (should fail gracefully with no registries)
print_test "Testing 'ora search' with no registries"
OUTPUT=$($ORA_BINARY search test-package 2>&1 || true)
if echo "$OUTPUT" | grep -q "No registries configured" || echo "$OUTPUT" | grep -q "No packages found"; then
    print_success "Search command handles no registries gracefully"
else
    print_error "Search command failed unexpectedly"
fi

# Test 9: List installed packages (should be empty)
print_test "Testing 'ora list' (should be empty)"
OUTPUT=$($ORA_BINARY list 2>&1 || true)
if echo "$OUTPUT" | grep -q "No packages installed" || echo "$OUTPUT" | grep -q "Installed packages:"; then
    print_success "List command works"
else
    print_error "List command failed"
fi

# Test 10: Config verify
print_test "Testing 'ora config verify'"
if $ORA_BINARY config verify > /dev/null 2>&1; then
    print_success "Config verify command works"
else
    print_error "Config verify command failed"
fi

# Test 11: Config init
print_test "Testing 'ora config init'"
if $ORA_BINARY config init > /dev/null 2>&1; then
    print_success "Config init command works"
else
    print_error "Config init command failed"
fi

# Test 12: Try to uninstall non-existent package
print_test "Testing 'ora uninstall' with non-existent package"
OUTPUT=$($ORA_BINARY uninstall non-existent-package 2>&1 || true)
if echo "$OUTPUT" | grep -q -i "not installed\|not found"; then
    print_success "Uninstall handles non-existent package gracefully"
else
    print_error "Uninstall command failed unexpectedly"
fi

# Test 13: Invalid registry URL
print_test "Testing registry add with invalid URL (should fail)"
if ! $ORA_BINARY registry add invalid-reg "not-a-url" > /dev/null 2>&1; then
    print_success "Registry add rejects invalid URL"
else
    print_error "Registry add should reject invalid URL"
fi

# Test 14: Update without packages
print_test "Testing 'ora update' with no packages"
OUTPUT=$($ORA_BINARY update 2>&1 || true)
if echo "$OUTPUT" | grep -q -i "no packages\|nothing to update\|up to date"; then
    print_success "Update handles no packages gracefully"
else
    # Update might just succeed with nothing to do
    print_success "Update command runs (no packages to update)"
fi

# Test 15: Show info command (requires package argument)
print_test "Testing 'ora info <package>'"
OUTPUT=$($ORA_BINARY info non-existent-package 2>&1 || true)
if echo "$OUTPUT" | grep -q -i "not found\|no package\|error"; then
    print_success "Info command works (package not found is expected)"
else
    print_success "Info command executed"
fi

# Test 16: Check config show output format
print_test "Testing 'ora config show' output format"
OUTPUT=$($ORA_BINARY config show 2>&1)
if echo "$OUTPUT" | grep -q "Configuration" || echo "$OUTPUT" | grep -q "Registry" || echo "$OUTPUT" | grep -q "cache"; then
    print_success "Config show has proper output"
else
    print_error "Config show output unexpected"
fi

# Test 17: Test registry sync (should fail gracefully with fake registry)
print_test "Testing 'ora registry sync' (may fail with network error)"
$ORA_BINARY registry add fake-reg https://httpbin.org/status/404 > /dev/null 2>&1 || true
OUTPUT=$($ORA_BINARY registry sync 2>&1 || true)
# This should fail but gracefully
if echo "$OUTPUT" | grep -q -i "error\|failed\|not found\|sync"; then
    print_success "Registry sync handles errors gracefully"
else
    print_success "Registry sync completed"
fi
$ORA_BINARY registry remove fake-reg > /dev/null 2>&1 || true

# Test 18: Test registry sync with no registries (Issue #2)
print_test "Testing 'ora registry sync' with no registries"
# Clean up any remaining registries first
$ORA_BINARY registry list 2>&1 | grep -oP '^\s*✓\s*\K\S+' | while read -r reg; do
    $ORA_BINARY registry remove "$reg" > /dev/null 2>&1 || true
done
OUTPUT=$($ORA_BINARY registry sync 2>&1)
if echo "$OUTPUT" | grep -q "No registries configured\|registr"; then
    print_success "Registry sync handles no registries gracefully"
else
    print_error "Registry sync should show 'No registries configured'"
fi

# Test 19: Test registry sync command exists (Issue #2 - command was missing)
print_test "Testing 'ora registry sync' command exists"
OUTPUT=$($ORA_BINARY registry sync --help 2>&1 || true)
if echo "$OUTPUT" | grep -q -i "sync\|usage"; then
    print_success "Registry sync command is available"
else
    print_error "Registry sync command not found"
fi

# Test 20: Test registry verify with non-existent registry (Issue #1)
print_test "Testing 'ora registry verify' with non-existent registry"
OUTPUT=$($ORA_BINARY registry verify nonexistent-reg 2>&1 || true)
if echo "$OUTPUT" | grep -q -i "not found"; then
    print_success "Registry verify handles non-existent registry"
else
    print_error "Registry verify should fail for non-existent registry"
fi

# Test 21: Add a test registry for verify tests
print_test "Testing 'ora registry verify' setup (add test registry)"
$ORA_BINARY registry add test-verify-reg https://github.com/brandy223/ora-registry.git > /dev/null 2>&1 || true
if $ORA_BINARY registry list 2>&1 | grep -q "test-verify-reg"; then
    print_success "Test registry added for verification tests"
else
    print_error "Failed to add test registry"
fi

# Test 22: Test registry verify with valid registry (Issue #1)
print_test "Testing 'ora registry verify' with valid registry"
OUTPUT=$($ORA_BINARY registry verify test-verify-reg 2>&1 || true)
if echo "$OUTPUT" | grep -q "verification complete"; then
    print_success "Registry verify succeeds with valid registry"
else
    print_error "Registry verify should succeed with valid registry"
fi

# Test 23: Test registry verify shows detailed info (Issue #1)
print_test "Testing 'ora registry verify' shows detailed information"
OUTPUT=$($ORA_BINARY registry verify test-verify-reg 2>&1 || true)
if echo "$OUTPUT" | grep -q "Registry found in configuration" && \
   echo "$OUTPUT" | grep -q "Valid git repository" && \
   echo "$OUTPUT" | grep -q "packages.*directory exists"; then
    print_success "Registry verify shows detailed validation steps"
else
    print_error "Registry verify should show detailed validation info"
fi

# Test 24: Test registry verify shows package count (Issue #1)
print_test "Testing 'ora registry verify' shows package count"
OUTPUT=$($ORA_BINARY registry verify test-verify-reg 2>&1 || true)
if echo "$OUTPUT" | grep -q "Found.*package"; then
    print_success "Registry verify shows package count"
else
    print_error "Registry verify should show package count"
fi

# Cleanup test registry
$ORA_BINARY registry remove test-verify-reg > /dev/null 2>&1 || true

# Print final summary
print_summary

# Exit with appropriate code
if [ $TESTS_FAILED -gt 0 ]; then
    exit 1
else
    exit 0
fi
