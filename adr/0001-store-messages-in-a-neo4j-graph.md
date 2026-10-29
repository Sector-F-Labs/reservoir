# 0001. Store messages in a Neo4j graph

Date: 2025-04-22

## Status

Proposed

## Context

Reservoir exists because chat APIs are stateless: every request has to carry the whole
conversation, which runs into token limits and can't reach related conversations. The first
commits (5f942c2, 44724f2) settled where messages live.

The data is relationships as much as rows: a reply follows a message, and a message is
similar to messages in other conversations. Following those links is the main query. A graph
database makes that a traversal rather than a chain of joins. The README's data model reflects
this: `MessageNode`, `RESPONDED_WITH` and `SYNAPSE`.

## Decision

We will store messages, embeddings and the links between them in Neo4j, accessed from Rust
with the `neo4rs` crate. Connection settings come from `reservoir.toml` or `NEO4J_URI`,
`NEO4J_USER` and `NEO4J_PASSWORD`, with an optional database name (fd88c94). `docker-compose.yml`
runs a local instance. Schema changes ship as Cypher scripts in `migrations/`.

## Consequences

- Thread context and cross-conversation links are graph queries, and vector search comes from
  Neo4j's vector indexes (see [0003](0003-embeddings-as-separate-nodes-per-model.md)).
- Running Reservoir needs a Neo4j server. That is the largest setup cost.
- Migrations are scripts run by hand with `make migrate`, not applied automatically.
- Moving to another store means rewriting `src/repos/`, and the Cypher in it.
