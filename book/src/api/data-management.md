# Data Management

Reservoir provides comprehensive data management capabilities for backing up, migrating, and organizing your conversation data. The system supports full data export/import, individual message management, and flexible partitioning strategies.

## Export and Import

### Export All Data

Export your entire conversation history as JSON for backup or migration:

```bash
# Export all messages to stdout
reservoir export

# Save to file with timestamp
reservoir export > backup_$(date +%Y%m%d_%H%M%S).json

# Export and compress for storage
reservoir export | gzip > reservoir_backup.json.gz
```

**Export Format:**
Each message is exported as a complete MessageNode with all metadata:

```json
[
  {
    "trace_id": "550e8400-e29b-41d4-a716-446655440000",
    "partition": "default",
    "instance": "default", 
    "role": "user",
    "content": "How do I implement error handling in async functions?",
    "timestamp": "2024-01-15T10:30:00.000Z",
    "embedding": [0.123, -0.456, 0.789, ...],
    "url": null
  },
  {
    "trace_id": "550e8400-e29b-41d4-a716-446655440001",
    "partition": "default",
    "instance": "default",
    "role": "assistant", 
    "content": "Here are several approaches to error handling in async functions...",
    "timestamp": "2024-01-15T10:30:15.000Z",
    "embedding": [0.234, -0.567, 0.890, ...],
    "url": null
  }
]
```

### Import Data

Import message data from JSON files:

```bash
# Import from a backup file
reservoir import backup_20240115.json

# Import from another Reservoir instance
reservoir import exported_conversations.json

# Import compressed backup
gunzip -c reservoir_backup.json.gz | reservoir import /dev/stdin
```

**Import Behavior:**
- Validates JSON format and MessageNode structure
- Preserves all metadata including timestamps and embeddings
- Maintains partition/instance organization
- Skips duplicate messages (based on trace_id)
- Rebuilds relationships and synapses

### Migration Workflows

**Complete System Migration:**
```bash
# On source system
reservoir export > full_backup.json

# Transfer file to new system
scp full_backup.json user@newserver:/path/to/reservoir/

# On destination system
reservoir import full_backup.json

# Verify migration
reservoir view 10
reservoir search --semantic "test query"
```

**Selective Migration:**
```bash
# Export from specific partition
reservoir export | jq '.[] | select(.partition=="alice")' > alice_messages.json

# Import to different partition (requires manual editing or processing)
# Edit JSON to change partition names, then import
reservoir import alice_messages.json
```

## Message Management

### Manual Message Ingestion

Add messages manually for testing, note-taking, or data entry:

```bash
# Add a user message
echo "How do I configure Neo4j for production?" | reservoir ingest

# Add to specific partition/instance
echo "Remember to update dependencies" | reservoir ingest --partition alice --instance notes

# Add assistant message
echo "Here's the production Neo4j configuration..." | reservoir ingest --role assistant

# Ingest from file
cat meeting_notes.txt | reservoir ingest --partition team --instance meetings
```

**Use Cases:**
- **Documentation**: Add important information manually
- **Testing**: Create test scenarios with known data
- **Migration**: Import data from other systems
- **Notes**: Add personal reminders or observations

### Viewing Recent Data

Monitor recent activity and verify data integrity:

```bash
# View last 10 messages
reservoir view 10

# View from specific partition
reservoir view --partition alice 15

# View from specific instance
reservoir view --partition alice --instance coding 20

# Pipe to other tools for analysis
reservoir view 50 | grep -i "error" | wc -l
```

## Partitioning Strategy

### Organizational Structure

Reservoir uses a two-level organizational hierarchy:

1. **Partition**: High-level boundary (user, project, team)
2. **Instance**: Sub-boundary within partition (topic, session, category)

```
default/
├── default/          # General conversations
├── coding/           # Programming discussions  
└── research/         # Research and analysis

alice/
├── personal/         # Personal conversations
├── work/            # Work-related discussions
└── learning/        # Educational content

team/
├── meetings/        # Team meeting notes
├── planning/        # Project planning
└── retrospectives/  # Review sessions
```

### Partition Management

**Creating Partitions:**
Partitions are created automatically when first used:

```bash
# Create new partition by using it
echo "Starting new project discussions" | reservoir ingest --partition newproject

# Create instance within partition
echo "Technical architecture discussion" | reservoir ingest --partition newproject --instance architecture
```

**Partition Benefits:**
- **Isolation**: Keep different contexts separate
- **Search Scoping**: Limit searches to relevant content
- **Access Control**: Enable future access restrictions
- **Organization**: Maintain clean separation of concerns

