#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo_test() {
    echo -e "${YELLOW}[TEST] $1${NC}"
}

echo_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

echo_error() {
    echo -e "${RED}✗ $1${NC}"
}

echo_test "Starting simple reservoir tests"

# Check binary
echo_test "Testing binary exists"
if [ -f "./target/release/reservoir" ]; then
    echo_success "Binary found"
else
    echo_error "Binary not found"
    exit 1
fi

# Test help
echo_test "Testing help command"
if ./target/release/reservoir --help > /dev/null 2>&1; then
    echo_success "Help command works"
else
    echo_error "Help command failed"
fi

# Test version
echo_test "Testing version command"
if ./target/release/reservoir --version > /dev/null 2>&1; then
    echo_success "Version command works"
else
    echo_error "Version command failed"
fi

# Test export (this tries to connect to Neo4j)
echo_test "Testing export command with timeout"
if timeout 10s ./target/release/reservoir export > /tmp/test_export.json 2>/dev/null; then
    echo_success "Export command completed"
    if [ -f "/tmp/test_export.json" ]; then
        echo_success "Export file created"
        if python3 -m json.tool /tmp/test_export.json > /dev/null 2>&1; then
            echo_success "Export file contains valid JSON"
        else
            echo_error "Export file is not valid JSON"
        fi
    fi
else
    echo_error "Export command failed or timed out (expected if Neo4j not running)"
fi

# Test invalid command
echo_test "Testing invalid command handling"
if ! ./target/release/reservoir invalid-command > /dev/null 2>&1; then
    echo_success "Invalid command properly rejected"
else
    echo_error "Invalid command should be rejected"
fi

echo_test "Simple tests completed"

# Cleanup
rm -f /tmp/test_export.json
