# Installation

This guide will walk you through installing and setting up Reservoir on your system.

## Prerequisites

Before installing Reservoir, make sure you have the following dependencies installed:

### Required Dependencies

1. **Rust and Cargo** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Neo4j Database** (version 4.4 or later)
   
   **Option A: Using Docker (Recommended)**
   ```bash
   docker run \
       --name neo4j \
       -p7474:7474 -p7687:7687 \
       -d \
       -v $HOME/neo4j/data:/data \
       -v $HOME/neo4j/logs:/logs \
       -v $HOME/neo4j/import:/var/lib/neo4j/import \
       -v $HOME/neo4j/plugins:/plugins \
       --env NEO4J_AUTH=neo4j/password \
       neo4j:latest
   ```

   **Option B: Native Installation**
   - Download from [Neo4j Download Center](https://neo4j.com/download/)
   - Follow the installation instructions for your operating system

### Optional Dependencies

1. **mdBook** (for building documentation)
   ```bash
   cargo install mdbook
   ```

2. **Hurl** (for running API tests)
   ```bash
   # macOS
   brew install hurl
   
   # Linux
   curl --location --remote-name https://github.com/Orange-OpenSource/hurl/releases/latest/download/hurl_amd64.deb
   sudo dpkg -i hurl_amd64.deb
   ```

## Installing Reservoir

### From Source (Recommended)

1. **Clone the repository**
   ```bash
   git clone https://github.com/divanvisagie/reservoir.git
   cd reservoir
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Install the binary** (optional)
   ```bash
   cargo install --path .
   ```

### Using Cargo Install

Once Reservoir is published to crates.io, you'll be able to install it directly:

```bash
cargo install reservoir
```

## Configuration

### Environment Variables

Create an `.env` file in your project directory or set these environment variables:

```bash
# Neo4j Configuration
NEO4J_URI=bolt://localhost:7687
NEO4J_USERNAME=neo4j
NEO4J_PASSWORD=password

# Server Configuration
RESERVOIR_PORT=3017
RESERVOIR_HOST=127.0.0.1

# API Keys (set as needed)
OPENAI_API_KEY=your-openai-key-here
MISTRAL_API_KEY=your-mistral-key-here
GEMINI_API_KEY=your-gemini-key-here

# Custom Provider Endpoints (optional)
RSV_OPENAI_BASE_URL=https://api.openai.com/v1/chat/completions
RSV_OLLAMA_BASE_URL=http://localhost:11434/v1/chat/completions
RSV_MISTRAL_BASE_URL=https://api.mistral.ai/v1/chat/completions
```

### Using direnv (Recommended)

If you're using [direnv](https://direnv.net/), you can create a `.envrc` file:

```bash
# .envrc
export NEO4J_URI=bolt://localhost:7687
export NEO4J_USERNAME=neo4j
export NEO4J_PASSWORD=password
export RESERVOIR_PORT=3017
export OPENAI_API_KEY=your-openai-key-here
```

Then activate it:
```bash
direnv allow
```

## Verification

### 1. Check Neo4j Connection

Make sure Neo4j is running and accessible:

```bash
# If using Docker
docker ps | grep neo4j

# Test connection (replace with your credentials)
curl -u neo4j:password http://localhost:7474/db/data/
```

### 2. Start Reservoir

```bash
# From the repository directory
cargo run -- start

# Or if you installed the binary
reservoir start
```

You should see output similar to:
```
2024-01-01T12:00:00Z [INFO] Initializing vector index in Neo4j...
2024-01-01T12:00:01Z [INFO] Server starting on http://127.0.0.1:3017
```

### 3. Test the Installation

Run the included tests to verify everything is working:

```bash
# Test all endpoints
./hurl/test.sh

# Or test individual endpoints
hurl --variable USER="$USER" --variable OPENAI_API_KEY="$OPENAI_API_KEY" hurl/chat_completion.hurl
```

### 4. Simple API Test

Test with a basic curl request:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/test/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "Hello, Reservoir!"
            }
        ]
    }'
```

## Troubleshooting Installation

### Common Issues

**Neo4j Connection Failed**
- Verify Neo4j is running: `docker ps` or check your local Neo4j service
- Check credentials in your environment variables
- Ensure ports 7474 and 7687 are not blocked

**Cargo Build Fails**
- Update Rust: `rustup update`
- Clear cargo cache: `cargo clean`
- Check for system dependency issues

**Port Already in Use**
- Change the port: `export RESERVOIR_PORT=3018`
- Kill existing processes: `lsof -ti:3017 | xargs kill`

**API Key Issues**
- Verify your API keys are set correctly: `echo $OPENAI_API_KEY`
- Check for extra whitespace or quotes in environment variables

### Getting Help

If you encounter issues:

1. Check the [Troubleshooting](../troubleshooting/common-issues.md) section
2. Review the server logs for detailed error messages
3. Verify all prerequisites are properly installed
4. Test with the simplest possible configuration first

## Next Steps

Once Reservoir is installed and running:

1. Follow the [Getting Started](./quick-start.md) guide
2. Try the [Chat Gipitty Integration](./chat-gipitty.md)
3. Explore the [API Reference](./api/overview.md)
4. Check out [Usage Examples](./python-integration.md)