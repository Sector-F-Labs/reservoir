#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test configuration
RESERVOIR_PORT=${RESERVOIR_PORT:-3017}
OLLAMA_PORT=${OLLAMA_PORT:-11434}
TEST_USER=${USER:-testuser}
OPENAI_API_KEY=${OPENAI_API_KEY:-"sk-test-key"}
SERVER_PID=""
HURL_DIR="hurl"

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

echo_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Check if required tools are available
check_dependencies() {
    echo_test "Checking dependencies"

    if ! command -v hurl >/dev/null 2>&1; then
        echo_error "hurl is required but not found. Install from https://hurl.dev/"
        exit 1
    fi

    if ! command -v curl >/dev/null 2>&1; then
        echo_error "curl is required but not found"
        exit 1
    fi

    if ! command -v jq >/dev/null 2>&1; then
        echo_error "jq is required but not found"
        exit 1
    fi

    echo_success "All dependencies found"
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

# Start reservoir server in background
start_server() {
    echo_test "Starting reservoir server on port $RESERVOIR_PORT"

    # Kill any existing process on the port
    pkill -f "reservoir.*start" || true
    sleep 2

    # Start server in background
    reservoir start &
    SERVER_PID=$!

    # Wait for server to start
    echo_info "Waiting for server to start..."
    for i in {1..30}; do
        if curl -s "http://localhost:$RESERVOIR_PORT/health" >/dev/null 2>&1; then
            echo_success "Server started successfully (PID: $SERVER_PID)"
            return 0
        fi
        sleep 1
    done

    echo_error "Server failed to start within 30 seconds"
    return 1
}

# Start reservoir server in ollama mode
start_ollama_server() {
    echo_test "Starting reservoir server in ollama mode on port $OLLAMA_PORT"

    # Kill any existing process on the port
    pkill -f "reservoir.*start.*ollama" || true
    sleep 2

    # Start server in ollama mode in background
    reservoir start --ollama &
    SERVER_PID=$!

    # Wait for server to start
    echo_info "Waiting for ollama mode server to start..."
    for i in {1..30}; do
        if curl -s "http://localhost:$OLLAMA_PORT/health" >/dev/null 2>&1; then
            echo_success "Ollama mode server started successfully (PID: $SERVER_PID)"
            return 0
        fi
        sleep 1
    done

    echo_error "Ollama mode server failed to start within 30 seconds"
    return 1
}

# Stop the server
stop_server() {
    if [ -n "$SERVER_PID" ]; then
        echo_test "Stopping server (PID: $SERVER_PID)"
        kill $SERVER_PID 2>/dev/null || true
        wait $SERVER_PID 2>/dev/null || true
        echo_success "Server stopped"
        SERVER_PID=""
    fi
}

# Test basic health endpoint
test_health_endpoint() {
    echo_test "Testing health endpoint"
    if curl -s -f "http://localhost:$RESERVOIR_PORT/health" >/dev/null; then
        echo_success "Health endpoint responds"
    else
        echo_error "Health endpoint failed"
    fi
}

# Test chat completion endpoint manually
test_chat_completion_manual() {
    echo_test "Testing chat completion endpoint (manual)"

    response=$(curl -s -w "%{http_code}" -o /tmp/chat_response.json \
        -X POST "http://localhost:$RESERVOIR_PORT/partition/$TEST_USER/instance/reservoir/v1/chat/completions" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $OPENAI_API_KEY" \
        -d '{
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "user",
                    "content": "Say hello in exactly one word"
                }
            ]
        }')

    http_code="${response: -3}"

    if [ "$http_code" = "200" ]; then
        if jq -e '.choices[0].message.content' /tmp/chat_response.json >/dev/null 2>&1; then
            echo_success "Chat completion endpoint works and returns valid response"
        else
            echo_error "Chat completion endpoint returns invalid JSON structure"
        fi
    else
        echo_error "Chat completion endpoint returned HTTP $http_code"
    fi
}

# Test view endpoint
test_view_endpoint() {
    echo_test "Testing view endpoint"

    response=$(curl -s -w "%{http_code}" -o /tmp/view_response.json \
        "http://localhost:$RESERVOIR_PORT/partition/$TEST_USER/instance/reservoir/command/view/5")

    http_code="${response: -3}"

    if [ "$http_code" = "200" ]; then
        if jq -e 'type == "array"' /tmp/view_response.json >/dev/null 2>&1; then
            echo_success "View endpoint works and returns array"
        else
            echo_error "View endpoint returns invalid JSON structure"
        fi
    else
        echo_error "View endpoint returned HTTP $http_code"
    fi
}

# Test search endpoint
test_search_endpoint() {
    echo_test "Testing search endpoint (keyword)"

    response=$(curl -s -w "%{http_code}" -o /tmp/search_response.json \
        "http://localhost:$RESERVOIR_PORT/partition/$TEST_USER/instance/reservoir/command/search/5?term=hello&semantic=false")

    http_code="${response: -3}"

    if [ "$http_code" = "200" ]; then
        if jq -e 'type == "array"' /tmp/search_response.json >/dev/null 2>&1; then
            echo_success "Search endpoint works and returns array"
        else
            echo_error "Search endpoint returns invalid JSON structure"
        fi
    else
        echo_error "Search endpoint returned HTTP $http_code"
    fi
}

