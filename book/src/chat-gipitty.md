# Chat Gipitty Integration

Reservoir was originally designed as a memory system for [Chat Gipitty](https://github.com/divanvisagie/chat-gipitty). This integration gives your cgip conversations persistent memory, context awareness, and the ability to search through your LLM interaction history.

## What You Get

When you integrate Reservoir with Chat Gipitty, you get:
- **Persistent Memory**: Your conversations are remembered across sessions
- **Semantic Search**: Find relevant past discussions automatically
- **Context Enrichment**: Each response is informed by your conversation history
- **Multi-Model Support**: Switch between different LLM providers while maintaining context

## Setup

### Prerequisites

- [Chat Gipitty](https://github.com/divanvisagie/chat-gipitty) installed and working
- Reservoir installed and running (see [Installation](./installation.md))
- Your shell configured (bash or zsh)

### Installation

Add this function to your `~/.bashrc` or `~/.zshrc` file:

```bash
function contextual_cgip_with_ingest() {
    local user_query="$1"

    # Validate input
    if [ -z "$user_query" ]; then
        echo "Usage: contextual_cgip_with_ingest 'Your question goes here'" >&2
        return 1
    fi

    # Ingest the user's query into Reservoir
    echo "$user_query" | reservoir ingest

    # Generate dynamic system prompt with context
    local system_prompt_content=$(
        echo "the following is info from semantic search based on your query:"
        reservoir search "$user_query" --semantic --link
        echo "the following is recent history:"
        reservoir view 10
    )

    # Call cgip with enriched context
    local assistant_response=$(cgip "${user_query}" --system-prompt="${system_prompt_content}")
    
    # Store the assistant's response
    echo "$assistant_response" | reservoir ingest --role assistant

    # Display the response
    echo "$assistant_response"
}

# Create a convenient alias
alias gpty='contextual_cgip_with_ingest'
```

After adding this to your shell configuration, reload it:

```bash
# For bash
source ~/.bashrc

# For zsh
source ~/.zshrc
```

## Usage

### Basic Usage

Use the function directly:

```bash
contextual_cgip_with_ingest "Explain quantum computing in simple terms"
```

Or use the convenient alias:

```bash
gpty "What is machine learning?"
```

### Follow-up Questions

The magic happens with follow-up questions:

```bash
gpty "Explain neural networks"
# ... LLM responds with explanation ...

gpty "How do they relate to what we discussed about machine learning earlier?"
# ... LLM responds with context from the previous conversation ...
```

### Different Topics

Start a new topic, and Reservoir will find relevant context:

```bash
gpty "I'm learning Rust programming"
# ... later in a different session ...

gpty "Show me some advanced Rust patterns"
# Reservoir will remember you're learning Rust and provide appropriate context
```

## How It Works

Here's what happens when you use the integrated function:

1. **Query Ingestion**: Your question is stored in Reservoir
2. **Context Gathering**: Reservoir searches for:
   - Semantically similar past conversations
   - Recent conversation history
3. **Context Injection**: This context is provided to cgip as a system prompt
4. **Enhanced Response**: cgip responds with awareness of your history
5. **Response Storage**: The LLM's response is stored for future context

## Advanced Configuration

### Custom Search Parameters

You can modify the function to customize how context is gathered:

```bash
function contextual_cgip_with_ingest() {
    local user_query="$1"
    
    if [ -z "$user_query" ]; then
        echo "Usage: contextual_cgip_with_ingest 'Your question goes here'" >&2
        return 1
    fi

    echo "$user_query" | reservoir ingest

    # Customize these parameters
    local system_prompt_content=$(
        echo "=== Relevant Context ==="
        reservoir search "$user_query" --semantic --link --limit 5
        echo ""
        echo "=== Recent History ==="
        reservoir view 15 --partition "$USER" --instance "cgip"
    )

    local assistant_response=$(cgip "${user_query}" --system-prompt="${system_prompt_content}")
    
    echo "$assistant_response" | reservoir ingest --role assistant
    echo "$assistant_response"
}
```

### Partitioned Conversations

Organize your conversations by topic or project:

```bash
function gpty_work() {
    local user_query="$1"
    if [ -z "$user_query" ]; then
        echo "Usage: gpty_work 'Your work-related question'" >&2
        return 1
    fi

    echo "$user_query" | reservoir ingest --partition "$USER" --instance "work"
    
    local system_prompt_content=$(
        echo "Context from work conversations:"
        reservoir search "$user_query" --semantic --partition "$USER" --instance "work"
        echo "Recent work discussion:"
        reservoir view 10 --partition "$USER" --instance "work"
    )

    local assistant_response=$(cgip "${user_query}" --system-prompt="${system_prompt_content}")
    echo "$assistant_response" | reservoir ingest --role assistant --partition "$USER" --instance "work"
    echo "$assistant_response"
}

function gpty_personal() {
    # Similar function for personal conversations
    # ... implement similarly with --instance "personal"
}
```

### Model Selection

Use different models while maintaining context:

```bash
function gpty_creative() {
    local user_query="$1"
    echo "$user_query" | reservoir ingest
    
    local system_prompt_content=$(
        reservoir search "$user_query" --semantic --link
        reservoir view 5
    )

    # Use a creative model via cgip configuration
    local assistant_response=$(cgip "${user_query}" --system-prompt="${system_prompt_content}" --model gpt-4)
    
    echo "$assistant_response" | reservoir ingest --role assistant
    echo "$assistant_response"
}
```

## Benefits of This Integration

### Continuous Learning
- Your LLM assistant learns from every interaction
- Context builds up over time, making responses more personalized
- No need to re-explain your projects or preferences

### Cross-Session Memory
- Resume conversations from days or weeks ago
- Reference past decisions and discussions
- Build on previous explanations and examples

### Semantic Understanding
- Ask "What did we discuss about X?" and get relevant results
- Similar topics are automatically connected
- Context is found even if you use different wording

### Privacy
- All your conversation history stays local
- No data sent to external services beyond the LLM API calls
- You control your data completely

## Troubleshooting

### Function Not Found
Make sure you've sourced your shell configuration:
```bash
source ~/.bashrc  # or ~/.zshrc
```

### No Context Being Added
Check that Reservoir is running:
```bash
# Should show Reservoir process
ps aux | grep reservoir

# Start if not running
cargo run -- start
```

### Empty Search Results
Build up some conversation history first:
```bash
gpty "Tell me about artificial intelligence"
gpty "What are neural networks?"
gpty "How does machine learning work?"

# Now try a search
gpty "What did we discuss about AI?"
```

### Permission Issues
Make sure the function has access to reservoir commands:
```bash
# Test individual commands
echo "test" | reservoir ingest
reservoir view 5
reservoir search "test"
```

## Next Steps

- Explore [API Reference](./api/overview.md) to understand Reservoir's capabilities
- Learn about [Partitioning](./features/partitioning.md) to organize conversations
- Check out [Python Integration](./python-integration.md) for programmatic access
- See [Troubleshooting](./troubleshooting/common-issues.md) if you encounter issues

The Chat Gipitty integration transforms your LLM interactions from isolated conversations into a connected, searchable knowledge base that grows smarter with every interaction.