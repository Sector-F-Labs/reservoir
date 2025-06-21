# Ollama Integration

Reservoir works seamlessly with [Ollama](https://ollama.ai/), allowing you to use local AI models with persistent memory and context enrichment. This is perfect for privacy-focused workflows where you want to keep all your conversations completely local.

## What is Ollama?

Ollama is a tool that makes it easy to run large language models locally on your machine. It supports popular models like Llama, Gemma, and many others, all running entirely on your hardware.

## Benefits of Using Ollama with Reservoir

- **Complete Privacy**: All conversations stay on your device
- **No API Keys**: No need for cloud service API keys
- **Offline Capable**: Works without internet connection
- **Cost Effective**: No usage-based charges
- **Full Control**: Choose exactly which models to use

## Setting Up Ollama

### Step 1: Install Ollama

First, install Ollama from [ollama.ai](https://ollama.ai/):

```bash
# On macOS
brew install ollama

# On Linux
curl -fsSL https://ollama.ai/install.sh | sh

# Or download from https://ollama.ai/download
```

### Step 2: Start Ollama Service

```bash
ollama serve
```

This starts the Ollama service on `http://localhost:11434`.

### Step 3: Download Models

Download the models you want to use:

```bash
# Download Gemma 3 (Google's model)
ollama pull gemma3

# Download Llama 3.2 (Meta's model)
ollama pull llama3.2

# Download Mistral (Mistral AI's model)
ollama pull mistral

# See all available models
ollama list
```

## Using Ollama with Reservoir

### Regular Mode

By default, Reservoir routes any unrecognized model names to Ollama:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/ollama-chat/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [
            {
                "role": "user",
                "content": "Explain machine learning in simple terms."
            }
        ]
    }'
```

No API key required!

### Ollama Mode

Reservoir also provides a special "Ollama mode" that makes it a drop-in replacement for Ollama's API:

```bash
# Start Reservoir in Ollama mode
cargo run -- start --ollama
```

In Ollama mode, Reservoir:
- Uses the same API endpoints as Ollama
- Provides the same response format
- Adds memory and context enrichment automatically
- Makes existing Ollama clients work with persistent memory

#### Testing Ollama Mode

```bash
# Test with the standard Ollama endpoint format
curl "http://127.0.0.1:3017/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [
            {
                "role": "user",
                "content": "Hello, can you remember our previous conversations?"
            }
        ]
    }'
```

## Popular Ollama Models

### Gemma 3 (Google)

Excellent for general conversation and coding:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/coding/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "gemma3",
        "messages": [
            {
                "role": "user",
                "content": "Write a Python function to sort a list of dictionaries by a specific key."
            }
        ]
    }'
```

### Llama 3.2 (Meta)

Great for reasoning and complex tasks:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/reasoning/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "llama3.2",
        "messages": [
            {
                "role": "user",
                "content": "Solve this logic puzzle: If all roses are flowers, and some flowers are red, can we conclude that some roses are red?"
            }
        ]
    }'
```

### Mistral 7B

Efficient and good for general tasks:

```bash
curl "http://127.0.0.1:3017/partition/$USER/instance/general/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "mistral",
        "messages": [
            {
                "role": "user",
                "content": "Summarize the key points of quantum computing for a beginner."
            }
        ]
    }'
```

## Python Integration with Ollama

Using the OpenAI library with local Ollama models:

```python
import os
from openai import OpenAI

# Setup for Ollama through Reservoir
INSTANCE = "ollama-python"
PARTITION = os.getenv("USER", "default")
RESERVOIR_PORT = os.getenv('RESERVOIR_PORT', '3017')
RESERVOIR_BASE_URL = f"http://localhost:{RESERVOIR_PORT}/v1/partition/{PARTITION}/instance/{INSTANCE}"

client = OpenAI(
    base_url=RESERVOIR_BASE_URL,
    api_key="not-needed-for-ollama"  # Ollama doesn't require API keys
)

# Chat with memory using local model
completion = client.chat.completions.create(
    model="gemma3",
    messages=[
        {
            "role": "user",
            "content": "My favorite hobby is gardening. What plants would you recommend for a beginner?"
        }
    ]
)

print(completion.choices[0].message.content)

# Ask a follow-up that requires memory
follow_up = client.chat.completions.create(
    model="gemma3",
    messages=[
        {
            "role": "user", 
            "content": "What tools do I need to get started with my hobby?"
        }
    ]
)

print(follow_up.choices[0].message.content)
# Will remember you're interested in gardening!
```

## Environment Configuration

You can customize the Ollama endpoint if needed:

```bash
# Default Ollama endpoint
export RSV_OLLAMA_BASE_URL="http://localhost:11434/v1/chat/completions"

# Custom endpoint (if running Ollama on different port/host)
export RSV_OLLAMA_BASE_URL="http://192.168.1.100:11434/v1/chat/completions"
```

## Performance Tips

### Model Selection

- **gemma3**: Good balance of speed and quality
- **llama3.2**: Higher quality but slower
- **mistral**: Fast and efficient
- **smaller models** (7B parameters): Faster on limited hardware
- **larger models** (13B+): Better quality but require more resources

### Hardware Considerations

- **RAM**: 8GB minimum, 16GB+ recommended for larger models
- **GPU**: Optional but significantly speeds up inference
- **Storage**: Models range from 4GB to 40GB+ each

### Optimizing Performance

```bash
# Use GPU acceleration if available
ollama run gemma3 --gpu

# Monitor resource usage
ollama ps
```

## Troubleshooting Ollama

### Common Issues

#### Ollama Not Found

```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# If not running, start it
ollama serve
```

#### Model Not Available

```bash
# List installed models
ollama list

# Pull missing model
ollama pull gemma3
```

#### Performance Issues

```bash
# Check system resources
ollama ps

# Try a smaller model
ollama pull gemma3:2b  # 2B parameter version
```

### Error Messages

- **"connection refused"**: Ollama service isn't running
- **"model not found"**: Model needs to be pulled with `ollama pull`
- **"out of memory"**: Try a smaller model or close other applications

## Combining Local and Cloud Models

One of Reservoir's strengths is seamlessly switching between local and cloud models:

```python
import os
from openai import OpenAI

# Same client setup
client = OpenAI(base_url=RESERVOIR_BASE_URL, api_key=os.environ.get("OPENAI_API_KEY", ""))

# Start with local model for initial draft
local_response = client.chat.completions.create(
    model="gemma3",  # Local Ollama model
    messages=[{"role": "user", "content": "Write a draft email about project updates"}]
)

# Refine with cloud model for better quality
cloud_response = client.chat.completions.create(
    model="gpt-4",  # Cloud OpenAI model
    messages=[{"role": "user", "content": "Please improve the writing quality and make it more professional"}]
)
```

Both responses will have access to the same conversation context!

## Next Steps

- **[Python Integration](./python-integration.md)** - Use Ollama models from Python
- **[Features - Multi-Provider Support](./features/providers.md)** - Learn about mixing different providers
- **[Partitioning & Organization](./features/partitioning.md)** - Organize your local conversations
- **[Architecture - Data Model](./architecture/data-model.md)** - Understand how conversations are stored

**Ready to go private?** 🔒 With Ollama and Reservoir, you have a completely local AI assistant with persistent memory!
