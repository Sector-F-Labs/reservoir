# 0008. Expose Reservoir to agents with a Python MCP bridge around the CLI

Date: 2026-03-15

## Status

Proposed

## Context

After the move to a CLI ([0006](0006-cli-only-no-proxy.md)), agents that speak the Model
Context Protocol had no way to use Reservoir. The options were to put MCP in the Rust binary or
to wrap it. FastMCP makes a server short in Python, and the CLI already has everything an agent
needs.

## Decision

We will ship `reservoir_mcp`, a Python MCP server built on FastMCP (48bb482). A bridge class
runs the `reservoir` binary as a subprocess for search, view, thread, ingest, export and config,
and turns a non-zero exit into an error with the CLI's output. The partition and instance are
set when the server starts and added to the commands that take them, unless the caller passes
its own, so an agent doesn't write to the default partition ([0002](0002-partition-and-instance-scope-every-message.md)).

## Consequences

- The Rust code stays free of an MCP dependency, and the CLI remains the one interface.
- Each tool call spawns the CLI, with the model loading cost that implies.
- The bridge has a hand-kept list of commands that accept partition and instance. A new
  partitioned command has to be added to it.
- The binary has to be on `PATH`, or its path given to the bridge.
