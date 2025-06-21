# Common Issues

This page covers the most common issues you might encounter when using Reservoir and how to solve them.

## Server Issues

### Server Not Starting

**Symptoms:**
- Cannot connect to `http://localhost:3017`
- Connection refused errors
- Server fails to start

**Solutions:**

#### Check Neo4j
Ensure Neo4j is running and accessible:
```bash
# Check if Neo4j is running
systemctl status neo4j  # Linux
brew services list | grep neo4j  # macOS

# Start Neo4j if not running
systemctl start neo4j  # Linux
brew services start neo4j  # macOS
```

#### Port Conflicts
Default port 3017 might be in use:
```bash
# Check what's using port 3017
lsof -i :3017

# Use a different port
RESERVOIR_PORT=3018 cargo run -- start
```

#### Environment Variables
If using direnv, make sure it's loaded:
```bash
# Check if direnv is working
direnv status

# Allow direnv for current directory
direnv allow
```

### Server Starts But Returns Errors

#### Check Server Logs
Look at the server output for detailed error messages:
```bash
# Start with verbose logging
RUST_LOG=debug cargo run -- start
```

#### Test Basic Connectivity
```bash
# Test if server is responding
curl http://localhost:3017/health

# If health endpoint doesn't exist, try a simple request
curl "http://localhost:3017/partition/test/instance/basic/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{"model": "gemma3", "messages": [{"role": "user", "content": "hello"}]}'
```

## API and Model Issues

### "Internal Server Error" Responses

**Symptoms:**
- HTTP 500 errors
- Generic error messages
- Requests failing unexpectedly

**Solutions:**

#### Verify API Keys
Check that your API keys are set correctly:
```bash
echo $OPENAI_API_KEY
echo $MISTRAL_API_KEY
echo $GEMINI_API_KEY
```

If not set:
```bash
export OPENAI_API_KEY="your-openai-key"
export MISTRAL_API_KEY="your-mistral-key"
export GEMINI_API_KEY="your-gemini-key"
```

#### Check Model Names
Ensure you're using supported model names:

| Model | Provider | API Key Required |
|-------|----------|------------------|
| `gpt-4`, `gpt-4o`, `gpt-4o-mini`, `gpt-3.5-turbo` | OpenAI | Yes (`OPENAI_API_KEY`) |
| `gpt-4o-search-preview` | OpenAI | Yes (`OPENAI_API_KEY`) |
| `llama3.2`, `gemma3`, or any custom name | Ollama | No |
| `mistral-large-2402` | Mistral | Yes (`MISTRAL_API_KEY`) |
| `gemini-2.0-flash`, `gemini-2.5-flash-preview-05-20` | Google | Yes (`GEMINI_API_KEY`) |

#### Verify Ollama (for local models)
If using Ollama models, verify Ollama is running:
```bash
# Check Ollama status
ollama list

# If not running, start it
ollama serve

# Test Ollama directly
curl http://localhost:11434/api/tags
```

### Deserialization Errors

**Symptoms:**
- JSON parsing errors
- "Failed to deserialize" messages
- Malformed request errors

**Solutions:**

#### Check JSON Format
Ensure your JSON request is properly formatted:
```bash
# Good format
curl "http://localhost:3017/partition/$USER/instance/test/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [
            {
                "role": "user",
                "content": "Hello"
            }
        ]
    }'
```

#### Content-Type Header
Always use the correct content type:
```bash
# Always include this header
-H "Content-Type: application/json"
```

#### Optional Fields
Remember that fields like `web_search_options` are optional and can be omitted:
```bash
# This is valid without web_search_options
{
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "Hello"}]
}
```

### Connection Issues

**Symptoms:**
- Timeout errors
- Network unreachable
- DNS resolution failures

**Solutions:**

#### Check Provider URLs
Verify that custom provider URLs are accessible:
```bash
# Test OpenAI endpoint
curl -I https://api.openai.com/v1/chat/completions

# Test custom endpoint (if configured)
curl -I $RSV_OPENAI_BASE_URL
```

#### Verify Internet Connectivity
For cloud providers, ensure internet connectivity:
```bash
# Test internet connection
ping google.com

# Test specific provider
ping api.openai.com
```

#### Check Firewall Settings
Ensure no firewall is blocking outbound requests:
```bash
# Check if ports are blocked
telnet api.openai.com 443
telnet localhost 11434  # For Ollama
```

## Database Issues

### Neo4j Connection Problems

**Symptoms:**
- "Failed to connect to Neo4j" errors
- Database timeout errors
- Authentication failures

**Solutions:**

#### Check Neo4j Status
```bash
# Check if Neo4j is running
systemctl status neo4j  # Linux
brew services list | grep neo4j  # macOS

# Check Neo4j logs
journalctl -u neo4j  # Linux
tail -f /usr/local/var/log/neo4j/neo4j.log  # macOS
```