# Run hurl tests
run_hurl_tests() {
    echo_test "Running hurl test suite"

    if [ ! -d "$HURL_DIR" ]; then
        echo_error "Hurl directory not found: $HURL_DIR"
        return
    fi

    cd "$HURL_DIR" || return

    # Test chat completion
    echo_test "Running hurl: chat_completion.hurl"
    if hurl --variable USER="$TEST_USER" --variable OPENAI_API_KEY="$OPENAI_API_KEY" chat_completion.hurl >/dev/null 2>&1; then
        echo_success "Hurl chat completion test passed"
    else
        echo_error "Hurl chat completion test failed"
    fi

    # Test view endpoint
    echo_test "Running hurl: reservoir-view.hurl"
    if hurl --variable USER="$TEST_USER" reservoir-view.hurl >/dev/null 2>&1; then
        echo_success "Hurl view test passed"
    else
        echo_error "Hurl view test failed"
    fi

    # Test search endpoint
    echo_test "Running hurl: reservoir-search.hurl"
    if hurl --variable USER="$TEST_USER" reservoir-search.hurl >/dev/null 2>&1; then
        echo_success "Hurl search test passed"
    else
        echo_error "Hurl search test failed"
    fi

    # Test web search if available
    if [ -f "chat_with_web_search.hurl" ]; then
        echo_test "Running hurl: chat_with_web_search.hurl"
        if hurl --variable USER="$TEST_USER" --variable OPENAI_API_KEY="$OPENAI_API_KEY" chat_with_web_search.hurl >/dev/null 2>&1; then
            echo_success "Hurl web search test passed"
        else
            echo_error "Hurl web search test failed"
        fi
    fi

    cd - >/dev/null
}

# Test ollama mode endpoints
test_ollama_mode() {
    echo_test "Running hurl: ollama_mode.hurl"

    cd "$HURL_DIR" || return

    if hurl ollama_mode.hurl >/dev/null 2>&1; then
        echo_success "Hurl ollama mode test passed"
    else
        echo_error "Hurl ollama mode test failed"
    fi

    cd - >/dev/null
}

# Test invalid endpoints
test_error_handling() {
    echo_test "Testing error handling for invalid endpoints"

    # Test non-existent endpoint
    response=$(curl -s -w "%{http_code}" -o /dev/null "http://localhost:$RESERVOIR_PORT/invalid/endpoint")
    if [ "${response: -3}" = "404" ]; then
        echo_success "Invalid endpoint returns 404"
    else
        echo_error "Invalid endpoint should return 404, got ${response: -3}"
    fi

    # Test malformed request
    response=$(curl -s -w "%{http_code}" -o /dev/null \
        -X POST "http://localhost:$RESERVOIR_PORT/partition/$TEST_USER/instance/reservoir/v1/chat/completions" \
        -H "Content-Type: application/json" \
        -d '{"invalid": "json"')

    if [ "${response: -3}" != "200" ]; then
        echo_success "Malformed request properly rejected"
    else
        echo_error "Malformed request should be rejected"
    fi
}

# Cleanup function
cleanup() {
    echo_test "Cleaning up test files"
    rm -f /tmp/chat_response.json /tmp/view_response.json /tmp/search_response.json
    stop_server
}

# Trap cleanup on exit
trap cleanup EXIT

# Run all endpoint tests
run_endpoint_tests() {
    echo_header "Starting Reservoir Endpoint Tests"

    check_dependencies
    check_binary

    echo_header "Testing Normal Mode"
    start_server

    test_health_endpoint
    test_chat_completion_manual
    test_view_endpoint
    test_search_endpoint
    run_hurl_tests
    test_error_handling

    stop_server
    sleep 2

    echo_header "Testing Ollama Mode"
    start_ollama_server
    test_ollama_mode
    stop_server

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
        echo -e "${GREEN}All endpoint tests passed!${NC}"
        exit 0
    fi
}

# Main execution
main() {
    case "${1:-}" in
        "help"|"-h"|"--help")
            echo "Usage: $0 [command]"
            echo "Commands:"
            echo "  help    - Show this help message"
            echo "  (none)  - Run all endpoint tests"
            echo ""
            echo "Prerequisites:"
            echo "  - direnv installed and enabled (direnv allow .)"
            echo "  - Project built (cargo build --release)"
            echo ""
            echo "Environment Variables:"
            echo "  RESERVOIR_PORT - Port for normal mode (default: 3017)"
            echo "  OLLAMA_PORT    - Port for ollama mode (default: 11434)"
            echo "  OPENAI_API_KEY - API key for testing (default: sk-test-key)"
            exit 0
            ;;
        *)
            run_endpoint_tests
            ;;
    esac
}

main "$@"
