# 0002. Partition and instance scope every message

Date: 2025-04-22

## Status

Proposed

## Context

One Reservoir store serves several producers. Without a boundary, a search from one app would
return another app's messages, and a thread would mix conversations. The README presents the
partition/instance hierarchy as multi-tenant isolation. The Python example (43eff32) and the
later MCP bridge ([0008](0008-mcp-bridge-wraps-the-cli.md)) both rely on it.

## Decision

We will tag every message with a `partition` and an `instance`. A partition is the owner
(an application or a person); an instance is one conversation or context inside it. Ingest,
search, view and thread take `--partition` and `--instance`, and embedding nodes carry both
too (a09dc09). Queries filter on them, so a search stays inside the partition it names.

## Consequences

- Producers share one database without seeing each other's data.
- Every caller has to pass the pair, or fall back to a default partition. The MCP bridge sets
  both at startup so its calls don't land in the default one.
- The boundary is a convention enforced by queries, not by Neo4j. It isn't access control.
- `export` and `import` act on the whole store; they don't take the pair. An instance left out defaults to the partition.
