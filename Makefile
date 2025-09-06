.PHONY: install-service uninstall-service book clean-book serve-book test test-cli test-endpoints test-unit test-lint setup dev-setup help

.DEFAULT_GOAL := help

help:
	@echo "Available targets:"
	@echo ""
	@echo "Development:"
	@echo "  setup           - Set up development environment"
	@echo "  dev-setup       - Check development prerequisites"
	@echo "  main            - Build the release binary"
	@echo "  run             - Run the application"
	@echo "  dev             - Run with cargo watch for development"
	@echo "  install         - Install the binary to cargo's bin directory"
	@echo ""
	@echo "Testing:"
	@echo "  test            - Run comprehensive test suite (unit, CLI, endpoints)"
	@echo "  test-cli        - Run CLI black box tests only"
	@echo "  test-endpoints  - Run endpoint integration tests only"
	@echo "  test-unit       - Run unit tests only"
	@echo "  test-lint       - Run lint checks only"
	@echo ""
	@echo "Database:"
	@echo "  migrate         - Run database migrations"
	@echo ""
	@echo "Services:"
	@echo "  install-service - Install as a macOS LaunchAgent service"
	@echo "  uninstall-service - Remove the macOS LaunchAgent service"
	@echo ""
	@echo "Documentation:"
	@echo "  book            - Build documentation to docs/ folder"
	@echo "  clean-book      - Clean generated documentation"
	@echo "  serve-book      - Serve documentation locally with live reload"
	@echo ""
	@echo "  help            - Show this help message"

main:
	cargo build --release

migrate:
	./scripts/migrations.sh

run:
	cargo run -- start

dev:
	cargo watch -x 'run -- start'

install:
	cargo install --path .

install-service:
	@echo "Copying plist to LaunchAgents..."
	mkdir -p ~/Library/LaunchAgents
	cp scripts/com.sectorflabs.reservoir.plist ~/Library/LaunchAgents/com.sectorflabs.reservoir.plist
	launchctl unload -w ~/Library/LaunchAgents/com.sectorflabs.reservoir.plist || true
	launchctl load -w ~/Library/LaunchAgents/com.sectorflabs.reservoir.plist
	@echo "Service installed and started."

uninstall-service:
	@echo "Unloading and removing service..."
	launchctl unload -w ~/Library/LaunchAgents/com.sectorflabs.reservoir.plist || true
	rm -f ~/Library/LaunchAgents/com.sectorflabs.reservoir.plist
	@echo "Service removed."

book:
	@echo "Building documentation..."
	cd book && mdbook build --dest-dir ../docs
	@echo "Documentation built to docs/ folder"

clean-book:
	@echo "Cleaning documentation..."
	rm -rf docs/*
	@echo "Documentation cleaned"

serve-book:
	@echo "Serving documentation locally..."
	cd book && mdbook serve --open

test:
	@echo "Running comprehensive test suite..."
	./scripts/test_all.sh

test-cli:
	@echo "Running CLI black box tests..."
	./scripts/test_all.sh cli

test-endpoints:
	@echo "Running endpoint integration tests..."
	./scripts/test_all.sh endpoints

test-unit:
	@echo "Running unit tests..."
	./scripts/test_all.sh unit

test-lint:
	@echo "Running lint checks..."
	./scripts/test_all.sh lint

setup: dev-setup main
	@echo "✅ Development environment setup complete!"
	@echo ""
	@echo "Next steps:"
	@echo "  1. Ensure direnv is configured in your shell"
	@echo "  2. Run 'direnv allow .' to load environment"
	@echo "  3. Start developing with 'reservoir --help'"

dev-setup:
	@echo "🔧 Checking development prerequisites..."
	@command -v direnv >/dev/null 2>&1 || { echo "❌ direnv not found. Please install direnv first."; exit 1; }
	@command -v cargo >/dev/null 2>&1 || { echo "❌ cargo not found. Please install Rust first."; exit 1; }
	@command -v hurl >/dev/null 2>&1 || { echo "⚠️  hurl not found. Install for endpoint testing: brew install hurl"; }
	@command -v jq >/dev/null 2>&1 || { echo "⚠️  jq not found. Install for JSON processing: brew install jq"; }
	@command -v python3 >/dev/null 2>&1 || { echo "⚠️  python3 not found. Install for JSON validation."; }
	@echo "✅ Development prerequisites check complete!"
	@echo ""
	@if [ ! -f .envrc ]; then echo "❌ .envrc not found"; exit 1; fi
	@echo "💡 Run 'direnv allow .' to load the development environment"
