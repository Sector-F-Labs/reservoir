# Chat Completions Endpoint

The Chat Completions endpoint is Reservoir's core API, providing full OpenAI API compatibility with intelligent context enrichment. This endpoint automatically enhances your conversations with relevant historical context while maintaining the same request/response format as OpenAI's Chat Completions API.

## Endpoint URL

```
POST /v1/partition/{partition}/instance/{instance}/chat/completions
```

### URL Parameters

| Parameter | Description | Example |
|-----------|-------------|---------|
| `partition` | Top-level organization boundary | `alice`, `project_name`, `$USER` |
| `instance` | Specific context within partition | `coding`, `research`, `session_123` |

### Example URLs

```bash
# User-specific coding assistant
POST /v1/partition/alice/instance/coding/chat/completions

# Project-specific documentation bot  
POST /v1/partition/docs_project/instance/support/chat/completions

# Personal research assistant
POST /v1/partition/$USER/instance/research/chat/completions

# Default partition/instance (if not specified)
POST /v1/chat/completions  # Uses partition=default, instance=default
```

## Request Format

### Headers

```http
Content-Type: application/json
Authorization: Bearer YOUR_API_KEY
```

### Request Body

Reservoir accepts the standard OpenAI Chat Completions request format:

```json
{
  "model": "gpt-4",
  "messages": [
    {
      "role": "user", 
      "content": "How do I implement error handling in async functions?"
    }
  ]
}
```

### Supported Models

**OpenAI Models:**
- `gpt-4.1`
- `gpt-4-turbo` 
- `gpt-4o`
- `gpt-4o-mini`
- `gpt-3.5-turbo`
- `gpt-4o-search-preview`

**Local Models (via Ollama):**
- `llama3.1:8b`
- `llama3.1:70b`
- `mistral:7b`
- `codellama:latest`
- Any Ollama-supported model

### Message Roles

| Role | Description | Usage |
|------|-------------|-------|
| `user` | User input messages | Questions, requests, instructions |
| `assistant` | AI responses | Previous AI responses in conversation |
| `system` | System instructions | Behavior modification, context setting |

## Context Enrichment Process

When you send a request, Reservoir automatically enhances it with relevant context:

### 1. Message Analysis
```json
// Your original request
{
  "model": "gpt-4",
  "messages": [
    {
      "role": "user",
      "content": "How do I handle database timeouts?"
    }
  ]
}
```

### 2. Context Discovery
Reservoir finds relevant context through:

- **Semantic Search**: Messages similar to "database timeouts"
- **Recent History**: Last 15 messages from same partition/instance
- **Synapse Connections**: Related discussions via SYNAPSE relationships

### 3. Context Injection
```json
// Enriched request sent to AI model
{
  "model": "gpt-4", 
  "messages": [
    {
      "role": "system",
      "content": "The following is the result of a semantic search of the most related messages by cosine similarity to previous conversations"
    },
    {
      "role": "user",
      "content": "What's the best way to configure database connection pools?"
    },
    {
      "role": "assistant", 
      "content": "For database connection pools, consider these settings..."
    },
    {
      "role": "system",
      "content": "The following are the most recent messages in the conversation in chronological order"
    },
    {
      "role": "user",
      "content": "I'm working on optimizing database queries"
    },
    {
      "role": "assistant",
      "content": "Here are some query optimization techniques..."
    },
    {
      "role": "user",
      "content": "How do I handle database timeouts?"  // Your original message
    }
  ]
}
```

## Response Format

Reservoir returns responses in the standard OpenAI Chat Completions format:

```json
{
  "id": "chatcmpl-abc123",
  "object": "chat.completion", 
  "created": 1677858242,
  "model": "gpt-4",
  "usage": {
    "prompt_tokens": 13,
    "completion_tokens": 7,
    "total_tokens": 20
  },
  "choices": [
    {
      "message": {
        "role": "assistant",
        "content": "To handle database timeouts, you should implement retry logic with exponential backoff..."
      },
      "finish_reason": "stop",
      "index": 0
    }
  ]
}
```

## Configuration and Model Selection

### Environment Variables

Configure different AI providers:

```bash
# OpenAI (default)
export OPENAI_API_KEY="your-openai-api-key"
export RSV_OPENAI_BASE_URL="https://api.openai.com/v1/chat/completions"

# Ollama (local)
export RSV_OLLAMA_BASE_URL="http://localhost:11434/v1/chat/completions"

# Mistral
export MISTRAL_API_KEY="your-mistral-api-key"
export RSV_MISTRAL_BASE_URL="https://api.mistral.ai/v1/chat/completions"

# Gemini
export GEMINI_API_KEY="your-gemini-api-key"
```

