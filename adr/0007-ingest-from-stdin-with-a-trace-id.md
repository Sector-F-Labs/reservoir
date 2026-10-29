# 0007. Ingest messages from stdin, tagged with an optional trace id

Date: 2025-05-11

## Status

Proposed

## Context

Once the proxy was no longer the way in ([0006](0006-cli-only-no-proxy.md)), producers needed a
simple way to add messages: shell scripts (7afba3e, fd0a54f), other tools and the MCP bridge.
Later, a producer wanted to tie a stored message back to an event in its own system.

## Decision

We will have `ingest` read the message body from stdin, with `--role` to say who wrote it
(b7084f0, 56711b2). It writes only the result to stdout and logs to stderr (bdb9dd1), so it
composes in pipes. An optional `--trace-id` records an external identifier on the message
(597b5ce).

## Consequences

- Any program that can write to a pipe can feed Reservoir, in any language.
- Each call starts a process and loads the embedding model, so bulk loading is better done
  through `import` (JSON).
- The trace id is stored but Reservoir doesn't look messages up by it.
