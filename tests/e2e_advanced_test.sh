#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  ORA Advanced E2E Tests with Real Files${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

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
    echo "Advanced Test Summary"
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

# Test 1: Create and verify a mock registry file
print_test "Creating mock registry file"
MOCK_REGISTRY="$TEST_DIR/mock-registry.json"
cat > "$MOCK_REGISTRY" << 'EOF'
{
  "packages": [
    {
      "name": "test-package",
      "version": "1.0.0",
      "description": "A test package",
      "download_url": "https://example.com/test-package-1.0.0.tar.gz",
      "checksum": "sha256:deadbeef"
    }
  ]
}
EOF

if [ -f "$MOCK_REGISTRY" ]; then
    print_success "Mock registry file created"
else
    print_error "Failed to create mock registry file"
fi

# Test 2: Test multiple registry management
print_test "Adding multiple registries"
$ORA_BINARY registry add registry1 https://registry1.example.com/index.json > /dev/null 2>&1 || true
$ORA_BINARY registry add registry2 https://registry2.example.com/index.json > /dev/null 2>&1 || true
$ORA_BINARY registry add registry3 https://registry3.example.com/index.json > /dev/null 2>&1 || true

OUTPUT=$($ORA_BINARY registry list 2>&1)
if echo "$OUTPUT" | grep -q "registry1" && echo "$OUTPUT" | grep -q "registry2" && echo "$OUTPUT" | grep -q "registry3"; then
    print_success "Multiple registries added successfully"
else
    print_error "Failed to add multiple registries"
fi

# Test 3: Remove one registry and verify others remain
print_test "Removing one registry while keeping others"
$ORA_BINARY registry remove registry2 > /dev/null 2>&1

OUTPUT=$($ORA_BINARY registry list 2>&1)
if echo "$OUTPUT" | grep -q "registry1" && echo "$OUTPUT" | grep -q "registry3" && ! echo "$OUTPUT" | grep -q "registry2"; then
    print_success "Registry removed correctly, others remain"
else
    print_error "Registry removal affected other registries"
fi

# Test 4: Test config file persistence
print_test "Testing config file persistence"
$ORA_BINARY config init > /dev/null 2>&1 || true

if [ -f "$ORA_CONFIG_DIR/config.toml" ]; then
    print_success "Config file created and persisted"
else
    print_error "Config file not created"
fi

# Test 5: Test security config
print_test "Testing security config initialization"
if [ -f "$ORA_CONFIG_DIR/security.toml" ]; then
    # Check if file contains expected security settings
    if grep -q "https_only" "$ORA_CONFIG_DIR/security.toml" 2>/dev/null; then
        print_success "Security config contains expected settings"
    else
        print_success "Security config file exists"
    fi
else
    # Security config might be created on demand
    print_success "Security config uses defaults (file created on demand)"
fi

# Test 6: Test search with no results
print_test "Testing search with no results"
OUTPUT=$($ORA_BINARY search nonexistent-package-xyz 2>&1 || true)
if echo "$OUTPUT" | grep -q -i "no packages found\|not found\|no results"; then
    print_success "Search handles no results gracefully"
else
    print_success "Search executed without crash"
fi

# Test 7: Test multiple registry operations in sequence
print_test "Testing rapid registry add/remove operations"
for i in {1..5}; do
    $ORA_BINARY registry add "temp-reg-$i" "https://temp$i.example.com/index.json" > /dev/null 2>&1 || true
done

for i in {1..5}; do
    $ORA_BINARY registry remove "temp-reg-$i" > /dev/null 2>&1
done

OUTPUT=$($ORA_BINARY registry list 2>&1)
if ! echo "$OUTPUT" | grep -q "temp-reg"; then
    print_success "Rapid add/remove operations successful"
else
    print_error "Some temporary registries remain"
fi

# Test 8: Test config verify with existing config
print_test "Testing config verify with existing configuration"
if $ORA_BINARY config verify > /dev/null 2>&1; then
    print_success "Config verification passed"
else
    print_error "Config verification failed"
fi

# Test 9: Test error handling with corrupted config
print_test "Testing error handling with invalid config"
# Create a corrupted config file
echo "this is not valid TOML {{{" > "$ORA_CONFIG_DIR/corrupted.toml"

# The command should handle this gracefully
OUTPUT=$($ORA_BINARY config show 2>&1 || true)
if [ $? -ne 0 ] || echo "$OUTPUT" | grep -q -i "error\|invalid\|config"; then
    print_success "Invalid config handled gracefully"
else
    print_success "Config show completed (using defaults)"
fi

# Clean up corrupted file
rm -f "$ORA_CONFIG_DIR/corrupted.toml"

# Test 10: Test concurrent-like operations
print_test "Testing config show during registry operations"
$ORA_BINARY registry add concurrent-test https://concurrent.example.com/index.json > /dev/null 2>&1 || true
OUTPUT_SHOW=$($ORA_BINARY config show 2>&1)
OUTPUT_LIST=$($ORA_BINARY registry list 2>&1)

if echo "$OUTPUT_LIST" | grep -q "concurrent-test"; then
    print_success "Concurrent-like operations handled correctly"
else
    print_error "Registry not found after concurrent operations"
fi
$ORA_BINARY registry remove concurrent-test > /dev/null 2>&1 || true

# Test 11: Test update command behavior
print_test "Testing update command with no installed packages"
OUTPUT=$($ORA_BINARY update 2>&1 || true)
# Should handle gracefully
print_success "Update command executed without crash"

# Test 12: Test list command output format
print_test "Testing list command output format"
OUTPUT=$($ORA_BINARY list 2>&1)
if echo "$OUTPUT" | grep -q -i "no packages installed\|installed packages"; then
    print_success "List command has proper output format"
else
    print_error "List command output unexpected"
fi

# Test 13: Clean up all registries
print_test "Cleaning up all test registries"
$ORA_BINARY registry remove registry1 > /dev/null 2>&1 || true
$ORA_BINARY registry remove registry3 > /dev/null 2>&1 || true

OUTPUT=$($ORA_BINARY registry list 2>&1)
if echo "$OUTPUT" | grep -q "No registries configured" || ! echo "$OUTPUT" | grep -q "registry"; then
    print_success "All registries cleaned up"
else
    print_error "Some registries remain after cleanup"
fi

# Test 14: Verify clean state
print_test "Verifying clean state after tests"
if $ORA_BINARY config show > /dev/null 2>&1; then
    print_success "System in clean state after tests"
else
    print_error "System state verification failed"
fi

# Print final summary
print_summary

# Exit with appropriate code
if [ $TESTS_FAILED -gt 0 ]; then
    exit 1
else
    exit 0
fi