#### Verify Connection Details
Check your Neo4j connection settings:
```bash
# Default connection
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=your-password
```

#### Test Neo4j Directly
```bash
# Test with cypher-shell
cypher-shell -a bolt://localhost:7687 -u neo4j -p your-password

# Or use Neo4j Browser
# Navigate to http://localhost:7474
```

### Vector Index Issues

**Symptoms:**
- Slow semantic search
- "Index not found" errors
- Context enrichment not working

**Solutions:**

#### Recreate Vector Index
```bash
# Stop Reservoir
# Connect to Neo4j and run:
DROP INDEX embedding_index IF EXISTS;
CREATE VECTOR INDEX embedding_index FOR (n:EmbeddingNode) ON (n.embedding) OPTIONS {indexConfig: {`vector.dimensions`: 1536, `vector.similarity_function`: 'cosine'}};
```

#### Check Index Status
```cypher
SHOW INDEXES;
```

## Memory and Performance Issues

### High Memory Usage

**Symptoms:**
- System running out of memory
- Slow responses
- Process killed by system

**Solutions:**

#### Monitor Resource Usage
```bash
# Check Reservoir process
ps aux | grep reservoir

# Monitor system resources
htop
# or
top
```

#### Use Smaller Models
Switch to smaller models if using Ollama:
```bash
# Instead of large models, use smaller ones
ollama pull gemma3:2b  # 2B parameters instead of 7B
```

#### Limit Conversation History
The system automatically manages token limits, but you can monitor:
```bash
# View recent conversations to check size
cargo run -- view 10 --partition $USER --instance your-instance
```

### Slow Responses

**Symptoms:**
- Long wait times for responses
- Timeouts
- Poor performance

**Solutions:**

#### Check Model Performance
Different models have different performance characteristics:
- **Fastest**: Smaller Ollama models (2B-7B parameters)
- **Medium**: Cloud models like GPT-3.5-turbo
- **Slowest**: Large local models (13B+ parameters)

#### Optimize Ollama
```bash
# Use GPU acceleration if available
ollama run gemma3 --gpu

# Check Ollama performance
ollama ps
```

#### Network Optimization
For cloud models:
```bash
# Test network speed to provider
curl -w "@curl-format.txt" -o /dev/null -s "https://api.openai.com/v1/models"
```

## Testing and Debugging

### Systematic Troubleshooting

#### Step 1: Test Basic Setup
```bash
# Test Reservoir is running
curl http://localhost:3017/health

# Test with simplest possible request
curl "http://localhost:3017/partition/test/instance/debug/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{"model": "gemma3", "messages": [{"role": "user", "content": "hi"}]}'
```

#### Step 2: Test with Different Models
```bash
# Test Ollama model (no API key)
curl "http://localhost:3017/partition/test/instance/debug/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{"model": "gemma3", "messages": [{"role": "user", "content": "test"}]}'

# Test OpenAI model (requires API key)
curl "http://localhost:3017/partition/test/instance/debug/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "test"}]}'
```

#### Step 3: Check Logs
```bash
# Run with debug logging
RUST_LOG=debug cargo run -- start

# Check for specific error patterns
grep -i error reservoir.log
grep -i "failed" reservoir.log
```

### Using the Included Tests

Reservoir includes hurl tests that you can use to verify your setup:

```bash
# Test all endpoints
./hurl/test.sh

# Test specific endpoints
hurl --variable USER="$USER" --variable OPENAI_API_KEY="$OPENAI_API_KEY" hurl/chat_completion.hurl
hurl --variable USER="$USER" hurl/reservoir-view.hurl
hurl --variable USER="$USER" hurl/reservoir-search.hurl

# Test Ollama mode
hurl hurl/ollama_mode.hurl
```

## Getting Help

If you encounter issues not covered here:

1. **Check the server logs** for detailed error messages
2. **Verify your environment variables** are set correctly
3. **Test with a simple curl request** first
4. **Try the included hurl tests** to isolate the problem
5. **Check the [FAQ](./faq.md)** for additional solutions
6. **Review the [debugging guide](./debugging.md)** for advanced troubleshooting

## Environment Variable Reference

For quick reference, here are the key environment variables:

```bash
# Provider endpoints
RSV_OPENAI_BASE_URL="https://api.openai.com/v1/chat/completions"
RSV_OLLAMA_BASE_URL="http://localhost:11434/v1/chat/completions"
RSV_MISTRAL_BASE_URL="https://api.mistral.ai/v1/chat/completions"

# API keys
OPENAI_API_KEY="your-openai-key"
MISTRAL_API_KEY="your-mistral-key"
GEMINI_API_KEY="your-gemini-key"

# Reservoir settings
RESERVOIR_PORT="3017"

# Neo4j settings
NEO4J_URI="bolt://localhost:7687"
NEO4J_USER="neo4j"
NEO4J_PASSWORD="your-password"
```
