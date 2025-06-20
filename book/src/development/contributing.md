# Contributing to Reservoir

Thank you for your interest in contributing to Reservoir! This guide will help you get started with development and contributing to the project.

## Development Setup

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (latest stable version)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source ~/.cargo/env
  ```

- **Docker** (for Neo4j database)
  - [Install Docker](https://docs.docker.com/get-docker/)
  - [Install Docker Compose](https://docs.docker.com/compose/install/)

- **Git** for version control

### Step 1: Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:

```bash
git clone https://github.com/yourusername/reservoir.git
cd reservoir
```

### Step 2: Start the Database

Start Neo4j using Docker Compose:

```bash
docker-compose up -d
```

This starts Neo4j on `bolt://localhost:7687` with the default credentials.

### Step 3: Environment Configuration

Create a `.env` file in the project root or export environment variables:

```env
# Server Configuration
RESERVOIR_PORT=3017
RESERVOIR_HOST=127.0.0.1

# Database Configuration
NEO4J_URI=bolt://localhost:7687
NEO4J_USERNAME=neo4j
NEO4J_PASSWORD=password

# API Keys (add as needed)
OPENAI_API_KEY=sk-your-key-here
MISTRAL_API_KEY=your-mistral-key
GEMINI_API_KEY=your-gemini-key

# Provider URLs (optional)
RSV_OPENAI_BASE_URL=https://api.openai.com/v1/chat/completions
RSV_OLLAMA_BASE_URL=http://localhost:11434/v1/chat/completions
```

### Step 4: Build and Run

```bash
# Build the project
cargo build

# Run in development mode with auto-reload
make dev

# Or run directly
cargo run -- start
```

Reservoir will be available at `http://localhost:3017`.

## Development Workflow

### Making Changes

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding standards below

3. **Test your changes**:
   ```bash
   # Run tests
   cargo test
   
   # Run API tests
   ./hurl/test.sh
   
   # Test specific functionality
   make run
   ```

4. **Update documentation** if needed:
   ```bash
   # Build documentation
   make book
   
   # Serve locally to preview
   make serve-book
   ```

### Code Standards

#### Rust Code Style

- Use `rustfmt` for formatting:
  ```bash
  cargo fmt
  ```

- Use `clippy` for linting:
  ```bash
  cargo clippy
  ```

- Follow Rust naming conventions:
  - `snake_case` for functions and variables
  - `PascalCase` for types and structs
  - `SCREAMING_SNAKE_CASE` for constants

#### Documentation

- Document all public APIs with rustdoc comments
- Include examples in documentation where helpful
- Update the book documentation for user-facing changes

#### Testing

- Write unit tests for new functionality
- Add integration tests for API endpoints
- Ensure existing tests pass before submitting

### Commit Guidelines

Use conventional commit messages:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(api): add web search options support
fix(db): resolve connection pooling issue
docs(book): update installation guide
```

## Testing

### Unit Tests

Run unit tests:
```bash
cargo test
```

### Integration Tests

Test the API endpoints:
```bash
# Test all endpoints
./hurl/test.sh

# Test specific endpoint
hurl --variable USER="$USER" --variable OPENAI_API_KEY="$OPENAI_API_KEY" hurl/chat_completion.hurl
```

### Manual Testing

1. Start Reservoir:
   ```bash
   make run
   ```

2. Test with curl:
   ```bash
   curl "http://127.0.0.1:3017/partition/$USER/instance/test/v1/chat/completions" \
       -H "Content-Type: application/json" \
       -H "Authorization: Bearer $OPENAI_API_KEY" \
       -d '{"model": "gpt-4", "messages": [{"role": "user", "content": "Hello!"}]}'
   ```

## Documentation

### Building the Book

The documentation is built with mdBook:

```bash
# Build documentation
make book

# Serve with live reload
make serve-book

# Clean generated docs
make clean-book
```

### Writing Documentation

- Use clear, concise language
- Include code examples
- Test all code examples
- Link related sections
- Consider the user's journey

## Submitting Changes

### Pull Request Process

1. **Ensure your code is ready**:
   - [ ] Tests pass (`cargo test`)
   - [ ] Code is formatted (`cargo fmt`)
   - [ ] No clippy warnings (`cargo clippy`)
   - [ ] Documentation updated if needed

2. **Create a pull request**:
   - Use a descriptive title
   - Explain what changes you made and why
   - Reference any related issues
   - Include screenshots for UI changes

3. **Respond to feedback**:
   - Address review comments promptly
   - Ask questions if feedback is unclear
   - Update your branch as needed

### Pull Request Template

When creating a PR, include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

## Release Process

For maintainers:

1. **Version Bump**: Update version in `Cargo.toml`
2. **Changelog**: Update `CHANGELOG.md` with changes
3. **Tag Release**: Create and push a git tag
4. **Build Documentation**: Ensure docs are up to date
5. **Publish**: Publish to crates.io if applicable

## Getting Help

- **Issues**: Check existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Refer to the full documentation at [sectorflabs.com/reservoir](http://sectorflabs.com/reservoir/)

## Code of Conduct

Please be respectful and constructive in all interactions. We're here to build something useful together!

## Architecture Overview

Before making significant changes, familiarize yourself with:

- [System Architecture](../architecture/overview.md)
- [Data Model](../architecture/data-model.md)
- [API Design](../api/overview.md)

## Common Development Tasks

### Adding a New API Endpoint

1. Define the endpoint in `src/api/`
2. Add routing logic
3. Implement request/response handling
4. Add tests
5. Update documentation

### Adding a New AI Provider

1. Implement provider trait
2. Add configuration options
3. Update model routing logic
4. Add tests with mock responses
5. Document the new provider

### Database Schema Changes

1. Create migration script in `migrations/`
2. Update data model documentation
3. Test migration on sample data
4. Ensure backward compatibility

Thank you for contributing to Reservoir! 🚀