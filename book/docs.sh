#!/bin/bash
set -e

# Convenience script for Reservoir documentation tasks

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BOOK_DIR="$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if mdbook is installed
check_mdbook() {
    if ! command -v mdbook &> /dev/null; then
        print_error "mdbook is not installed. Install it with:"
        echo "  cargo install mdbook"
        exit 1
    fi
}

# Check if mdbook-mermaid is installed (optional)
check_mermaid() {
    if ! command -v mdbook-mermaid &> /dev/null; then
        print_warning "mdbook-mermaid is not installed. Diagrams won't render."
        print_warning "Install it with: cargo install mdbook-mermaid"
    fi
}

# Show usage information
usage() {
    echo "Reservoir Documentation Helper"
    echo ""
    echo "Usage: $0 <command>"
    echo ""
    echo "Commands:"
    echo "  build       Build the documentation"
    echo "  serve       Serve documentation locally with live reload"
    echo "  watch       Watch for changes and rebuild"
    echo "  clean       Clean build artifacts"
    echo "  check       Check for broken links and issues"
    echo "  new PAGE    Create a new documentation page"
    echo "  install     Install required dependencies"
    echo "  help        Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 serve                    # Start local development server"
    echo "  $0 new features/webhooks    # Create new page at src/features/webhooks.md"
    echo "  $0 build                    # Build the documentation"
    echo ""
}

# Install dependencies
install_deps() {
    print_status "Installing mdBook dependencies..."

    if ! command -v mdbook &> /dev/null; then
        print_status "Installing mdbook..."
        cargo install mdbook
    else
        print_success "mdbook is already installed"
    fi

    if ! command -v mdbook-mermaid &> /dev/null; then
        print_status "Installing mdbook-mermaid..."
        cargo install mdbook-mermaid
    else
        print_success "mdbook-mermaid is already installed"
    fi

    print_success "All dependencies installed!"
}

# Build the documentation
build_docs() {
    print_status "Building documentation..."
    cd "$BOOK_DIR"
    check_mdbook
    check_mermaid

    if mdbook build; then
        print_success "Documentation built successfully!"
        print_status "Output available in: $BOOK_DIR/book/"
    else
        print_error "Build failed!"
        exit 1
    fi
}

# Serve documentation locally
serve_docs() {
    print_status "Starting local documentation server..."
    cd "$BOOK_DIR"
    check_mdbook
    check_mermaid

    print_status "Documentation will be available at: http://localhost:3000"
    print_status "Press Ctrl+C to stop the server"
    mdbook serve --open
}

# Watch for changes
watch_docs() {
    print_status "Watching for changes..."
    cd "$BOOK_DIR"
    check_mdbook

    print_status "Watching for changes in: $BOOK_DIR/src/"
    print_status "Press Ctrl+C to stop watching"
    mdbook watch
}

# Clean build artifacts
clean_docs() {
    print_status "Cleaning build artifacts..."
    cd "$BOOK_DIR"

    if [ -d "book" ]; then
        rm -rf book
        print_success "Cleaned build directory"
    else
        print_status "No build artifacts to clean"
    fi
}

# Check for issues
check_docs() {
    print_status "Checking documentation for issues..."
    cd "$BOOK_DIR"
    check_mdbook

    # Build to catch any build errors
    if ! mdbook build --dest-dir temp_build &> /dev/null; then
        print_error "Build check failed - there are build errors"
        mdbook build --dest-dir temp_build
        rm -rf temp_build 2>/dev/null || true
        exit 1
    fi

    rm -rf temp_build 2>/dev/null || true

    # Check for common issues
    print_status "Checking for common issues..."

    # Check if all files in SUMMARY.md exist
    missing_files=()
    while IFS= read -r line; do
        if [[ $line =~ \[.*\]\((.*\.md)\) ]]; then
            file="${BASH_REMATCH[1]}"
            # Remove leading ./ if present
            file="${file#./}"
            if [[ ! -f "src/$file" ]]; then
                missing_files+=("$file")
            fi
        fi
    done < "src/SUMMARY.md"

    if [ ${#missing_files[@]} -gt 0 ]; then
        print_error "Missing files referenced in SUMMARY.md:"
        for file in "${missing_files[@]}"; do
            echo "  - $file"
        done
        exit 1
    fi

    print_success "Documentation check passed!"
}

# Create a new documentation page
new_page() {
    local page_path="$1"

    if [ -z "$page_path" ]; then
        print_error "Please specify a page path"
        echo "Usage: $0 new <path>"
        echo "Example: $0 new features/webhooks"
        exit 1
    fi

    # Ensure .md extension
    if [[ ! "$page_path" =~ \.md$ ]]; then
        page_path="$page_path.md"
    fi

    local full_path="$BOOK_DIR/src/$page_path"
    local dir_path=$(dirname "$full_path")

    # Create directory if it doesn't exist
    if [ ! -d "$dir_path" ]; then
        mkdir -p "$dir_path"
        print_status "Created directory: $dir_path"
    fi

    # Check if file already exists
    if [ -f "$full_path" ]; then
        print_error "File already exists: $page_path"
        exit 1
    fi

    # Generate page title from filename
    local filename=$(basename "$page_path" .md)
    local title=$(echo "$filename" | sed 's/-/ /g' | sed 's/\b\w/\U&/g')

    # Create the new page
    cat > "$full_path" << EOF
# $title

<!-- Add your content here -->

## Overview

Describe what this page covers.

## Examples

\`\`\`bash
# Add example commands
echo "Hello, World!"
\`\`\`

## See Also

- [Related Page 1](./other-page.md)
- [Related Page 2](../api/overview.md)
EOF

    print_success "Created new page: $page_path"
    print_warning "Don't forget to add it to src/SUMMARY.md!"

    # Offer to open the file
    if command -v code &> /dev/null; then
        echo ""
        read -p "Open in VS Code? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            code "$full_path"
        fi
    fi
}

# Main command dispatcher
main() {
    case "${1:-}" in
        "build")
            build_docs
            ;;
        "serve")
            serve_docs
            ;;
        "watch")
            watch_docs
            ;;
        "clean")
            clean_docs
            ;;
        "check")
            check_docs
            ;;
        "new")
            new_page "$2"
            ;;
        "install")
            install_deps
            ;;
        "help"|"--help"|"-h")
            usage
            ;;
        *)
            if [ -n "${1:-}" ]; then
                print_error "Unknown command: $1"
                echo ""
            fi
            usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
