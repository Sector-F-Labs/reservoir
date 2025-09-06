#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

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
}

echo_error() {
    echo -e "${RED}✗ $1${NC}"
}

echo_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Check if required dependencies are available
check_dependencies() {
    echo_test "Checking system dependencies"

    local missing_deps=()

    if ! command -v cargo >/dev/null 2>&1; then
        missing_deps+=("cargo (Rust toolchain)")
    fi

    if ! command -v hurl >/dev/null 2>&1; then
        missing_deps+=("hurl (https://hurl.dev/)")
    fi

    if ! command -v curl >/dev/null 2>&1; then
        missing_deps+=("curl")
    fi

    if ! command -v jq >/dev/null 2>&1; then
        missing_deps+=("jq")
    fi

    if ! command -v python3 >/dev/null 2>&1; then
        missing_deps+=("python3")
    fi

    if [ ${#missing_deps[@]} -eq 0 ]; then
        echo_success "All dependencies found"
    else
        echo_error "Missing dependencies:"
        for dep in "${missing_deps[@]}"; do
            echo -e "${RED}  - $dep${NC}"
        done
        exit 1
    fi
}

# Build the project
build_project() {
    echo_test "Building reservoir project"
    cd "$PROJECT_ROOT"

    if cargo build --release >/dev/null 2>&1; then
        echo_success "Project built successfully"
    else
        echo_error "Project build failed"
        exit 1
    fi
}

# Run CLI tests
run_cli_tests() {
    echo_header "Running CLI Black Box Tests"
    cd "$PROJECT_ROOT"

    if [ -f "$SCRIPT_DIR/test_cli.sh" ]; then
        if bash "$SCRIPT_DIR/test_cli.sh"; then
            echo_success "CLI tests passed"
            return 0
        else
            echo_error "CLI tests failed"
            return 1
        fi
    else
        echo_error "CLI test script not found: $SCRIPT_DIR/test_cli.sh"
        return 1
    fi
}

# Run endpoint tests
run_endpoint_tests() {
    echo_header "Running Endpoint Tests"
    cd "$PROJECT_ROOT"

    if [ -f "$SCRIPT_DIR/test_endpoints.sh" ]; then
        if bash "$SCRIPT_DIR/test_endpoints.sh"; then
            echo_success "Endpoint tests passed"
            return 0
        else
            echo_error "Endpoint tests failed"
            return 1
        fi
    else
        echo_error "Endpoint test script not found: $SCRIPT_DIR/test_endpoints.sh"
        return 1
    fi
}

# Run unit tests
run_unit_tests() {
    echo_header "Running Unit Tests"
    cd "$PROJECT_ROOT"

    if cargo test >/dev/null 2>&1; then
        echo_success "Unit tests passed"
        return 0
    else
        echo_error "Unit tests failed"
        return 1
    fi
}

# Run linting checks
run_lint_checks() {
    echo_header "Running Lint Checks"
    cd "$PROJECT_ROOT"

    echo_test "Running cargo check"
    if cargo check >/dev/null 2>&1; then
        echo_success "Cargo check passed"
    else
        echo_error "Cargo check failed"
        return 1
    fi

    echo_test "Running cargo clippy"
    if cargo clippy -- -D warnings >/dev/null 2>&1; then
        echo_success "Clippy passed"
    else
        echo_info "Clippy found warnings (not failing build)"
    fi

    echo_test "Running cargo fmt check"
    if cargo fmt -- --check >/dev/null 2>&1; then
        echo_success "Format check passed"
    else
        echo_info "Format check found issues (not failing build)"
    fi
}

# Main test execution
run_all_tests() {
    local cli_result=0
    local endpoint_result=0
    local unit_result=0
    local lint_result=0

    echo_header "Starting Comprehensive Reservoir Test Suite"
    echo_info "Project root: $PROJECT_ROOT"

    check_dependencies
    build_project

    # Run unit tests first (fastest)
    if ! run_unit_tests; then
        unit_result=1
    fi

    # Run lint checks
    if ! run_lint_checks; then
        lint_result=1
    fi

    # Run CLI tests
    if ! run_cli_tests; then
        cli_result=1
    fi

    # Run endpoint tests (slowest, requires server)
    if ! run_endpoint_tests; then
        endpoint_result=1
    fi

    # Summary
    echo_header "Test Suite Summary"

    if [ $unit_result -eq 0 ]; then
        echo_success "Unit Tests: PASSED"
    else
        echo_error "Unit Tests: FAILED"
    fi

    if [ $lint_result -eq 0 ]; then
        echo_success "Lint Checks: PASSED"
    else
        echo_error "Lint Checks: FAILED"
    fi

    if [ $cli_result -eq 0 ]; then
        echo_success "CLI Tests: PASSED"
    else
        echo_error "CLI Tests: FAILED"
    fi

    if [ $endpoint_result -eq 0 ]; then
        echo_success "Endpoint Tests: PASSED"
    else
        echo_error "Endpoint Tests: FAILED"
    fi

    local total_failures=$((unit_result + lint_result + cli_result + endpoint_result))

    if [ $total_failures -eq 0 ]; then
        echo_header "🎉 ALL TESTS PASSED! 🎉"
        echo_success "Reservoir is ready for deployment"
        exit 0
    else
        echo_header "❌ SOME TESTS FAILED ❌"
        echo_error "$total_failures test suite(s) failed"
        exit 1
    fi
}

# Handle script arguments
case "${1:-}" in
    "help"|"-h"|"--help")
        echo "Reservoir Comprehensive Test Suite"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  help       - Show this help message"
        echo "  cli        - Run only CLI tests"
        echo "  endpoints  - Run only endpoint tests"
        echo "  unit       - Run only unit tests"
        echo "  lint       - Run only lint checks"
        echo "  build      - Build project only"
        echo "  (none)     - Run all tests"
        echo ""
        echo "This script will:"
        echo "  1. Check system dependencies"
        echo "  2. Build the project"
        echo "  3. Run unit tests"
        echo "  4. Run lint checks"
        echo "  5. Run CLI black box tests"
        echo "  6. Run endpoint integration tests"
        echo ""
        echo "Environment Variables:"
        echo "  RESERVOIR_PORT - Port for normal mode (default: 3017)"
        echo "  OLLAMA_PORT    - Port for ollama mode (default: 11434)"
        echo "  OPENAI_API_KEY - API key for testing (default: sk-test-key)"
        exit 0
        ;;
    "cli")
        check_dependencies
        build_project
        run_cli_tests
        ;;
    "endpoints")
        check_dependencies
        build_project
        run_endpoint_tests
        ;;
    "unit")
        check_dependencies
        build_project
        run_unit_tests
        ;;
    "lint")
        check_dependencies
        run_lint_checks
        ;;
    "build")
        check_dependencies
        build_project
        ;;
    *)
        run_all_tests
        ;;
esac
