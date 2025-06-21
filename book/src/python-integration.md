# Python Integration

Reservoir works seamlessly with the popular OpenAI Python library. You simply point the client to your Reservoir instance instead of directly to OpenAI, and Reservoir handles all the memory and context management automatically.

## Setup

First, install the OpenAI Python library if you haven't already:

```bash
pip install openai
```

## Basic Configuration

```python
import os
from openai import OpenAI

INSTANCE = "my-application"
PARTITION = os.getenv("USER")
RESERVOIR_PORT = os.getenv('RESERVOIR_PORT', '3017')
RESERVOIR_BASE_URL = f"http://localhost:{RESERVOIR_PORT}/v1/partition/{PARTITION}/instance/{INSTANCE}"
```

## OpenAI Models

### Basic Usage with OpenAI

```python
import os
from openai import OpenAI

INSTANCE = "my-application"
PARTITION = os.getenv("USER")
RESERVOIR_PORT = os.getenv('RESERVOIR_PORT', '3017')
RESERVOIR_BASE_URL = f"http://localhost:{RESERVOIR_PORT}/v1/partition/{PARTITION}/instance/{INSTANCE}"

client = OpenAI(
    base_url=RESERVOIR_BASE_URL,
    api_key=os.environ.get("OPENAI_API_KEY")
)

completion = client.chat.completions.create(
    model="gpt-4",
    messages=[
        {
            "role": "user",
            "content": "Write a one-sentence bedtime story about a curious robot."
        }
    ]
)
print(completion.choices[0].message.content)
```

### With Web Search Options

For models that support web search (like `gpt-4o-search-preview`), you can enable web search capabilities:

```python
completion = client.chat.completions.create(
    model="gpt-4o-search-preview",
    messages=[
        {
            "role": "user",
            "content": "What are the latest trends in machine learning?"
        }
    ],
    extra_body={
        "web_search_options": {
            "enabled": True,
            "max_results": 5
        }
    }
)
```

## Ollama Models (Local)

### Using Ollama (No API Key Required)

```python
import os
from openai import OpenAI

INSTANCE = "my-application"
PARTITION = os.getenv("USER")
RESERVOIR_PORT = os.getenv('RESERVOIR_PORT', '3017')
RESERVOIR_BASE_URL = f"http://localhost:{RESERVOIR_PORT}/v1/partition/{PARTITION}/instance/{INSTANCE}"

client = OpenAI(
    base_url=RESERVOIR_BASE_URL,
    api_key="not-needed-for-ollama"  # Ollama doesn't require API keys
)

completion = client.chat.completions.create(
    model="llama3.2",  # or "gemma3", or any Ollama model
    messages=[
        {
            "role": "user",
            "content": "Explain the concept of recursion with a simple example."
        }
    ]
)
print(completion.choices[0].message.content)
```

## Supported Models

Reservoir automatically routes requests to the appropriate provider based on the model name:

| Model | Provider | API Key Required |
|-------|----------|------------------|
| `gpt-4`, `gpt-4o`, `gpt-4o-mini`, `gpt-3.5-turbo` | OpenAI | Yes (`OPENAI_API_KEY`) |
| `gpt-4o-search-preview` | OpenAI | Yes (`OPENAI_API_KEY`) |
| `llama3.2`, `gemma3`, or any custom name | Ollama | No |
| `mistral-large-2402` | Mistral | Yes (`MISTRAL_API_KEY`) |
| `gemini-2.0-flash`, `gemini-2.5-flash-preview-05-20` | Google | Yes (`GEMINI_API_KEY`) |

**Note:** Any model name not explicitly configured will default to using Ollama.

## Environment Variables

You can customize provider endpoints and set API keys using environment variables:

```python
import os

# Set environment variables (or use .env file)
os.environ['OPENAI_API_KEY'] = 'your-openai-key'
os.environ['MISTRAL_API_KEY'] = 'your-mistral-key'
os.environ['GEMINI_API_KEY'] = 'your-gemini-key'

# Custom provider endpoints (optional)
os.environ['RSV_OPENAI_BASE_URL'] = 'https://api.openai.com/v1/chat/completions'
os.environ['RSV_OLLAMA_BASE_URL'] = 'http://localhost:11434/v1/chat/completions'
os.environ['RSV_MISTRAL_BASE_URL'] = 'https://api.mistral.ai/v1/chat/completions'
```

## Complete Example

Here's a complete example that demonstrates Reservoir's memory capabilities:

```python
import os
from openai import OpenAI

def setup_reservoir_client():
    """Setup Reservoir client with proper configuration"""
    instance = "chat-example"
    partition = os.getenv("USER", "default")
    port = os.getenv('RESERVOIR_PORT', '3017')
    base_url = f"http://localhost:{port}/v1/partition/{partition}/instance/{instance}"
    
    return OpenAI(
        base_url=base_url,
        api_key=os.environ.get("OPENAI_API_KEY", "not-needed-for-ollama")
    )

def chat_with_memory(message, model="gpt-4"):
    """Send a message through Reservoir with automatic memory"""
    client = setup_reservoir_client()
    
    completion = client.chat.completions.create(
        model=model,
        messages=[
            {
                "role": "user",
                "content": message
            }
        ]
    )
    
    return completion.choices[0].message.content

# Example conversation that builds context
if __name__ == "__main__":
    # First message
    response1 = chat_with_memory("My name is Alice and I love Python programming.")
    print("Assistant:", response1)
    
    # Second message - Reservoir will automatically include context
    response2 = chat_with_memory("What programming language do I like?")
    print("Assistant:", response2)  # Will know you like Python!
    
    # Third message - Even more context
    response3 = chat_with_memory("Can you suggest a project for me?")
    print("Assistant:", response3)  # Will suggest Python projects for Alice!
```

## Benefits of Using Reservoir

When you use Reservoir with the OpenAI library, you get:

- **Automatic Context**: Previous conversations are automatically included
- **Cross-Session Memory**: Conversations persist across different Python sessions
- **Smart Token Management**: Reservoir handles token limits automatically
- **Multi-Provider Support**: Switch between different AI providers seamlessly
- **Local Storage**: All your conversation data stays on your device

## Next Steps

- Learn about [Partitioning & Organization](./features/partitioning.md) to organize your conversations
- Check out [Token Management](./features/token-management.md) to understand how Reservoir handles context limits
- Explore the [API Reference](./api/overview.md) for more advanced usage patterns
