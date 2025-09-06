#!/bin/bash

# Don't exit on error - we want to collect all check results
set +e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

echo_error() {
    echo -e "${RED}✗ $1${NC}"
}

echo_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

echo_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

echo_header() {
    echo -e "${BLUE}===============================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}===============================================${NC}"
}

check_direnv() {
    echo_header "Checking direnv Environment"

    # Check if direnv is available
    if command -v direnv >/dev/null 2>&1; then
        echo_success "direnv is installed: $(which direnv)"
    else
        echo_error "direnv is not installed"
        echo_info "Install with: brew install direnv (macOS) or apt install direnv (Ubuntu)"
        return 1
    fi

    # Check if .envrc exists
    if [ -f ".envrc" ]; then
        echo_success ".envrc file exists"
    else
        echo_error ".envrc file not found"
        return 1
    fi

    # Check if .envrc is allowed
    if direnv status | grep -q "Found RC allowed true"; then
        echo_success ".envrc is allowed"
    else
        echo_warning ".envrc may not be allowed - run 'direnv allow .'"
    fi
}

check_path() {
    echo_header "Checking PATH Configuration"

    # Check if reservoir is in PATH
    if command -v reservoir >/dev/null 2>&1; then
        echo_success "reservoir binary found in PATH: $(which reservoir)"
    else
        echo_error "reservoir binary not found in PATH"
        echo_info "Make sure direnv is loaded and project is built"
        return 1
    fi

    # Check if test scripts are in PATH
    if command -v test_simple.sh >/dev/null 2>&1; then
        echo_success "test scripts found in PATH: $(which test_simple.sh)"
    else
        echo_warning "test scripts not in PATH (may need to reload direnv)"
    fi
}

check_environment_vars() {
    echo_header "Checking Environment Variables"

    # Check key environment variables
    local vars=(
        "RESERVOIR_PORT"
        "RUST_LOG"
        "NEO4J_URI"
        "OPENAI_API_KEY"
    )

    for var in "${vars[@]}"; do
        if [ -n "${!var}" ]; then
            echo_success "$var = ${!var}"
        else
            echo_warning "$var is not set"
        fi
    done
}

check_build_status() {
    echo_header "Checking Build Status"

    # Check if release binary exists
    if [ -f "target/release/reservoir" ]; then
        echo_success "Release binary exists"
        echo_info "Built: $(stat -c %y target/release/reservoir 2>/dev/null || stat -f %Sm target/release/reservoir 2>/dev/null || echo 'unknown')"
    else
        echo_error "Release binary not found - run 'cargo build --release'"
    fi

    # Check if debug binary exists
    if [ -f "target/debug/reservoir" ]; then
        echo_success "Debug binary exists"
    else
        echo_warning "Debug binary not found - run 'cargo build' if needed"
    fi
}

check_dependencies() {
    echo_header "Checking External Dependencies"

    local deps=(
        "cargo:Rust toolchain"
        "hurl:HTTP testing"
        "jq:JSON processing"
        "python3:Script utilities"
        "curl:HTTP client"
    )

    for dep in "${deps[@]}"; do
        local cmd="${dep%%:*}"
        local desc="${dep##*:}"

        if command -v "$cmd" >/dev/null 2>&1; then
            echo_success "$desc: $cmd available"
        else
            echo_warning "$desc: $cmd not found"
        fi
    done
}

test_basic_functionality() {
    echo_header "Testing Basic Functionality"

    # Test help command
    if reservoir --help >/dev/null 2>&1; then
        echo_success "reservoir --help works"
    else
        echo_error "reservoir --help failed"
    fi

    # Test version command
    if reservoir --version >/dev/null 2>&1; then
        local version=$(reservoir --version)
        echo_success "reservoir --version works: $version"
    else
        echo_error "reservoir --version failed"
    fi
}

main() {
    echo_header "Reservoir Development Environment Check"

    local checks_passed=0
    local total_checks=6

    check_direnv && ((checks_passed++)) || true
    check_path && ((checks_passed++)) || true
    check_environment_vars && ((checks_passed++)) || true
    check_build_status && ((checks_passed++)) || true
    check_dependencies && ((checks_passed++)) || true
    test_basic_functionality && ((checks_passed++)) || true

    echo_header "Summary"
    echo_info "Passed: $checks_passed/$total_checks checks"

    if [ $checks_passed -eq $total_checks ]; then
        echo_success "Development environment is ready! 🎉"
        echo_info "You can now run 'reservoir --help' and start developing"
        exit 0
    else
        echo_warning "Some checks failed - see above for details"
        echo_info "Common fixes:"
        echo_info "  - Run 'direnv allow .' to load environment"
        echo_info "  - Run 'cargo build --release' to build"
        echo_info "  - Install missing dependencies"
        exit 1
    fi
}

# Handle script arguments
case "${1:-}" in
    "help"|"-h"|"--help")
        echo "Reservoir Development Environment Checker"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  help    - Show this help message"
        echo "  (none)  - Run all checks"
        echo ""
        echo "This script verifies:"
        echo "  - direnv configuration"
        echo "  - PATH setup"
        echo "  - Environment variables"
        echo "  - Build status"
        echo "  - External dependencies"
        echo "  - Basic functionality"
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac
