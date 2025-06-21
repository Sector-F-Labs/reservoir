# Your First Chat

This guide will walk you through sending your first message through Reservoir and demonstrate how its memory and context features work.

## Prerequisites

Before starting, make sure you have:
- Reservoir server running (`cargo run -- start`)
- Neo4j database accessible
- API keys set up (if using cloud providers)

## Example 1: Testing with Ollama (Local)

Let's start with a local Ollama model since it doesn't require API keys:

### Step 1: Send your first message

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/first-chat/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [
            {
                "role": "user",
                "content": "Hello! My name is Alice and I love programming in Python."
            }
        ]
    }'
```

### Step 2: Ask a follow-up question

Now ask something that requires memory of the previous conversation:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/first-chat/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [
            {
                "role": "user",
                "content": "What programming language do I like?"
            }
        ]
    }'
```

**Magic!** The Language Model will remember that you like Python, even though you didn't include the previous conversation in your request. Reservoir handled that automatically!

### Step 3: Continue the conversation

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/first-chat/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [
            {
                "role": "user",
                "content": "Can you suggest a Python project for someone at my skill level?"
            }
        ]
    }'
```

The Language Model will make suggestions based on knowing you're Alice who loves Python programming!

## Example 2: Using OpenAI Models

If you have an OpenAI API key set up:

### Step 1: Introduction with GPT-4

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/gpt-chat/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "Hi! I am working on a machine learning project about image classification."
            }
        ]
    }'
```

### Step 2: Ask for specific help

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/gpt-chat/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "What neural network architecture would you recommend for my project?"
            }
        ]
    }'
```

GPT-4 will remember you're working on image classification and provide relevant recommendations!

## Example 3: Cross-Model Conversations

One of Reservoir's unique features is that conversation context can span multiple models:

### Step 1: Start with Ollama

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/cross-model/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [
            {
                "role": "user",
                "content": "I am learning about quantum computing basics."
            }
        ]
    }'
```

### Step 2: Switch to GPT-4

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/cross-model/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "Can you explain quantum superposition in more detail?"
            }
        ]
    }'
```

GPT-4 will know you're learning quantum computing and provide an explanation appropriate to your level!

## Understanding the Results

### What Reservoir Does Behind the Scenes

When you send a message, Reservoir:

1. **Stores your message** in Neo4j with embeddings
2. **Searches for relevant context** from previous conversations
3. **Injects relevant history** into your request automatically
4. **Forwards the enriched request** to the Language Model provider
5. **Stores the Language Model's response** for future context

### Viewing Your Conversation History

You can see your stored conversations using the CLI:

```bash
# View last 5 messages in the first-chat instance
cargo run -- view 5 --partition $USER --instance first-chat
```

Sample output:
```
2025-06-21T09:10:01+00:00 [abc123] user: Hello! My name is Alice and I love programming in Python.
2025-06-21T09:10:02+00:00 [abc123] assistant: Hello Alice! It's great to meet a fellow Python enthusiast...
2025-06-21T09:11:10+00:00 [def456] user: What programming language do I like?
2025-06-21T09:11:12+00:00 [def456] assistant: You mentioned that you love programming in Python!
2025-06-21T09:12:00+00:00 [ghi789] user: Can you suggest a Python project for someone at my skill level?
```

## Testing Different Scenarios

### Scenario 1: Different Partitions

Try organizing conversations by topic using different partitions:

```bash
# Work-related conversations
curl "http://127.0.0.1:3017/partition/work/instance/coding/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{"model": "gemma3", "messages": [{"role": "user", "content": "I need help debugging a React component."}]}'

# Personal learning
curl "http://127.0.0.1:3017/partition/personal/instance/learning/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{"model": "gemma3", "messages": [{"role": "user", "content": "I want to learn guitar playing."}]}'
```

Each partition maintains separate conversation history!

### Scenario 2: Web Search Integration

If using a model that supports web search:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/research/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4o-search-preview",
        "messages": [{"role": "user", "content": "What are the latest trends in AI development?"}],
        "web_search_options": {"enabled": true, "max_results": 5}
    }'
```

## Common Issues and Solutions

### Server Not Responding
```bash
# Check if Reservoir is running
curl http://127.0.0.1:3017/health

# If not running, start it
cargo run -- start
```

### "Model not found" Error
- For Ollama models: Make sure Ollama is running and the model is installed
- For cloud models: Check your API keys are set correctly

### Empty Responses
- Check your internet connection for cloud providers
- Verify the model name is spelled correctly
- Ensure your API key has sufficient credits

## Next Steps

Now that you've sent your first chat, explore these features:

- **[Python Integration](./python-integration.md)** - Use Reservoir from Python code
- **[Partitioning & Organization](./features/partitioning.md)** - Organize your conversations
- **[Chat Gipitty Integration](./chat-gipitty.md)** - Add memory to your existing chat tools
- **[API Reference](./api/overview.md)** - Learn about advanced features

**Congratulations!** You've successfully used Reservoir to have a conversation with persistent memory. The Language Model now remembers everything from your conversation and can reference it in future chats!
