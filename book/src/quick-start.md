# Quick Start

This guide will get you up and running with Reservoir in just a few minutes.

## Before You Begin

Make sure you have:
- Reservoir installed (see [Installation](./installation.md))
- Neo4j running locally
- At least one API key configured (OpenAI, Mistral, or Gemini)

## Step 1: Start the Server

Open a terminal and start Reservoir:

```bash
cargo run -- start
```

You should see:
```
[INFO] Initializing vector index in Neo4j for semantic search
[INFO] Server starting on http://127.0.0.1:3017
```

Keep this terminal open - Reservoir is now running and ready to handle requests.

## Step 2: Your First Chat Request

Open a new terminal and send your first chat request:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/quickstart/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "Hello! What is Reservoir?"
            }
        ]
    }'
```

The response will look like a standard OpenAI API response, but Reservoir has:
- Stored your message and the AI's response
- Tagged them with your username and "quickstart" instance
- Made them available for future context enrichment

## Step 3: See the Memory in Action

Send a follow-up question that references your previous conversation:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/quickstart/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "Can you elaborate on what you just told me?"
            }
        ]
    }'
```

Notice how the AI understands "what you just told me" - that's Reservoir automatically injecting the previous conversation context!

## Step 4: View Your Conversation History

Check what Reservoir has stored:

```bash
cargo run -- view 5 --partition "$USER" --instance quickstart
```

You'll see output like:
```
2024-01-01T12:00:00+00:00 [abc123] user: Hello! What is Reservoir?
2024-01-01T12:00:01+00:00 [abc123] assistant: Reservoir is a memory system for AI conversations...
2024-01-01T12:01:00+00:00 [def456] user: Can you elaborate on what you just told me?
2024-01-01T12:01:01+00:00 [def456] assistant: Certainly! Let me expand on Reservoir's capabilities...
```

## Step 5: Try Different Models

Reservoir supports multiple providers. Try Ollama (no API key needed):

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/quickstart/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "llama3.2",
        "messages": [
            {
                "role": "user",
                "content": "What did we discuss earlier about Reservoir?"
            }
        ]
    }'
```

Even though you're using a different model (Ollama instead of OpenAI), Reservoir still provides the conversation context!

## Understanding the URL Structure

The Reservoir API endpoint follows this pattern:

```
http://localhost:3017/partition/{partition}/instance/{instance}/v1/chat/completions
```

- **Partition**: Organizes conversations (typically your username)
- **Instance**: Sub-organizes within a partition (like "quickstart", "work", "personal")
- This keeps different contexts separate while allowing context sharing within each space

## What Just Happened?

1. **Storage**: Every message (yours and the AI's) was stored in Neo4j
2. **Context Enrichment**: Reservoir automatically found relevant past messages and included them in requests
3. **Multi-Provider**: You used both OpenAI and Ollama with the same conversation history
4. **Organization**: Your conversations were organized by partition and instance

## Next Steps

Now that you've seen Reservoir in action, explore:

- [Chat Gipitty Integration](./chat-gipitty.md) - Add memory to your existing cgip setup
- [Python Integration](./python-integration.md) - Use with the OpenAI Python library
- [API Reference](./api/overview.md) - Detailed API documentation
- [Features](./features/providers.md) - Learn about advanced features

## Quick Reference

### Common Commands

```bash
# Start the server
cargo run -- start

# View recent messages
cargo run -- view 10 --partition $USER --instance myapp

# Export conversations
cargo run -- export > backup.json

# Import conversations
cargo run -- import backup.json

# Search conversations
cargo run -- search "your query" --partition $USER
```

### Environment Variables

```bash
export RESERVOIR_PORT=3017                    # Server port
export NEO4J_URI=bolt://localhost:7687        # Neo4j connection
export OPENAI_API_KEY=your-key-here          # OpenAI API key
export MISTRAL_API_KEY=your-key-here         # Mistral API key
```

Ready to dive deeper? Check out the [Usage Examples](./python-integration.md) or learn about [Chat Gipitty Integration](./chat-gipitty.md)!