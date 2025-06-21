# Import/Export

Reservoir provides comprehensive import and export capabilities for backing up your conversation data, migrating between systems, and integrating with external tools. The system exports data in JSON format, preserving all message metadata, embeddings, and relationships.

## Export Functionality

### Basic Export

Export all conversation data to JSON format:

```bash
# Export to stdout
reservoir export

# Save to file
reservoir export > conversations.json

# Export with timestamp
reservoir export > backup_$(date +%Y%m%d_%H%M%S).json
```

### Export Format

Each exported message includes complete metadata:

```json
[
  {
    "id": null,
    "trace_id": "550e8400-e29b-41d4-a716-446655440000",
    "partition": "alice",
    "instance": "coding",
    "content": "How do I implement error handling in async functions?",
    "role": "user",
    "embedding": [0.123, -0.456, 0.789, ...],
    "url": null,
    "timestamp": 1705315800000
  },
  {
    "id": null,
    "trace_id": "550e8400-e29b-41d4-a716-446655440001",
    "partition": "alice",
    "instance": "coding",
    "content": "Here are several approaches to error handling in async functions...",
    "role": "assistant",
    "embedding": [0.234, -0.567, 0.890, ...],
    "url": null,
    "timestamp": 1705315815000
  }
]
```

### What's Included in Export

- **Complete Message Data**: All message content and metadata
- **Vector Embeddings**: Full embedding vectors for similarity search
- **Partition Organization**: Partition and instance information
- **Conversation Structure**: Trace IDs linking user/assistant pairs
- **Timestamps**: Precise timing information
- **Roles**: User, assistant, and system message roles

### Export Use Cases

**Data Backup**
```bash
# Daily backup
reservoir export > "backup_$(date +%Y%m%d).json"

# Compressed backup
reservoir export | gzip > "backup_$(date +%Y%m%d).json.gz"
```

**Migration**
```bash
# Export from source system
reservoir export > migration_data.json

# Transfer to new system
scp migration_data.json user@newserver:/path/to/reservoir/
```

**Analysis**
```bash
# Export for external analysis
reservoir export | jq '.[] | select(.role=="user")' > user_messages.json

# Export specific time range
reservoir export | jq '.[] | select(.timestamp > 1705315800000)' > recent_messages.json
```

## Import Functionality

### Basic Import

Import conversation data from JSON files:

```bash
# Import from file
reservoir import conversations.json

# Import from compressed backup
gunzip -c backup_20240115.json.gz | reservoir import /dev/stdin
```

### Import Behavior

**Data Validation**
- Validates JSON format and structure
- Checks required fields (trace_id, partition, instance, role, content)
- Verifies embedding vector format and dimensions

**Duplicate Handling**
- Skips messages with duplicate trace_id and role combinations
- Preserves existing data integrity
- Logs skipped duplicates for review

**Relationship Reconstruction**
- Automatically rebuilds RESPONDED_WITH relationships
- Recreates HAS_EMBEDDING connections
- Maintains partition/instance boundaries

### Import Process

1. **File Reading**: Load and parse JSON data
2. **Validation**: Check data format and completeness
3. **Message Creation**: Create MessageNode entries
4. **Embedding Processing**: Store vector embeddings
5. **Relationship Building**: Establish graph relationships
6. **Index Updates**: Update vector indices

### Import Examples

**Complete System Restore**
```bash
# Stop Reservoir service
systemctl stop reservoir

# Clear existing data (if needed)
# WARNING: This is destructive!

# Import backup
reservoir import full_backup_20240115.json

# Verify import
reservoir view 10
```

**Selective Import**
```bash
# Import specific partition data
cat full_backup.json | jq '.[] | select(.partition=="alice")' > alice_data.json
reservoir import alice_data.json

# Import recent messages only
cat backup.json | jq '.[] | select(.timestamp > 1705315800000)' > recent.json
reservoir import recent.json
```

## Advanced Export/Import

### Filtering Exports

**By Partition**
```bash
# Export specific user's data
reservoir export | jq '.[] | select(.partition=="alice")' > alice_conversations.json
```

**By Time Range**
```bash
# Export last 24 hours
YESTERDAY=$(date -d '1 day ago' +%s)000
reservoir export | jq ".[] | select(.timestamp > $YESTERDAY)" > recent_conversations.json
```

**By Role**
```bash
# Export only user messages
reservoir export | jq '.[] | select(.role=="user")' > user_questions.json

# Export only assistant responses
reservoir export | jq '.[] | select(.role=="assistant")' > ai_responses.json
```

**By Content**
```bash
# Export messages containing specific terms
reservoir export | jq '.[] | select(.content | test("python|programming"; "i"))' > programming_discussions.json
```

### Data Transformation

**Convert to CSV**
```bash
reservoir export | jq -r '.[] | [.timestamp, .partition, .instance, .role, .content] | @csv' > conversations.csv
```

**Extract Text Only**
```bash
reservoir export | jq -r '.[] | .content' > all_messages.txt
```

**Create Markdown Format**
```bash
reservoir export | jq -r '.[] | "## " + (.timestamp | tostring) + " (" + .role + ")\n\n" + .content + "\n"' > conversations.md
```

### Batch Operations

