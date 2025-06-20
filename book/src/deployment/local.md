# Local Deployment

This guide covers setting up Reservoir for local development and production use on your local machine.

## Prerequisites

Before deploying Reservoir locally, ensure you have the following installed:

- **Rust** (latest stable version)
- **Docker** (for Neo4j database)
- **Git** for version control

## Quick Setup

### Step 1: Clone the Repository

```bash
git clone https://github.com/divanvisagie/reservoir.git
cd reservoir
```

### Step 2: Start Neo4j Database

You have several options for running Neo4j locally:

#### Option A: Docker Compose (Recommended)

```bash
docker-compose up -d
```

This starts Neo4j on the default `bolt://localhost:7687` with the credentials defined in the docker-compose file.

#### Option B: Docker Manual Setup

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

#### Option C: Homebrew (macOS Service)

If you prefer to run Neo4j as a permanent background service:

```bash
brew install neo4j
brew services start neo4j
```

This will start Neo4j on `bolt://localhost:7687` and ensure it runs automatically when your computer boots.

### Step 3: Configure Environment Variables

Create a `.env` file in the project root or export the following environment variables:

```env
# Server Configuration
RESERVOIR_PORT=3017
RESERVOIR_HOST=127.0.0.1

# Database Configuration
NEO4J_URI=bolt://localhost:7687
NEO4J_USERNAME=neo4j
NEO4J_PASSWORD=password

# API Keys (required for respective providers)
OPENAI_API_KEY=sk-your-openai-key-here
MISTRAL_API_KEY=your-mistral-key-here
GEMINI_API_KEY=your-gemini-key-here

# Custom Provider URLs (optional)
RSV_OPENAI_BASE_URL=https://api.openai.com/v1/chat/completions
RSV_OLLAMA_BASE_URL=http://localhost:11434/v1/chat/completions
RSV_MISTRAL_BASE_URL=https://api.mistral.ai/v1/chat/completions
```

> **Note**: Most environment variables have sensible defaults. Only the API keys for your chosen providers are required.

### Step 4: Build and Run

#### Manual Execution

```bash
# Build the project
cargo build --release

# Run Reservoir
cargo run -- start
```

#### Using Make Commands

```bash
# Build the release binary
make main

# Run for development (with auto-reload)
make dev

# Run normally
make run
```

Reservoir will now be available at `http://localhost:3017`.

## Service Installation (macOS)

For a more permanent setup, you can install Reservoir as a macOS LaunchAgent service.

### Install the Service

```bash
make install-service
```

This command:
- Copies the LaunchAgent plist to `~/Library/LaunchAgents/`
- Loads the service using `launchctl`
- Starts Reservoir automatically in the background

### Service Management

**Check service status:**
```bash
launchctl list | grep reservoir
```

**View service logs:**
```bash
tail -f /tmp/reservoir.log
tail -f /tmp/reservoir.err
```

**Manually start/stop the service:**
```bash
# Start
launchctl start com.sectorflabs.reservoir

# Stop
launchctl stop com.sectorflabs.reservoir
```

### Uninstall the Service

```bash
make uninstall-service
```

This removes the service and cleans up all related files.

## Verification

### Test the Installation

1. **Check if Reservoir is running:**
   ```bash
   curl http://localhost:3017/health
   ```

2. **Test with a simple API call:**
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

3. **Run the test suite:**
   ```bash
   ./hurl/test.sh
   ```

### Check Neo4j Connection

Verify that Neo4j is accessible:

```bash
# Check Neo4j web interface
open http://localhost:7474

# Test connection with curl
curl -u neo4j:password http://localhost:7474/db/data/
```

## Configuration Options

### Database Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `NEO4J_URI` | `bolt://localhost:7687` | Neo4j connection URI |
| `NEO4J_USERNAME` | `neo4j` | Database username |
| `NEO4J_PASSWORD` | `password` | Database password |

### Server Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `RESERVOIR_PORT` | `3017` | HTTP server port |
| `RESERVOIR_HOST` | `127.0.0.1` | HTTP server host |

### Provider Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `RSV_OPENAI_BASE_URL` | `https://api.openai.com/v1/chat/completions` | OpenAI API endpoint |
| `RSV_OLLAMA_BASE_URL` | `http://localhost:11434/v1/chat/completions` | Ollama API endpoint |
| `RSV_MISTRAL_BASE_URL` | `https://api.mistral.ai/v1/chat/completions` | Mistral API endpoint |

## Troubleshooting

### Common Issues

**Port Already in Use:**
```bash
# Check what's using port 3017
lsof -i :3017

# Use a different port
export RESERVOIR_PORT=3018
```

**Neo4j Connection Failed:**
```bash
# Check if Neo4j is running
docker ps | grep neo4j

# Check Neo4j logs
docker logs neo4j
```

**Permission Issues (macOS Service):**
```bash
# Ensure the binary path is correct in the plist
ls -la ~/.cargo/bin/reservoir

# Update the path in scripts/com.sectorflabs.reservoir.plist if needed
```

**API Key Issues:**
```bash
# Verify your API key is set
echo $OPENAI_API_KEY

# Test the key directly with OpenAI
curl https://api.openai.com/v1/models \
    -H "Authorization: Bearer $OPENAI_API_KEY"
```

### Performance Tuning

For better performance in local deployment:

1. **Increase Neo4j memory allocation:**
   ```bash
   # In docker-compose.yml, add:
   NEO4J_dbms_memory_heap_initial__size=512m
   NEO4J_dbms_memory_heap_max__size=2G
   ```

2. **Use SSD storage for Neo4j data:**
   ```bash
   # Mount Neo4j data on fast storage
   -v /path/to/fast/storage:/data
   ```

3. **Optimize connection pooling:**
   ```env
   # Add to .env
   NEO4J_MAX_CONNECTIONS=20
   NEO4J_CONNECTION_TIMEOUT=30s
   ```

## Next Steps

After successful local deployment:

- [Configure Environment Variables](./environment.md)
- [Set up Production Deployment](./production.md)
- [Learn about API Usage](../api/overview.md)
- [Explore Chat Gipitty Integration](../chat-gipitty.md)

Your Reservoir instance is now ready for local development and testing!