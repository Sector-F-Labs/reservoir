.PHONY: install-service uninstall-service book clean-book serve-book help

.DEFAULT_GOAL := help

help:
	@echo "Available targets:"
	@echo "  main            - Build the release binary"
	@echo "  migrate         - Run database migrations"
	@echo "  run             - Run the application"
	@echo "  dev             - Run with cargo watch for development"
	@echo "  install         - Install the binary to cargo's bin directory"
	@echo "  install-service - Install as a macOS LaunchAgent service"
	@echo "  uninstall-service - Remove the macOS LaunchAgent service"
	@echo "  book            - Build documentation to docs/ folder"
	@echo "  clean-book      - Clean generated documentation"
	@echo "  serve-book      - Serve documentation locally with live reload"
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
