# Introduction

Reservoir is your helpful memory for AI conversations. It sits between your app and any OpenAI-compatible Chat Completions API, making it easier to have rich, ongoing conversations with your favorite language models from multiple providers.

## What is Reservoir?

When you use any [OpenAI-compatible Chat Completions API](https://platform.openai.com/docs/guides/chat), you need to send the full conversation history with every request. For example:

```json
[
  {"role": "user", "content": "What is 1 + 1?"},
  {"role": "assistant", "content": "2"},
  {"role": "user", "content": "What is the answer times 3?"}
]
```

If you only send the last question, the model won't know what "the answer" refers to. You have to keep track of all previous messages and include them every time.

**This can get tricky as conversations grow!**

Reservoir acts as a smart proxy: it automatically stores your chat history and inserts the right context into each request. You just talk to any OpenAI-compatible API as usual and Reservoir handles the memory, context, and even finds other relevant messages from your past conversations to help the model give better answers.

## Why Use Reservoir?

- **Own your AI history**: All your conversations are stored locally, never in the cloud.
- **Search and recall**: Instantly find previous chats, ideas, or code snippets from your AI interactions.
- **Enrich context**: Automatically inject relevant history into new prompts for more coherent, personalized responses.
- **Visualize conversations**: See how your discussions branch and connect over time.
- **Stay private**: Your data never leaves your device.

## Supported Providers

**Supported Providers:**
- OpenAI (GPT-4, GPT-4o, GPT-3.5-turbo, etc.)
- Ollama (llama3.2, gemma3, and any local models)
- Mistral AI (mistral-large-2402, etc.)
- Google Gemini (gemini-2.0-flash, etc.)
- Any OpenAI-compatible API endpoint

## Key Benefits

- **No more manual history management**: Reservoir automatically tracks conversation context
- **Automatic context enrichment**: Relevant past conversations are injected into new requests
- **Your data stays private and local**: Everything is stored on your device
- **Multi-app support**: Use one Reservoir instance across multiple applications
- **Flexible organization**: Partition conversations by app, project, or any context you choose

## How It Works

Reservoir intercepts your API calls, enriches them with relevant history, manages token limits, and then forwards them to the actual LLM service. Every interaction is stored on your device, building a personal knowledge base that never leaves your network.

A single thread of conversation can span multiple models without losing context, allowing you to seamlessly switch between different AI providers while maintaining the flow of your discussion.

![Conversation Graph View](../docs/conversation_graph_view.png)

## Use Cases

- **Chat Applications**: Add memory to any chat interface
- **Development Tools**: Keep context across coding sessions
- **Research**: Build a personal knowledge base from AI interactions
- **Multi-Provider Workflows**: Switch between different AI models seamlessly
- **Team Collaboration**: Share conversation contexts across team members

Ready to get started? Check out the [Quick Start](./quick-start.md) guide!