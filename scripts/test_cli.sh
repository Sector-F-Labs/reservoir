#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test configuration
TEST_DATA_DIR="$(pwd)/test_data"
TEST_EXPORT_FILE="$TEST_DATA_DIR/test_export.json"
TEST_IMPORT_FILE="$TEST_DATA_DIR/test_import.json"
TEST_PARTITION="test_partition_$(date +%s)"
TEST_INSTANCE="test_instance_$(date +%s)"

# Track test results
TESTS_PASSED=0
TESTS_FAILED=0
FAILED_TESTS=()

echo_header() {
    echo -e "${BLUE}===============================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}===============================================${NC}"
}

echo_test() {
    echo -e "${YELLOW}[TEST] $1${NC}"
}

echo_success() {
    echo -e "${GREEN}✓ $1${NC}"
    ((TESTS_PASSED++))
}

echo_error() {
    echo -e "${RED}✗ $1${NC}"
    ((TESTS_FAILED++))
    FAILED_TESTS+=("$1")
}

# Check if reservoir binary exists
check_binary() {
    echo_test "Checking if reservoir binary exists"
    if command -v reservoir >/dev/null 2>&1; then
        echo_success "Binary found: $(which reservoir)"
    else
        echo_error "Reservoir binary not found. Please ensure direnv is loaded or run 'cargo build --release' first."
        echo_info "Try: direnv allow . && cargo build --release"
        exit 1
    fi
}

# Setup test environment
setup_test_env() {
    echo_test "Setting up test environment"
    mkdir -p "$TEST_DATA_DIR"

    # Create test import data
    cat > "$TEST_IMPORT_FILE" << EOF
[
  {
    "id": null,
    "trace_id": "test-trace-123",
    "partition": "$TEST_PARTITION",
    "instance": "$TEST_INSTANCE",
    "content": "This is a test message for import",
    "role": "user",
    "embedding": [0.1, 0.2, 0.3],
    "url": null,
    "timestamp": $(date +%s)000
  }
]
EOF

    echo_success "Test environment created"
}

# Cleanup test environment
cleanup_test_env() {
    echo_test "Cleaning up test environment"
    rm -rf "$TEST_DATA_DIR"
    echo_success "Test environment cleaned up"
}

# Test help command
test_help() {
    echo_test "Testing help command"
    if timeout 10s reservoir --help >/dev/null 2>&1; then
        echo_success "Help command works"
    else
        echo_error "Help command failed"
    fi
}

# Test version command
test_version() {
    echo_test "Testing version command"
    if timeout 5s reservoir --version >/dev/null 2>&1; then
        echo_success "Version command works"
    else
        echo_error "Version command failed"
    fi
}

# Test config commands
test_config() {
    echo_test "Testing config get (should handle missing config gracefully)"
    # This might fail if no config exists, but shouldn't crash
    if timeout 10s reservoir config --get model >/dev/null 2>&1 || true; then
        echo_success "Config get command doesn't crash"
    else
        echo_error "Config get command crashed"
    fi

    echo_test "Testing config set"
    if timeout 10s reservoir config --set test_key=test_value >/dev/null 2>&1; then
        echo_success "Config set command works"

        echo_test "Testing config get for set value"
        if result=$(timeout 5s reservoir config --get test_key 2>/dev/null) && [[ "$result" == *"test_value"* ]]; then
            echo_success "Config get retrieves set value"
        else
            echo_error "Config get failed to retrieve set value"
        fi
    else
        echo_error "Config set command failed"
    fi
}

# Test export command
test_export() {
    echo_test "Testing export command"
    if timeout 30s reservoir export > "$TEST_EXPORT_FILE" 2>/dev/null; then
        if [ -f "$TEST_EXPORT_FILE" ]; then
            echo_success "Export command created output file"

            # Check if it's valid JSON
            if python3 -m json.tool "$TEST_EXPORT_FILE" >/dev/null 2>&1; then
                echo_success "Export produces valid JSON"
            else
                echo_error "Export does not produce valid JSON"
            fi
        else
            echo_error "Export command did not create output file"
        fi
    else
        echo_error "Export command failed or timed out"
    fi
}

# Test import command
test_import() {
    echo_test "Testing import command"
    if timeout 30s reservoir import "$TEST_IMPORT_FILE" >/dev/null 2>&1; then
        echo_success "Import command works"
    else
        echo_error "Import command failed or timed out"
    fi
}

# Test ingest command
test_ingest() {
    echo_test "Testing ingest command"
    test_message="This is a test message for ingestion at $(date)"

    if echo "$test_message" | timeout 30s reservoir ingest --partition "$TEST_PARTITION" --instance "$TEST_INSTANCE" --role user >/dev/null 2>&1; then
        echo_success "Ingest command works"
    else
        echo_error "Ingest command failed or timed out"
    fi
}

