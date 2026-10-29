# 0004. Embed locally with FastEmbed and BGE-Large

Date: 2025-05-24

## Status

Proposed

## Context

Embeddings started with OpenAI's API (ada-002, 1536 dimensions). That sends every message to a
third party and needs a key, even for a tool meant to hold a private memory. Commit 94a8aff
began local embedding. A later fix (49a04ee) found `ingest` still calling OpenAI by mistake,
which shows how easy it was to mix the two.

## Decision

We will compute embeddings in-process with the `fastembed` crate and `BGE-Large-EN-v1.5`
(1024 dimensions, `bge-large-en-v15`). The model downloads on first use into a `reservoir/models`
folder under the user's data directory. The OpenAI embeddings client was removed with the
proxy ([0006](0006-cli-only-no-proxy.md)).

## Consequences

- Messages never leave the machine to be embedded, and no API key is needed.
- The first run downloads the model, and loading it costs time and memory on every command.
- The model is fixed in code. A different one needs a new `EmbeddingInfo`, an index (see
  [0003](0003-embeddings-as-separate-nodes-per-model.md)) and a `replay`.
- Neighbouring ADRs assume the 1024-dimension size.
