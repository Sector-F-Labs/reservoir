# Command Line Interface

Reservoir provides a comprehensive command-line interface for managing your conversation data, searching through message history, and configuring the system. This section covers all available commands and their usage.

## Overview

Reservoir's CLI allows you to:
- Start the proxy server
- Search through conversations
- Import and export conversation data
- View recent messages
- Ingest new messages manually
- Configure system settings
- Replay embeddings for existing data

## Available Commands

### `reservoir start`

Start the Reservoir proxy server.

```bash
reservoir start [OPTIONS]
```

**Options:**
- `-o, --ollama` - Ollama mode which sets up on same default port as ollama useful for using as a proxy for clients that don't support setting a url
- `-h, --help` - Print help
- `-V, --version` - Print version

**Examples:**
```bash
# Start in normal mode
reservoir start

# Start in Ollama mode (uses port 11434)
reservoir start --ollama
```

### `reservoir search`

Search messages by keyword or semantic similarity.

```bash
reservoir search [OPTIONS] <TERM>
```

**Arguments:**
- `<TERM>` - The search term (keyword or semantic)

**Options:**
- `--semantic` - Use semantic search instead of keyword search
- `-p, --partition <PARTITION>` - Partition to search (defaults to "default")
- `-i, --instance <INSTANCE>` - Instance to search (defaults to partition)
- `-l, --link` - Use the same search strategy as RAG does when injecting into the model
- `-d, --deduplicate` - Deduplicate first similarity results
- `-h, --help` - Print help
- `-V, --version` - Print version

**Examples:**
```bash
# Keyword search
reservoir search "python programming"

# Semantic search
reservoir search --semantic "machine learning concepts"

# Search in specific partition/instance
reservoir search --partition alice --instance coding "neural networks"

# Use RAG search strategy
reservoir search --link --semantic "database design"

# Deduplicate results
reservoir search --deduplicate --semantic "API design"
```

### `reservoir export`

Export all message nodes as JSON.

```bash
reservoir export
```

**Options:**
- `-h, --help` - Print help

**Examples:**
```bash
# Export all messages to stdout
reservoir export > my_conversations.json

# Export and view
reservoir export | jq '.[0]'
```

### `reservoir import`

Import message nodes from a JSON file.

```bash
reservoir import <FILE>
```

**Arguments:**
- `<FILE>` - Path to the JSON file to import

**Options:**
- `-h, --help` - Print help
- `-V, --version` - Print version

**Examples:**
```bash
# Import from a file
reservoir import my_conversations.json

# Import from a backup
reservoir import backup_2024_01_15.json
```

### `reservoir view`

View last x messages in the default partition/instance.

```bash
reservoir view [OPTIONS] <COUNT>
```

**Arguments:**
- `<COUNT>` - Number of messages to display

**Options:**
- `-p, --partition <PARTITION>` - Partition to view (defaults to "default")
- `-i, --instance <INSTANCE>` - Instance to view (defaults to partition)
- `-h, --help` - Print help
- `-V, --version` - Print version

**Examples:**
```bash
# View last 10 messages
reservoir view 10

# View messages from specific partition
reservoir view --partition alice 5

# View messages from specific instance
reservoir view --partition alice --instance coding 15
```

### `reservoir ingest`

Ingest a message from stdin as a user MessageNode.

```bash
reservoir ingest [OPTIONS]
```

**Options:**
- `-p, --partition <PARTITION>` - Partition to save the message in (defaults to "default")
- `-i, --instance <INSTANCE>` - Instance to save the message in (defaults to partition)
- `--role <ROLE>` - Role to assign to the message (defaults to "user")
- `-h, --help` - Print help
- `-V, --version` - Print version

**Examples:**
```bash
# Ingest a user message
echo "How do I implement a binary search tree?" | reservoir ingest

# Ingest to specific partition/instance
echo "What are design patterns?" | reservoir ingest --partition alice --instance coding

# Ingest as assistant message
echo "Here's how to implement a BST..." | reservoir ingest --role assistant

# Ingest from file
cat question.txt | reservoir ingest --partition research --instance ai
```

### `reservoir config`

Set or get default configuration values with your config.toml.

```bash
reservoir config [OPTIONS]
```

**Options:**
- `-s, --set <SET>` - Set a configuration value. Use the format key=value. `reservoir config --set model=gpt-4-turbo`
- `-g, --get <GET>` - Get your current configuration value. `reservoir config --get model`
- `-h, --help` - Print help
- `-V, --version` - Print version

**Examples:**
```bash
# View current configuration
reservoir config --get semantic_context_size

# Set configuration value
reservoir config --set semantic_context_size=20

# Set Neo4j connection
reservoir config --set neo4j_uri=bolt://localhost:7687
```

### `reservoir replay`

Replay embeddings process.

```bash
reservoir replay [MODEL]
```

**Arguments:**
- `[MODEL]` - Partition to replay (defaults to "default")

**Options:**
- `-h, --help` - Print help
- `-V, --version` - Print version

**Examples:**
```bash
# Replay embeddings for default model
reservoir replay

# Replay for specific model
reservoir replay bge-large-en-v15
```

## Common Workflows

### Daily Usage

```bash
# Start the server
reservoir start

# View recent conversations
reservoir view 10

# Search for specific topics
reservoir search --semantic "machine learning"

# Add a note or question
echo "Remember to implement error handling" | reservoir ingest
```

### Data Management

```bash
# Export all data for backup
reservoir export > backup_$(date +%Y%m%d).json

# Import previous backup
reservoir import backup_20240115.json

# View configuration
reservoir config --get semantic_context_size
```

### Development and Testing

```bash
# Start in Ollama mode for local testing
reservoir start --ollama

# Search with debugging
reservoir search --link --deduplicate --semantic "API design"

# Replay embeddings after model changes
reservoir replay bge-large-en-v15
```

## Configuration

The CLI respects configuration from:
1. Command-line arguments (highest priority)
2. Configuration file (`~/.config/reservoir/reservoir.toml`)
3. Environment variables
4. Default values (lowest priority)

See [Environment Variables](../deployment/environment.md) for detailed configuration options.

## Error Handling

The CLI provides helpful error messages for common issues:

- **Connection errors**: Check if Neo4j is running
- **Permission errors**: Verify file permissions for import/export
- **Invalid arguments**: Use `--help` for correct syntax
- **Configuration errors**: Verify config file format

## Integration with Scripts

The CLI is designed to work well in scripts and automation:

```bash
#!/bin/bash
# Backup and restart script

# Export current data
reservoir export > "backup_$(date +%Y%m%d_%H%M%S).json"

# Restart with fresh embeddings
reservoir replay

# Start the server
reservoir start
```