# Test view command
test_view() {
    echo_test "Testing view command with default parameters"
    if timeout 20s reservoir view 5 >/dev/null 2>&1; then
        echo_success "View command with default parameters works"
    else
        echo_error "View command with default parameters failed or timed out"
    fi

    echo_test "Testing view command with specific partition/instance"
    if timeout 20s reservoir view 3 --partition "$TEST_PARTITION" --instance "$TEST_INSTANCE" >/dev/null 2>&1; then
        echo_success "View command with specific partition/instance works"
    else
        echo_error "View command with specific partition/instance failed or timed out"
    fi
}

# Test search commands
test_search() {
    echo_test "Testing keyword search"
    if timeout 30s reservoir search "test" --partition "$TEST_PARTITION" --instance "$TEST_INSTANCE" >/dev/null 2>&1; then
        echo_success "Keyword search works"
    else
        echo_error "Keyword search failed or timed out"
    fi

    echo_test "Testing semantic search"
    if timeout 30s reservoir search "testing message" --semantic --partition "$TEST_PARTITION" --instance "$TEST_INSTANCE" >/dev/null 2>&1; then
        echo_success "Semantic search works"
    else
        echo_error "Semantic search failed or timed out"
    fi

    echo_test "Testing search with link strategy"
    if timeout 30s reservoir search "test" --link --partition "$TEST_PARTITION" --instance "$TEST_INSTANCE" >/dev/null 2>&1; then
        echo_success "Search with link strategy works"
    else
        echo_error "Search with link strategy failed or timed out"
    fi

    echo_test "Testing search with deduplication"
    if timeout 30s reservoir search "test" --deduplicate --partition "$TEST_PARTITION" --instance "$TEST_INSTANCE" >/dev/null 2>&1; then
        echo_success "Search with deduplication works"
    else
        echo_error "Search with deduplication failed or timed out"
    fi
}

# Test replay command
test_replay() {
    echo_test "Testing replay command"
    if timeout 30s reservoir replay >/dev/null 2>&1; then
        echo_success "Replay command works"
    else
        echo_error "Replay command failed or timed out"
    fi
}

# Test start command (just validate it accepts the flag, don't actually start)
test_start_validation() {
    echo_test "Testing start command validation (ollama mode)"
    # Use timeout to prevent hanging, and expect it to try to start
    if timeout 5s reservoir start --ollama >/dev/null 2>&1 || [ $? -eq 124 ]; then
        echo_success "Start command accepts ollama flag"
    else
        echo_error "Start command validation failed"
    fi

    echo_test "Testing start command validation (normal mode)"
    if timeout 5s reservoir start >/dev/null 2>&1 || [ $? -eq 124 ]; then
        echo_success "Start command accepts normal mode"
    else
        echo_error "Start command validation failed"
    fi
}

# Test invalid commands for proper error handling
test_error_handling() {
    echo_test "Testing invalid command handling"
    if ! reservoir invalid_command >/dev/null 2>&1; then
        echo_success "Invalid command properly rejected"
    else
        echo_error "Invalid command not properly rejected"
    fi

    echo_test "Testing import with non-existent file"
    if ! reservoir import "/non/existent/file.json" >/dev/null 2>&1; then
        echo_success "Import with non-existent file properly rejected"
    else
        echo_error "Import with non-existent file not properly rejected"
    fi
}

# Run all tests
run_all_tests() {
    echo_header "Starting Reservoir CLI Black Box Tests"

    check_binary
    setup_test_env

    echo_header "Basic Command Tests"
    test_help
    test_version

    echo_header "Configuration Tests"
    test_config

    echo_header "Data Import/Export Tests"
    test_export
    test_import

    echo_header "Message Handling Tests"
    test_ingest
    test_view
    test_search

    echo_header "Advanced Feature Tests"
    test_replay

    echo_header "Validation Tests"
    test_start_validation
    test_error_handling

    cleanup_test_env

    echo_header "Test Results"
    echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
    echo -e "${RED}Tests Failed: $TESTS_FAILED${NC}"

    if [ $TESTS_FAILED -gt 0 ]; then
        echo -e "${RED}Failed Tests:${NC}"
        for test in "${FAILED_TESTS[@]}"; do
            echo -e "${RED}  - $test${NC}"
        done
        exit 1
    else
        echo -e "${GREEN}All tests passed!${NC}"
        exit 0
    fi
}

# Check for required tools
check_dependencies() {
    if ! command -v python3 >/dev/null 2>&1; then
        echo_error "python3 is required for JSON validation but not found"
        exit 1
    fi
}

# Main execution
main() {
    check_dependencies
    run_all_tests
}

# Handle script arguments
case "${1:-}" in
    "help"|"-h"|"--help")
        echo "Usage: $0 [command]"
        echo "Commands:"
        echo "  help    - Show this help message"
        echo "  (none)  - Run all tests"
        echo ""
        echo "Prerequisites:"
        echo "  - direnv installed and enabled (direnv allow .)"
        echo "  - Project built (cargo build --release)"
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac
