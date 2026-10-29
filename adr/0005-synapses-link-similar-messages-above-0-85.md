# 0005. Synapses link similar messages above a cosine similarity of 0.85

Date: 2025-04-26

## Status

Proposed

## Context

A message's own thread is not enough context. The aim is to bring in related messages from
other conversations, and to follow a topic through the history. Searching the vector index on
every request is costly and unstable. Links made at ingest are cheap to follow later
(e7819df, 1f17adc).

## Decision

We will create a `SYNAPSE` relationship between a new message and any earlier message whose
embedding has a cosine similarity of at least 0.85 (`SYNAPSE_THRESHOLD` in
`src/repos/message/neo4j_message.rs`). Replies are linked separately by `RESPONDED_WITH`. The
`thread` command (7435d97) follows synapses backward from the latest message and adds recent
messages.

## Consequences

- Cross-conversation context and topic threads come from edges, not from a search per request.
- The threshold is a guess. The tests record that natural conversation often scores 0.6 to
  0.8, so 0.85 may be too strict and link too little.
- Changing the threshold only affects new messages. Existing edges stay unless `replay` is run.
- Links depend on the embedding model, so a model change ([0004](0004-embed-locally-with-fastembed.md))
  changes which messages link.