### Data Isolation

Partitions provide logical isolation:

- **Context Enrichment**: Only includes messages from same partition/instance
- **Search**: Can be scoped to specific partitions
- **Export**: Can filter by partition (with additional tooling)
- **Privacy**: Enables separation of personal/professional content

## Data Integrity

### Backup Strategies

**Daily Backups:**
```bash
#!/bin/bash
# Daily backup script

BACKUP_DIR="/backup/reservoir"
DATE=$(date +%Y%m%d)
TIMESTAMP=$(date +%H%M%S)

# Create backup directory
mkdir -p "$BACKUP_DIR/$DATE"

# Export data
reservoir export > "$BACKUP_DIR/$DATE/reservoir_$TIMESTAMP.json"

# Compress older backups
find "$BACKUP_DIR" -name "*.json" -mtime +7 -exec gzip {} \;

# Clean old backups (keep 30 days)
find "$BACKUP_DIR" -name "*.json.gz" -mtime +30 -delete

# Log backup
echo "$(date): Backup completed - $BACKUP_DIR/$DATE/reservoir_$TIMESTAMP.json" >> /var/log/reservoir_backup.log
```

**Incremental Exports:**
```bash
# Export recent messages (last 24 hours)
reservoir view 1000 | jq -r '.[] | select(.timestamp > "'$(date -d '1 day ago' -Iseconds)'")' > incremental_backup.json
```

### Data Validation

**Verify Data Integrity:**
```bash
# Check message count
TOTAL_MESSAGES=$(reservoir export | jq length)
echo "Total messages: $TOTAL_MESSAGES"

# Verify embeddings
EMBEDDED_COUNT=$(reservoir export | jq '[.[] | select(.embedding != null)] | length')
echo "Messages with embeddings: $EMBEDDED_COUNT"

# Check partition distribution
reservoir export | jq -r '.[] | .partition' | sort | uniq -c
```

### Recovery Procedures

**Restore from Backup:**
```bash
# Stop Reservoir (if running as service)
systemctl stop reservoir

# Clear existing data (WARNING: destructive)
# This requires manual Neo4j database clearing

# Import backup
reservoir import /backup/reservoir/20240115/reservoir_full.json

# Verify restoration
reservoir view 10
reservoir search --semantic "test"

# Restart service
systemctl start reservoir
```

## Advanced Data Operations

### Data Analysis

**Export for Analysis:**
```bash
# Export specific fields for analysis
reservoir export | jq -r '.[] | [.timestamp, .partition, .role, (.content | length)] | @csv' > message_stats.csv

# Analyze conversation patterns
reservoir export | jq -r '.[] | .partition' | sort | uniq -c | sort -nr

# Find most active time periods
reservoir export | jq -r '.[] | .timestamp[0:10]' | sort | uniq -c | sort -nr
```

### Data Transformation

**Format Conversion:**
```bash
# Convert to CSV format
reservoir export | jq -r '.[] | [.timestamp, .partition, .instance, .role, .content] | @csv' > conversations.csv

# Extract just message content
reservoir export | jq -r '.[] | .content' > all_messages.txt

# Create markdown format
reservoir export | jq -r '.[] | "## " + .timestamp + " (" + .role + ")\n\n" + .content + "\n"' > conversations.md
```

### Embedding Management

**Replay Embeddings:**
When embedding models change or for data recovery:

```bash
# Replay embeddings for all messages
reservoir replay

# Replay for specific model/partition
reservoir replay bge-large-en-v15

# Monitor embedding progress
# (Check logs for embedding generation status)
tail -f /var/log/reservoir.log | grep -i embedding
```

## Best Practices

### Regular Maintenance

1. **Schedule Regular Backups**: Daily exports with compression
2. **Monitor Disk Usage**: Embeddings require significant storage
3. **Validate Data Integrity**: Regular checks for missing embeddings
4. **Clean Old Logs**: Rotate and archive log files
5. **Test Recovery**: Periodically test backup restoration

### Storage Optimization

1. **Compress Backups**: Use gzip for long-term storage
2. **Archive Old Data**: Move historical data to cold storage
3. **Monitor Neo4j Storage**: Regular database maintenance
4. **Embedding Efficiency**: Consider embedding model size vs. quality

### Security Considerations

1. **Encrypt Backups**: Sensitive conversation data should be encrypted
2. **Access Controls**: Limit access to export/import capabilities
3. **Audit Trails**: Log all data management operations
4. **Data Retention**: Define policies for data lifecycle management

Data management in Reservoir is designed to be straightforward while providing enterprise-grade capabilities for backup, migration, and organization of your conversation data.
