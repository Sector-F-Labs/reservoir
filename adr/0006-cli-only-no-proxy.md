# 0006. A CLI only: remove the proxy server

Date: 2026-02-10

## Status

Proposed

## Context

Reservoir began as a stateful proxy for OpenAI-compatible APIs (ba3e2b1). It took chat
completion requests, added stored context, forwarded them to OpenAI, Mistral, Gemini or Ollama,
and stored the replies. That needed a web server, per-provider model tables and API keys. It
also overlapped with the clients that already call those providers. The CLI commands (ingest,
search, view, export, import) were what the current workflows used; the commit message says the proxy wasn't.

## Decision

We will drop the proxy and keep Reservoir as a CLI memory store (7435d97): ingest, search,
view, thread, export, import, replay and config. The HTTP server, chat completion handler,
OpenAI client and the static Ollama model files were removed. Other systems integrate by
calling the CLI.

## Consequences

- Fewer dependencies, no server to deploy, and faster tests.
- Reservoir no longer adds context to a chat request itself. A caller must run `thread` or
  `search` and build the prompt.
- The Ollama and cgip integration docs, the OpenAPI file and the Hurl scripts describe the old
  server and are out of date.
- Bringing back a server would be a new decision. The CLI's commands are a smaller surface to
  keep stable.
