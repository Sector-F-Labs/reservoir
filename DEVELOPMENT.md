# Reservoir Development Guide

This guide covers the development environment setup and workflow for Reservoir.

## Prerequisites

### Required Tools

- **Rust** (latest stable): Install via [rustup.rs](https://rustup.rs/)
- **direnv**: Environment variable management
  - macOS: `brew install direnv`
  - Ubuntu/Debian: `apt install direnv`
  - Other: See [direnv installation guide](https://direnv.net/docs/installation.html)
- **hurl**: HTTP testing tool
  - macOS: `brew install hurl`
  - Other: See [hurl installation guide](https://hurl.dev/docs/installation.html)

### Optional Tools

- **Neo4j**: Graph database (for full functionality)
  - Docker: `docker run -p 7474:7474 -p 7687:7687 neo4j:latest`
  - Or install locally from [neo4j.com](https://neo4j.com/download/)

## Development Environment Setup

### 1. Clone and Setup

```bash
git clone https://github.com/sector-f-labs/reservoir.git
cd reservoir
```

### 2. Configure direnv

```bash
# Allow direnv to load the environment
direnv allow .

# Copy example local config (optional)
cp .envrc.local.example .envrc.local
# Edit .envrc.local with your settings
```

The `.envrc` file automatically:
- Adds `target/release` and `target/debug` to your PATH
- Sets up development environment variables
- Makes test scripts available in PATH
- Configures logging and ports

### 3. Build the Project

```bash
# Build release version (recommended for testing)
cargo build --release

# Or build debug version
cargo build
```

After building, the `reservoir` command will be available in your PATH thanks to direnv.

## Development Workflow

### Running Reservoir

```bash
# Start the server
reservoir start

# Start in ollama compatibility mode
reservoir start --ollama

# View help
reservoir --help
```

### Testing

#### Unit Tests
```bash
# Run all unit tests
cargo test

# Or use make target
make test-unit
```

#### Integration Tests
```bash
# Run all tests (requires hurl)
make test

# Run specific test suites
make test-cli        # CLI command tests
make test-endpoints  # HTTP endpoint tests
make test-lint       # Code quality checks
```

#### Manual Testing
```bash
# Test CLI commands
reservoir export
reservoir view 10
reservoir search "test query"

# Test HTTP endpoints (with server running)
curl http://localhost:3017/partition/testuser/instance/reservoir/command/view/5
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for issues
cargo check
```

## Project Structure

```
reservoir/
├── src/
│   ├── main.rs              # Application entry point
│   ├── args.rs              # CLI argument parsing
│   ├── clients/             # External API clients
│   ├── commands/            # CLI command implementations
│   ├── handler/             # HTTP request handlers
│   ├── models/              # Data structures
│   ├── repos/               # Repository layer (NEW!)
│   │   ├── traits.rs        # Repository abstractions
│   │   ├── neo4j_factory.rs # Neo4j repository factory
│   │   ├── message/         # Message repositories
│   │   └── embedding/       # Embedding repositories
│   ├── services/            # Business logic
│   └── utils/               # Utilities
├── scripts/                 # Development and test scripts
├── hurl/                    # HTTP endpoint tests
├── cpd/                     # Change Proposal Documents
└── book/                    # Documentation source
```

## Architecture Notes

### Repository Pattern (NEW!)

Reservoir now uses a repository pattern with trait-based abstractions:

- **`MessageRepository`**: Interface for message storage operations
- **`EmbeddingRepository`**: Interface for embedding operations  
- **`RepositoryFactory`**: Creates repository instances
- **Neo4j Implementation**: Current implementation using Neo4j
- **File-based Implementation**: Coming soon!

This allows swapping storage backends without changing business logic.

### Storage Backends

Currently supported:
- **Neo4j**: Graph database with vector search capabilities

Planned:
- **Filesystem**: XDG-compliant file-based storage

## Environment Variables

Key environment variables (set by `.envrc`):

```bash
# Server configuration
RESERVOIR_PORT=3017          # HTTP server port
OLLAMA_PORT=11434           # Ollama compatibility port

# Database configuration
NEO4J_URI=neo4j://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=password

# Development
RUST_LOG=debug              # Logging level
RUST_BACKTRACE=1           # Show backtraces
OPENAI_API_KEY=sk-test-key # Test API key
```

Override any of these in `.envrc.local` for your local setup.

## Common Tasks

### Adding a New Command

1. Add command struct to `src/args.rs`
2. Implement command logic in `src/commands/`
3. Wire up in `src/main.rs`
4. Add tests to `scripts/test_cli.sh`

### Adding a New Repository

1. Define interface in `src/repos/traits.rs`
2. Implement for existing backends
3. Create factory method
4. Update services to use new interface

### Debugging

```bash
# Run with detailed logging
RUST_LOG=trace reservoir start

# Run with backtrace
RUST_BACKTRACE=full reservoir start

# Debug specific modules
RUST_LOG=reservoir::repos=debug reservoir start
```

## Troubleshooting

### Common Issues

**`reservoir` command not found**
- Ensure direnv is installed and allowed: `direnv allow .`
- Build the project: `cargo build --release`

**Tests hanging**
- Check if Neo4j is running (required for some tests)
- Use timeouts: most test commands have built-in timeouts

**Neo4j connection errors**
- Start Neo4j: `docker run -p 7474:7474 -p 7687:7687 neo4j:latest`
- Check `NEO4J_*` environment variables

**Port conflicts**
- Change ports in `.envrc.local`:
  ```bash
  export RESERVOIR_PORT=3018
  export OLLAMA_PORT=11435
  ```

### Getting Help

- Check the [main README](./README.md) for usage information
- Review [Change Proposal Documents](./cpd/) for architectural decisions
- Run `reservoir --help` for command reference
- Run test scripts with `help` argument for testing info

## Contributing

1. Set up development environment as above
2. Create feature branch: `git checkout -b feature/my-feature`
3. Make changes and add tests
4. Run test suite: `make test`
5. Format and lint: `cargo fmt && cargo clippy`
6. Submit pull request

The comprehensive test suite ensures your changes don't break existing functionality!