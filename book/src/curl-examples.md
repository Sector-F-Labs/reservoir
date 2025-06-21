# Curl Examples

This page provides comprehensive examples of using Reservoir with curl commands. These examples are perfect for testing, scripting, or understanding the API structure.

## Basic URL Structure

Instead of calling the provider directly, you call Reservoir with this URL pattern:

- **Direct Provider:** `https://api.openai.com/v1/chat/completions`
- **Through Reservoir:** `http://127.0.0.1:3017/partition/$USER/instance/reservoir/v1/chat/completions`

Where:
- `$USER` is your system username (acts as the partition)
- `reservoir` is the instance name (you can use any name)

## OpenAI Models

### Basic GPT-4 Example

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/reservoir/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "Write a one-sentence bedtime story about a brave little toaster."
            }
        ]
    }'
```

### GPT-4 with System Message

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/reservoir/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant that explains complex topics in simple terms."
            },
            {
                "role": "user",
                "content": "Explain quantum computing to a 10-year-old."
            }
        ]
    }'
```

### Web Search Integration

For models that support web search (like `gpt-4o-search-preview`):

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/reservoir/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4o-search-preview",
        "messages": [
            {
                "role": "user",
                "content": "What are the latest developments in AI?"
            }
        ],
        "web_search_options": {
            "enabled": true,
            "max_results": 5
        }
    }'
```

## Ollama Models (Local)

### Basic Ollama Example

No API key needed for Ollama models:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/reservoir/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [
            {
                "role": "user",
                "content": "Explain quantum computing in simple terms."
            }
        ]
    }'
```

### Using Llama Models

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/reservoir/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "llama3.2",
        "messages": [
            {
                "role": "user",
                "content": "Write a Python function to calculate fibonacci numbers."
            }
        ]
    }'
```

## Other Providers

### Mistral AI

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/reservoir/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $MISTRAL_API_KEY" \
    -d '{
        "model": "mistral-large-2402",
        "messages": [
            {
                "role": "user",
                "content": "Explain the differences between functional and object-oriented programming."
            }
        ]
    }'
```

### Google Gemini

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/reservoir/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $GEMINI_API_KEY" \
    -d '{
        "model": "gemini-2.0-flash",
        "messages": [
            {
                "role": "user",
                "content": "Compare different sorting algorithms and their time complexities."
            }
        ]
    }'
```

## Partitioning Examples

### Using Different Partitions

You can organize conversations by using different partition names:

```bash
# Work conversations
curl "http://127.0.0.1:3017/partition/work/instance/coding/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [{"role": "user", "content": "Review this code for security issues"}]
    }'

# Personal conversations
curl "http://127.0.0.1:3017/partition/personal/instance/creative/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [{"role": "user", "content": "Help me write a short story"}]
    }'
```

### Using Different Instances

Different instances within the same partition:

```bash
# Development instance
curl "http://127.0.0.1:3017/partition/$USER/instance/development/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [{"role": "user", "content": "Debug this Python error"}]
    }'

# Research instance
curl "http://127.0.0.1:3017/partition/$USER/instance/research/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [{"role": "user", "content": "Explain machine learning concepts"}]
    }'
```

## Testing Scenarios

### Test Basic Connectivity

```bash
# Simple test with Ollama (no API key needed)
curl "http://127.0.0.1:3017/partition/test/instance/basic/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [{"role": "user", "content": "Hello, can you hear me?"}]
    }'
```

### Test Memory Functionality

Send multiple requests to see memory in action:

```bash
# First message
curl "http://127.0.0.1:3017/partition/test/instance/memory/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [{"role": "user", "content": "My favorite color is blue."}]
    }'

# Second message - should remember the color
curl "http://127.0.0.1:3017/partition/test/instance/memory/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3", 
        "messages": [{"role": "user", "content": "What is my favorite color?"}]
    }'
```

## Error Handling

### Invalid Model

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/test/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "nonexistent-model",
        "messages": [{"role": "user", "content": "Hello"}]
    }'
```

### Missing API Key

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/test/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gpt-4",
        "messages": [{"role": "user", "content": "Hello"}]
    }'
# Will return error because OPENAI_API_KEY is required for GPT-4
```

## Environment Variables

Set up your environment for easier testing:

```bash
export OPENAI_API_KEY="your-openai-key"
export MISTRAL_API_KEY="your-mistral-key"
export GEMINI_API_KEY="your-gemini-key"
export RESERVOIR_URL="http://127.0.0.1:3017"
export USER_PARTITION="$USER"
```

Then use in requests:

```bash
curl "$RESERVOIR_URL/partition/$USER_PARTITION/instance/test/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [{"role": "user", "content": "Hello from the environment!"}]
    }'
```

## Debugging Tips

### Pretty Print JSON Response

Add `| jq` to format the JSON response:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/test/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [{"role": "user", "content": "Hello"}]
    }' | jq
```

### Verbose Output

Use `-v` flag to see request/response headers:

```bash
curl -v "http://127.0.0.1:3017/partition/$USER/instance/test/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [{"role": "user", "content": "Hello"}]
    }'
```

### Save Response

Save the response to a file:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/test/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [{"role": "user", "content": "Hello"}]
    }' -o response.json
```

## Next Steps

- Learn about [API Reference](./api/overview.md) for more endpoint details
- Check out [Python Integration](./python-integration.md) for programmatic usage
- Explore [Partitioning & Organization](./features/partitioning.md) to organize your conversations