### Model Detection

Reservoir automatically routes requests based on model name:

- **OpenAI models**: `gpt-*` → OpenAI API
- **Local models**: `llama*`, `mistral*`, etc. → Ollama API  
- **Mistral models**: `mistral-*` → Mistral API

## Error Handling

### Token Limit Errors

If your message exceeds model token limits:

```json
{
  "choices": [
    {
      "message": {
        "role": "assistant",
        "content": "Your last message is too long. It contains approximately 5000 tokens, which exceeds the maximum limit of 4096. Please shorten your message."
      },
      "finish_reason": "length",
      "index": 0
    }
  ]
}
```

### API Connection Errors

```json
{
  "error": {
    "message": "Failed to connect to OpenAI API: Connection timeout. Check your API key and network connection. Using model 'gpt-4' at 'https://api.openai.com/v1/chat/completions'"
  }
}
```

### Invalid Model Errors

```json
{
  "error": {
    "message": "Invalid OpenAI model name: 'gpt-5'. Valid models are: ['gpt-4.1', 'gpt-4-turbo', 'gpt-4o', 'gpt-4o-mini', 'gpt-3.5-turbo', 'gpt-4o-search-preview']"
  }
}
```

## Usage Examples

### Basic Request

```bash
curl -X POST "http://localhost:3017/v1/partition/alice/instance/coding/chat/completions" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-4",
    "messages": [
      {
        "role": "user",
        "content": "Explain async/await in Python"
      }
    ]
  }'
```

### With System Message

```bash
curl -X POST "http://localhost:3017/v1/partition/docs/instance/writing/chat/completions" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-4",
    "messages": [
      {
        "role": "system",
        "content": "You are a technical documentation expert. Provide clear, concise explanations."
      },
      {
        "role": "user",
        "content": "How should I document API endpoints?"
      }
    ]
  }'
```

### Local Model (Ollama)

```bash
curl -X POST "http://localhost:3017/v1/partition/alice/instance/local/chat/completions" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "llama3.1:8b",
    "messages": [
      {
        "role": "user",
        "content": "What are the benefits of using local LLMs?"
      }
    ]
  }'
```

## Integration Examples

### Python with OpenAI Library

```python
import openai

# Configure to use Reservoir instead of OpenAI directly
openai.api_base = "http://localhost:3017/v1/partition/alice/instance/coding"

response = openai.ChatCompletion.create(
    model="gpt-4",
    messages=[
        {"role": "user", "content": "How do I optimize this database query?"}
    ]
)

print(response.choices[0].message.content)
```

### JavaScript/Node.js

```javascript
const OpenAI = require('openai');

const openai = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
  baseURL: 'http://localhost:3017/v1/partition/myapp/instance/support'
});

async function chat(message) {
  const completion = await openai.chat.completions.create({
    messages: [{ role: 'user', content: message }],
    model: 'gpt-4',
  });

  return completion.choices[0].message.content;
}
```

### Streaming Responses

Reservoir supports streaming responses when the underlying model supports it:

```python
import openai

openai.api_base = "http://localhost:3017/v1/partition/alice/instance/chat"

response = openai.ChatCompletion.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Explain machine learning"}],
    stream=True
)

for chunk in response:
    print(chunk.choices[0].delta.content, end="")
```

## Advanced Features

### Web Search Integration

Some models support web search capabilities:

```json
{
  "model": "gpt-4o-search-preview",
  "messages": [
    {
      "role": "user",
      "content": "What are the latest developments in AI?"
    }
  ],
  "web_search_options": {
    "enabled": true
  }
}
```

### Message Storage

All messages (user and assistant) are automatically stored with:

- **Embeddings**: For semantic search and context enrichment
- **Timestamps**: For chronological ordering
- **Partition/Instance**: For data organization
- **Trace IDs**: For linking request/response pairs

### Context Control

Control context enrichment via configuration:

```bash
# Adjust context size
reservoir config --set semantic_context_size=20
reservoir config --set recent_context_size=15

# View current settings
reservoir config --get semantic_context_size
```

## Performance Considerations

### Token Management

- Reservoir automatically manages token limits for each model
- Context is intelligently truncated when necessary
- Priority given to most relevant and recent content

### Caching

- Embeddings are cached to avoid recomputation
- Vector indices are optimized for fast similarity search
- Connection pooling for database efficiency

### Latency

- Typical latency: 200-500ms for context enrichment
- Parallel processing of semantic search and recent history
- Optimized Neo4j queries for fast retrieval

The Chat Completions endpoint provides the full power of Reservoir's context enrichment while maintaining complete compatibility with existing OpenAI-based applications, making it easy to add conversational memory to any AI application.
