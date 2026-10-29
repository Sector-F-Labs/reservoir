# 0003. Embeddings as separate nodes, one label and index per model

Date: 2025-05-10

## Status

Proposed

## Context

Messages first carried their embedding as a property. That tied a message to one model and one
vector size. Neo4j vector indexes are fixed to a dimension, and Reservoir had already used
OpenAI's 1536-dimension vectors while planning local models with 1024 (e418aa3, 2f92701).
Changing the model would otherwise mean re-embedding everything in place.

## Decision

We will store each embedding as its own node, linked to its message by `HAS_EMBEDDING`. The
node label and vector index name follow the vector size: `Embedding1536` and `embedding1536`,
then `Embedding1024` and `embedding1024` (27f6b1b). Each node records its model, partition and
instance. A migration (`move_embeddings_out.cypher`) moved the old property into nodes, and a
`replay` command re-embeds stored messages for a new model (1dfc3b8).

## Consequences

- A message can have embeddings from several models, and a new model gets a new index without
  touching the old one.
- Search must ask for the embedding type it wants, so the embedding info is passed through the
  service and repository layers.
- Each model adds an index and a migration. The label is derived from the vector size, so two
  models with the same size share an index.
- Changing the default model means running `replay`.
