# Frequently Asked Questions

This section addresses common questions and issues you might encounter while using Reservoir.

## General Questions

### What is Reservoir?
Reservoir is a memory system for AI conversations that acts as a smart proxy between your applications and OpenAI-compatible APIs. It automatically stores conversation history and enriches new requests with relevant context from past conversations.

### Does Reservoir support streaming responses?
No, streaming responses are not currently supported. All requests are handled in a non-streaming manner. The response is returned once the complete message is received from the AI provider.

### Can I use Reservoir with clients other than the OpenAI Python library?
Yes, Reservoir is designed to be fully OpenAI-compatible. It has been tested with:
- `curl` command line tool
- OpenAI Python library
- Chat Gipitty
- Any application that can make HTTP requests to OpenAI-compatible endpoints

However, compatibility with some specialized clients may vary. If you encounter issues with a specific client, please report it as an issue.

### What AI providers does Reservoir support?
Reservoir supports multiple AI providers:
- **OpenAI**: GPT-4, GPT-4o, GPT-3.5-turbo, and specialized models
- **Ollama**: Local models like Llama, Gemma, and any custom models
- **Mistral AI**: Cloud-hosted Mistral models
- **Google Gemini**: Google's AI models
- **Custom providers**: Any OpenAI-compatible API endpoint

### How does Reservoir organize conversations?
Reservoir uses a two-level organization system:
- **Partition**: Top-level grouping (typically your username)
- **Instance**: Application-specific context within a partition

This allows you to keep conversations from different applications separate while maintaining context within each application.

### Is my data private?
Yes, absolutely. All conversation data is stored locally in your Neo4j database and never leaves your infrastructure. Reservoir only forwards your requests to the AI providers you choose to use.

## Technical Questions

### What database does Reservoir use?
Reservoir uses Neo4j as its graph database. Neo4j provides:
- Vector similarity search for semantic matching
- Graph relationships for conversation threading
- Efficient querying for context enrichment
- Scalable storage for large conversation histories

### How does context enrichment work?
When you send a message, Reservoir:
1. Stores your message in the database
2. Searches for semantically similar past messages
3. Retrieves recent conversation history
4. Injects relevant context into your request
5. Sends the enriched request to the AI provider
6. Stores the response for future context

### What are the token limits?
Reservoir respects the token limits of the underlying AI models:
- **GPT-4**: 8,192 tokens (context window)
- **GPT-4-32k**: 32,768 tokens
- **GPT-3.5-turbo**: 4,096 tokens
- **Local models**: Varies by model

Reservoir automatically truncates context to fit within these limits while preserving system prompts and your latest message.

### Can I run multiple Reservoir instances?
Yes, you can run multiple instances by:
- Using different ports (`RESERVOIR_PORT`)
- Using different Neo4j databases
- Using different partition/instance combinations

## Troubleshooting

### Neo4j Connection Issues

**Problem**: Unable to connect to Neo4j.

**Solutions**:
1. Ensure Neo4j is running:
   ```bash
   docker ps | grep neo4j
   ```

2. Check your connection details in `.env`:
   ```env
   NEO4J_URI=bolt://localhost:7687
   NEO4J_USERNAME=neo4j
   NEO4J_PASSWORD=password
   ```

3. Test the connection manually:
   ```bash
   curl -u neo4j:password http://localhost:7474/db/data/
   ```

### OpenAI API Key Issues

**Problem**: Requests fail due to missing or invalid API key.

**Solutions**:
1. Verify your API key is set:
   ```bash
   echo $OPENAI_API_KEY
   ```

2. Test the key directly with OpenAI:
   ```bash
   curl https://api.openai.com/v1/models \
       -H "Authorization: Bearer $OPENAI_API_KEY"
   ```

3. Ensure there are no extra spaces or quotes in your environment variable.

### Token Limit Errors

**Problem**: Requests fail due to exceeding the token limit.

**Solutions**:
1. Reduce the size of your input message
2. Clear old conversation history for the partition/instance
3. Use a model with a larger context window (e.g., GPT-4-32k)
4. Check if context enrichment is adding too much historical data

### Port Already in Use

**Problem**: Reservoir fails to start because port 3017 is already in use.

**Solutions**:
1. Check what's using the port:
   ```bash
   lsof -i :3017
   ```

2. Use a different port:
   ```bash
   export RESERVOIR_PORT=3018
   ```

3. Kill the process using the port (if safe to do so):
   ```bash
   kill -9 $(lsof -ti:3017)
   ```

### Permission Denied (macOS Service)

**Problem**: Service fails to start due to permission issues.

**Solutions**:
1. Check the binary path in the plist file:
   ```bash
   cat ~/Library/LaunchAgents/com.sectorflabs.reservoir.plist
   ```

2. Ensure the binary exists and is executable:
   ```bash
   ls -la ~/.cargo/bin/reservoir
   ```

3. Update the path in the plist if necessary

### Slow Performance

**Problem**: Reservoir responses are slow.

**Solutions**:
1. Check Neo4j memory allocation
2. Ensure Neo4j data is on fast storage (SSD)
3. Optimize vector index settings
4. Reduce the number of context messages retrieved
5. Check network connectivity to AI providers

## Installation Questions

### Do I need to install Neo4j separately?
No, the recommended approach is to use Docker Compose, which automatically sets up Neo4j for you:
```bash
docker-compose up -d
```

### Can I use an existing Neo4j instance?
Yes, you can connect to any Neo4j instance by setting the appropriate environment variables:
```env
NEO4J_URI=bolt://your-neo4j-host:7687
NEO4J_USERNAME=your-username
NEO4J_PASSWORD=your-password
```

### What Rust version do I need?
Reservoir requires the latest stable version of Rust. You can install it with:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Integration Questions

### How do I integrate with Chat Gipitty?
See the dedicated [Chat Gipitty Integration](../chat-gipitty.md) guide for detailed setup instructions.

### Can I use Reservoir with my existing Python scripts?
Yes, simply change the base URL in your OpenAI client:
```python
from openai import OpenAI

client = OpenAI(
    base_url="http://localhost:3017/v1/partition/myuser/instance/myapp",
    api_key=os.environ.get("OPENAI_API_KEY")
)
```

### How do I migrate my existing conversation data?
Reservoir provides import/export functionality:
```bash
# Export from another system (if supported)
reservoir export > conversations.json

# Import into Reservoir
reservoir import conversations.json
```

## Advanced Usage

### Can I customize the similarity threshold for context matching?
Currently, the similarity threshold (0.85) is hardcoded, but this may become configurable in future versions.

### How do I backup my conversation data?
Use the export command to create backups:
```bash
reservoir export > backup-$(date +%Y%m%d).json
```

### Can I run Reservoir in production?
Reservoir is currently designed for local development use. For production deployment, consider:
- Securing the Neo4j database
- Setting up proper authentication
- Configuring appropriate firewall rules
- Using HTTPS for external access

## Getting Help

If your question isn't answered here:

1. Check the [Common Issues](./common-issues.md) section
2. Review the [API Documentation](../api/overview.md)
3. Look at existing GitHub issues
4. Create a new issue with details about your problem

Remember to include:
- Your operating system
- Rust version (`rustc --version`)
- Neo4j version
- Relevant log output
- Steps to reproduce the issue