**Multiple File Import**
```bash
# Import multiple backup files
for file in backup_*.json; do
    echo "Importing $file..."
    reservoir import "$file"
done
```

**Incremental Backup Strategy**
```bash
#!/bin/bash
# Incremental backup script

BACKUP_DIR="/backup/reservoir"
LAST_BACKUP_TIME=$(cat "$BACKUP_DIR/.last_backup" 2>/dev/null || echo "0")
CURRENT_TIME=$(date +%s)000

# Export messages since last backup
reservoir export | jq ".[] | select(.timestamp > $LAST_BACKUP_TIME)" > "$BACKUP_DIR/incremental_$(date +%Y%m%d_%H%M%S).json"

# Update last backup time
echo "$CURRENT_TIME" > "$BACKUP_DIR/.last_backup"
```

## Data Migration Workflows

### System Migration

**Complete Migration**
```bash
# Source system
reservoir export > complete_migration.json

# Target system  
reservoir import complete_migration.json

# Verify migration
SOURCE_COUNT=$(jq length complete_migration.json)
TARGET_COUNT=$(reservoir export | jq length)
echo "Source: $SOURCE_COUNT messages, Target: $TARGET_COUNT messages"
```

**Partition Migration**
```bash
# Migrate specific user to new system
reservoir export | jq '.[] | select(.partition=="alice")' > alice_migration.json

# On target system
reservoir import alice_migration.json

# Verify partition migration
reservoir view --partition alice 10
```

### Cross-System Integration

**Export for External Processing**
```bash
# Export for machine learning analysis
reservoir export | jq '.[] | {content: .content, embedding: .embedding}' > ml_dataset.json

# Export conversation pairs for training
reservoir export | jq -r 'group_by(.trace_id) | .[] | select(length == 2) | {user: .[0].content, assistant: .[1].content}' > conversation_pairs.json
```

**Import from External Sources**

Convert external data to Reservoir format:
```json
{
  "trace_id": "external-001",
  "partition": "imported",
  "instance": "external_system",
  "content": "Question from external system",
  "role": "user",
  "embedding": [], // Will be generated if empty
  "url": null,
  "timestamp": 1705315800000
}
```

## Data Integrity and Verification

### Export Verification

```bash
# Check export completeness
EXPORTED_COUNT=$(reservoir export | jq length)
echo "Exported $EXPORTED_COUNT messages"

# Verify embeddings
EMBEDDED_COUNT=$(reservoir export | jq '[.[] | select(.embedding | length > 0)] | length')
echo "$EMBEDDED_COUNT messages have embeddings"

# Check partition distribution
reservoir export | jq -r '.[] | .partition' | sort | uniq -c
```

### Import Validation

```bash
# Validate JSON format before import
jq . backup.json > /dev/null && echo "Valid JSON" || echo "Invalid JSON"

# Check required fields
jq '.[] | select(.trace_id and .partition and .instance and .role and .content)' backup.json | jq length

# Verify import success
reservoir view 10
reservoir search --semantic "test query"
```

## Performance Considerations

### Large Dataset Handling

**Streaming Export**
```bash
# For very large datasets, process in chunks
reservoir export | jq -c '.[]' | split -l 1000 - chunk_

# Import chunks
for chunk in chunk_*; do
    jq -s '.' "$chunk" | reservoir import /dev/stdin
done
```

**Compression**
```bash
# Compress exports to save space
reservoir export | gzip > backup.json.gz

# Decompress for import
gunzip -c backup.json.gz | reservoir import /dev/stdin
```

### Network Transfer

**Efficient Transfer**
```bash
# Direct transfer without intermediate files
ssh source_server 'reservoir export' | reservoir import /dev/stdin

# Compressed transfer
ssh source_server 'reservoir export | gzip' | gunzip | reservoir import /dev/stdin
```

## Troubleshooting

### Common Issues

**Import Failures**
```bash
# Check JSON validity
jq . import_file.json

# Verify required fields
jq '.[] | keys' import_file.json | head -5

# Check for duplicate trace_ids
jq -r '.[] | .trace_id' import_file.json | sort | uniq -d
```

**Missing Embeddings**
```bash
# Check embedding status
reservoir export | jq '[.[] | select(.embedding | length == 0)] | length'

# Regenerate embeddings if needed
reservoir replay
```

**Partition Issues**
```bash
# Check partition consistency
reservoir export | jq -r '.[] | "\(.partition)/\(.instance)"' | sort | uniq -c

# View messages in specific partition
reservoir view --partition problematic_partition 10
```

### Recovery Procedures

**Partial Import Recovery**
```bash
# If import fails partway through, check what was imported
IMPORTED_COUNT=$(reservoir export | jq length)
TOTAL_COUNT=$(jq length backup.json)
echo "Imported $IMPORTED_COUNT of $TOTAL_COUNT messages"

# Import remaining messages (requires identifying what's missing)
```

**Data Corruption Recovery**
```bash
# Export current state
reservoir export > current_state.json

# Restore from known good backup
reservoir import good_backup.json

# Compare and merge if needed
```

The import/export system provides a robust foundation for data management, enabling seamless backup, migration, and integration workflows while maintaining complete data fidelity and system integrity.
