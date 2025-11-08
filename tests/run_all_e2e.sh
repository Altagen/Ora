#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}================================================${NC}"
echo -e "${CYAN}     ORA E2E Test Suite - Complete Run${NC}"
echo -e "${CYAN}================================================${NC}"
echo ""

# Find the binary
if [ -z "$ORA_BINARY" ]; then
    if [ -f "./target/release/ora" ]; then
        ORA_BINARY="./target/release/ora"
    elif [ -f "./target/debug/ora" ]; then
        ORA_BINARY="./target/debug/ora"
    else
        echo -e "${RED}Error: ora binary not found.${NC}"
        echo "Please build first with: cargo build --release"
        exit 1
    fi
fi

export ORA_BINARY

echo -e "${BLUE}Using binary:${NC} $ORA_BINARY"
echo ""

# Track overall results
TOTAL_SUITES=0
PASSED_SUITES=0
FAILED_SUITES=0

# Function to run a test suite
run_suite() {
    local suite_name=$1
    local suite_script=$2

    echo ""
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Running: $suite_name${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    TOTAL_SUITES=$((TOTAL_SUITES + 1))

    if bash "$suite_script"; then
        echo -e "${GREEN}✓ $suite_name PASSED${NC}"
        PASSED_SUITES=$((PASSED_SUITES + 1))
        return 0
    else
        echo -e "${RED}✗ $suite_name FAILED${NC}"
        FAILED_SUITES=$((FAILED_SUITES + 1))
        return 1
    fi
}

# Run all test suites
echo -e "${CYAN}Starting test execution...${NC}"
echo ""

run_suite "Basic E2E Tests" "tests/e2e_test.sh" || true
run_suite "Advanced E2E Tests" "tests/e2e_advanced_test.sh" || true

# Print final summary
echo ""
echo -e "${CYAN}================================================${NC}"
echo -e "${CYAN}           FINAL TEST SUMMARY${NC}"
echo -e "${CYAN}================================================${NC}"
echo ""
echo "Total test suites: $TOTAL_SUITES"
echo -e "${GREEN}Passed suites: $PASSED_SUITES${NC}"

if [ $FAILED_SUITES -gt 0 ]; then
    echo -e "${RED}Failed suites: $FAILED_SUITES${NC}"
    echo ""
    echo -e "${RED}❌ SOME TESTS FAILED${NC}"
    exit 1
else
    echo "Failed suites: 0"
    echo ""
    echo -e "${GREEN}✓ ALL TESTS PASSED!${NC}"
    echo ""
    echo -e "${CYAN}The Ora package manager is working correctly!${NC}"
    exit 0
fi
