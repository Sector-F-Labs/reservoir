# Multi-Provider Support

Reservoir supports multiple AI providers through its flexible routing system. This allows you to use different AI models seamlessly while maintaining conversation context and history across all providers.

## Supported Providers

### OpenAI
- **Models**: GPT-4, GPT-4o, GPT-4o-mini, GPT-3.5-turbo, GPT-4o-search-preview
- **API Key Required**: Yes (`OPENAI_API_KEY`)
- **Endpoint**: `https://api.openai.com/v1/chat/completions`
- **Features**: Full feature support, web search capabilities

### Ollama
- **Models**: llama3.2, gemma3, and any locally installed models
- **API Key Required**: No
- **Endpoint**: `http://localhost:11434/v1/chat/completions`
- **Features**: Local inference, privacy-focused, custom model support

### Mistral AI
- **Models**: mistral-large-2402, mistral-medium, mistral-small
- **API Key Required**: Yes (`MISTRAL_API_KEY`)
- **Endpoint**: `https://api.mistral.ai/v1/chat/completions`
- **Features**: European AI provider, competitive performance

### Google Gemini
- **Models**: gemini-2.0-flash, gemini-2.5-flash-preview-05-20
- **API Key Required**: Yes (`GEMINI_API_KEY`)
- **Endpoint**: Custom Google AI endpoint
- **Features**: Google's latest AI models, multimodal capabilities

### Custom Providers
- **Models**: Any model name not explicitly configured
- **Default Routing**: Routes to Ollama by default
- **Configuration**: Set custom endpoints via environment variables

## Automatic Model Routing

Reservoir automatically determines which provider to use based on the model name in your request:

```json
{
  "model": "gpt-4",           // → Routes to OpenAI
  "model": "llama3.2",        // → Routes to Ollama
  "model": "mistral-large",   // → Routes to Mistral
  "model": "gemini-2.0-flash" // → Routes to Google
}
```

## Configuration

### Environment Variables

Set provider endpoints and API keys:

```bash
# API Keys
export OPENAI_API_KEY="sk-your-openai-key"
export MISTRAL_API_KEY="your-mistral-key"
export GEMINI_API_KEY="your-gemini-key"

# Custom Endpoints (optional)
export RSV_OPENAI_BASE_URL="https://api.openai.com/v1/chat/completions"
export RSV_OLLAMA_BASE_URL="http://localhost:11434/v1/chat/completions"
export RSV_MISTRAL_BASE_URL="https://api.mistral.ai/v1/chat/completions"
```

### Provider-Specific Features

#### OpenAI Features
- **Web Search**: Available with `gpt-4o-search-preview`
- **Function Calling**: Supported on compatible models
- **Vision**: GPT-4o supports image inputs
- **JSON Mode**: Structured output support

Example with web search:
```bash
curl "http://localhost:3017/partition/$USER/instance/research/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4o-search-preview",
        "messages": [{"role": "user", "content": "Latest AI developments"}],
        "web_search_options": {
            "enabled": true,
            "max_results": 5
        }
    }'
```

#### Ollama Features
- **Local Models**: No API key required
- **Privacy**: Data never leaves your machine
- **Custom Models**: Load any compatible model
- **Performance**: Direct local inference

Example with local model:
```bash
curl "http://localhost:3017/partition/$USER/instance/local/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -d '{
        "model": "llama3.2",
        "messages": [{"role": "user", "content": "Explain quantum computing"}]
    }'
```

## Multi-Provider Workflows

### Seamless Model Switching

You can switch between providers within the same conversation while maintaining context:

```python
import os
from openai import OpenAI

client = OpenAI(
    base_url="http://localhost:3017/v1/partition/myuser/instance/research",
    api_key=os.environ.get("OPENAI_API_KEY")
)

# Start with OpenAI
response1 = client.chat.completions.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Explain neural networks"}]
)

# Continue with Ollama (context is preserved)
response2 = client.chat.completions.create(
    model="llama3.2",
    messages=[{"role": "user", "content": "What did we just discuss?"}]
)

# Switch to Mistral (still has context)
response3 = client.chat.completions.create(
    model="mistral-large-2402",
    messages=[{"role": "user", "content": "How does this relate to AI safety?"}]
)
```

### Provider-Specific Use Cases

#### Development Workflow
```bash
# Use Ollama for quick local testing
curl -d '{"model": "llama3.2", "messages": [...]}' localhost:3017/...

# Use OpenAI for production queries
curl -d '{"model": "gpt-4", "messages": [...]}' localhost:3017/...

# Use Mistral for European compliance
curl -d '{"model": "mistral-large", "messages": [...]}' localhost:3017/...
```

## Error Handling

Reservoir provides consistent error handling across all providers:

### Common Error Responses

```json
{
  "error": {
    "type": "invalid_request_error",
    "message": "Invalid model specified",
    "code": "model_not_found"
  }
}
```

### Provider-Specific Errors

- **OpenAI**: Rate limits, quota exceeded, invalid API key
- **Ollama**: Model not found, service unavailable
- **Mistral**: Authentication errors, model access restrictions
- **Gemini**: API quota limits, geographic restrictions

## Performance Considerations

### Provider Comparison

| Provider | Latency | Cost | Privacy | Features |
|----------|---------|------|---------|----------|
| OpenAI | Medium | High | Cloud | Most comprehensive |
| Ollama | Low | Free | Local | Basic, customizable |
| Mistral | Medium | Medium | Cloud | European focus |
| Gemini | Medium | Medium | Cloud | Google integration |

### Optimization Tips

1. **Use Ollama for development**: Faster iteration, no API costs
2. **Use OpenAI for production**: Most reliable, feature-rich
3. **Use Mistral for compliance**: European data residency
4. **Cache responses**: Reduce API calls and costs

## Custom Provider Integration

To add a new OpenAI-compatible provider:

1. **Set the endpoint URL**:
   ```bash
   export RSV_CUSTOM_BASE_URL="https://api.custom-provider.com/v1/chat/completions"
   ```

2. **Configure model routing** (if needed):
   ```rust
   // In your configuration
   match model_name {
       "custom-model" => "custom-provider",
       _ => "default-provider"
   }
   ```

3. **Test the integration**:
   ```bash
   curl "http://localhost:3017/partition/$USER/instance/test/v1/chat/completions" \
       -H "Content-Type: application/json" \
       -H "Authorization: Bearer $CUSTOM_API_KEY" \
       -d '{"model": "custom-model", "messages": [...]}'
   ```

## Future Enhancements

Planned improvements for multi-provider support:

- **Load Balancing**: Distribute requests across multiple providers
- **Failover**: Automatic fallback to backup providers
- **Cost Optimization**: Route to cheapest provider based on request
- **Model Capabilities**: Automatic routing based on required features
- **Custom Routing Rules**: User-defined routing logic

## Troubleshooting

### Provider Connection Issues

**Check provider availability**:
```bash
# OpenAI
curl https://api.openai.com/v1/models -H "Authorization: Bearer $OPENAI_API_KEY"

# Ollama
curl http://localhost:11434/api/tags

# Mistral
curl https://api.mistral.ai/v1/models -H "Authorization: Bearer $MISTRAL_API_KEY"
```

**Common solutions**:
- Verify API keys are correctly set
- Check network connectivity
- Ensure provider services are running
- Validate model names and availability

Multi-provider support makes Reservoir a flexible foundation for AI applications, allowing you to choose the best provider for each use case while maintaining conversation continuity.