# Search & Retrieval

Reservoir provides powerful search capabilities for finding relevant conversations and messages across your entire conversation history. The search system supports both keyword-based and semantic similarity searches, enabling you to discover related discussions even when they use different terminology.

## Search Methods

### Keyword Search

Traditional text-based search that finds exact matches or partial matches within message content.

**CLI Usage:**
```bash
# Basic keyword search
reservoir search "python programming"

# Search in specific partition
reservoir search --partition alice "machine learning"
```

**Characteristics:**
- Fast and precise for exact term matches
- Case-insensitive matching
- Supports partial word matching
- Best for finding specific technical terms or names

### Semantic Search

Vector-based similarity search that finds conceptually related messages even when they use different words.

**CLI Usage:**
```bash
# Semantic search
reservoir search --semantic "machine learning concepts"

# Use RAG strategy (same as context enrichment)
reservoir search --link --semantic "database design"
```

**Characteristics:**
- Finds conceptually similar content
- Works across different terminology
- Uses BGE-Large-EN-v1.5 embeddings
- Powers Reservoir's context enrichment system

## Search Options

### Partitioning

Scope your search to specific organizational boundaries:

```bash
# Search in specific partition
reservoir search --partition alice "neural networks"

# Search in specific instance within partition
reservoir search --partition alice --instance coding "API design"
```

### Deduplication

Remove duplicate or highly similar results:

```bash
# Remove duplicate results
reservoir search --deduplicate --semantic "error handling"
```

### RAG Strategy

Use the same search strategy that powers context enrichment:

```bash
# Use advanced search with synapse expansion
reservoir search --link --semantic "software architecture"
```

The `--link` option:
- Searches for semantically similar messages
- Expands results using synapse relationships
- Follows conversation threads
- Deduplicates automatically
- Limits results to most relevant matches

## Search Implementation

### Vector Similarity

Reservoir uses cosine similarity to find related messages:

1. **Query Embedding**: Your search term is converted to a vector using BGE-Large-EN-v1.5
2. **Index Search**: Neo4j's vector index finds similar message embeddings
3. **Scoring**: Results are ranked by similarity score (0.0 to 1.0)
4. **Filtering**: Results are filtered by partition/instance boundaries

### Synapse Expansion

When using `--link`, the search expands beyond direct similarity:

1. **Initial Search**: Find semantically similar messages
2. **Synapse Following**: Explore connected messages via SYNAPSE relationships
3. **Thread Discovery**: Follow conversation threads and related discussions
4. **Relevance Scoring**: Combine similarity scores with relationship strength
5. **Result Limiting**: Return top matches within context limits

## Example Queries

### Finding Programming Discussions

```bash
# Find all Python-related conversations
reservoir search --semantic "python programming"

# Find specific error discussions
reservoir search "TypeError" 

# Find design pattern conversations
reservoir search --link --semantic "software design patterns"
```

### Research and Analysis

```bash
# Find machine learning discussions
reservoir search --semantic "neural networks deep learning"

# Find database-related conversations
reservoir search --partition research --semantic "database optimization"

# Find recent discussions on a topic
reservoir view 50 | grep -i "kubernetes"
```

### Cross-Conversation Discovery

```bash
# Find related discussions across all conversations
reservoir search --link --semantic "microservices architecture"

# Discover connections between topics
reservoir search --deduplicate --semantic "testing strategies"
```

## Search Results Format

### CLI Output

Search results include:
- **Timestamp**: When the message was created
- **Partition/Instance**: Organizational context
- **Role**: User or assistant message
- **Content**: The actual message text
- **Score**: Similarity score (for semantic search)

### JSON Format

When exported, search results follow the MessageNode structure:

```json
{
  "trace_id": "abc123-def456",
  "partition": "alice",
  "instance": "coding",
  "role": "user",
  "content": "How do I implement error handling in async functions?",
  "timestamp": "2024-01-15T10:30:00Z",
  "embedding": [0.1, -0.2, 0.3, ...],
  "url": null
}
```

## Integration with Context Enrichment

The search system directly powers Reservoir's context enrichment:

1. **Automatic Search**: Every user message triggers a semantic search
2. **Context Building**: Search results become conversation context
3. **Relevance Filtering**: Only high-quality matches (>0.85 similarity) are used
4. **Token Management**: Results are truncated to fit model token limits

## Performance Considerations

### Vector Index

Reservoir maintains optimized vector indices for fast search:

```cypher
CREATE VECTOR INDEX embedding1536 
FOR (n:Embedding1536) ON (n.embedding) 
OPTIONS {
  indexConfig: {
    `vector.dimensions`: 1536,
    `vector.similarity_function`: 'cosine'
  }
}
```

### Search Strategies

- **Keyword Search**: Fastest for exact matches
- **Basic Semantic**: Good balance of speed and relevance
- **RAG Strategy (`--link`)**: Most comprehensive but slower
- **Deduplication**: Adds processing time but improves result quality

### Optimization Tips

1. **Use Specific Partitions**: Reduces search space
2. **Keyword for Exact Terms**: Faster than semantic for specific names
3. **Semantic for Concepts**: Better for finding related ideas
4. **Limit Result Count**: Implicit in CLI, configurable in API

## Advanced Usage

### Combining with Other Commands

```bash
# Search and then view context
reservoir search --semantic "error handling" | head -5
reservoir view 10

# Search and ingest related information
echo "Related to error handling discussion" | reservoir ingest

# Export search results for analysis
reservoir search --semantic "API design" > api_discussions.txt
```

### Scripting and Automation

```bash
#!/bin/bash
# Find and analyze topic discussions

TOPIC="$1"
echo "Searching for discussions about: $TOPIC"

# Semantic search with RAG strategy
reservoir search --link --semantic "$TOPIC" > "search_results_$TOPIC.txt"

# Count total discussions
TOTAL=$(wc -l < "search_results_$TOPIC.txt")
echo "Found $TOTAL related messages"

# Show recent activity
echo "Recent activity:"
reservoir view 20 | grep -i "$TOPIC" | head -3
```

The search system is designed to make your conversation history searchable and discoverable, turning your accumulated AI interactions into a valuable knowledge base that grows more useful over time.
