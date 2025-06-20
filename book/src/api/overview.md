# API Overview

Reservoir provides an OpenAI-compatible API endpoint that acts as a smart proxy between your application and AI language models. This section covers the core API structure and basic usage patterns.

## URL Structure

The Reservoir API follows this pattern:

```
/v1/partition/{partition}/instance/{instance}/chat/completions
```

### Parameters

- **`{partition}`**: A broad category for organizing conversations (e.g., project name, application name, username)
- **`{instance}`**: A specific context within the partition (e.g., user ID, session ID, specific feature)

This structure allows you to organize conversations hierarchically and scope context enrichment appropriately.

### Example URL Transformation

- **Instead of**:
  ```
  https://api.openai.com/v1/chat/completions
  ```
- **Use**:
  ```
  http://localhost:3017/v1/partition/$USER/instance/my-application/chat/completions
  ```

Here, `$USER` is your system username, and `my-application` is your application instance. All context enrichment and history retrieval are scoped to this specific partition/instance combination.

## Basic Request Structure

Reservoir maintains full compatibility with the OpenAI Chat Completions API. You can use the same request structure, headers, and parameters you would use with OpenAI directly.

### Required Headers

```http
Content-Type: application/json
Authorization: Bearer YOUR_API_KEY
```

### Request Body

The request body follows the same format as OpenAI's Chat Completions API:

```json
{
    "model": "gpt-4",
    "messages": [
        {
            "role": "user",
            "content": "Your message here"
        }
    ]
}
```

## What Happens Behind the Scenes

When you make a request to Reservoir:

1. **Message Storage**: Your message is stored with the specified partition/instance
2. **Context Enrichment**: Reservoir finds relevant past conversations and recent history
3. **Token Management**: The enriched context is checked against token limits
4. **Request Forwarding**: The enriched request is forwarded to the appropriate AI provider
5. **Response Storage**: The AI's response is stored for future context

## Response Format

Responses maintain the same format as the underlying AI provider (OpenAI, Ollama, etc.), so your existing code will work without modification.

## Next Steps

- [Chat Completions Endpoint](./chat-completions.md) - Detailed endpoint documentation
- [Search & Retrieval](./search.md) - Finding past conversations
- [Data Management](./data-management.md) - Import/export and management
- [Command Line Interface](./cli.md) - CLI usage and commands