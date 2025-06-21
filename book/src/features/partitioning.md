# Partitioning & Organization

Reservoir uses a flexible partitioning system to organize your conversations and data. This two-level hierarchy enables you to separate different contexts, users, projects, or topics while maintaining intelligent context enrichment within each boundary.

## Partitioning Concepts

### Two-Level Hierarchy

Reservoir organizes data using two levels:

1. **Partition**: The top-level organizational boundary
2. **Instance**: The sub-level within each partition

```
partition_name/
├── instance_1/
├── instance_2/
└── instance_3/
```

### Default Organization

When no partition is specified, Reservoir uses:
- **Partition**: `"default"`
- **Instance**: `"default"`

```bash
# These are equivalent
reservoir view 10
reservoir view --partition default --instance default 10
```

## Partition Use Cases

### User Separation

Separate different users or personas:

```
alice/
├── personal/      # Personal conversations
├── work/         # Work-related discussions  
└── research/     # Research and learning

bob/
├── coding/       # Programming discussions
├── writing/      # Content creation
└── planning/     # Project planning
```

**Usage Examples:**
```bash
# Alice's personal conversations
echo "What's the weather like?" | reservoir ingest --partition alice --instance personal

# Bob's coding discussions
echo "How do I implement OAuth2?" | reservoir ingest --partition bob --instance coding

# View Alice's work conversations
reservoir view --partition alice --instance work 15

# Search Bob's coding history
reservoir search --partition bob --instance coding --semantic "database optimization"
```

### Project Organization

Organize by projects or domains:

```
webapp_project/
├── backend/      # Backend development
├── frontend/     # Frontend development
├── database/     # Database design
└── deployment/   # DevOps and deployment

mobile_app/
├── ios/          # iOS development
├── android/      # Android development
├── api/          # API integration
└── testing/      # QA and testing
```

**Usage Examples:**
```bash
# Backend development discussions
echo "Should we use microservices or monolith?" | reservoir ingest --partition webapp_project --instance backend

# Mobile API integration
echo "API authentication best practices" | reservoir ingest --partition mobile_app --instance api

# Search across web project
reservoir search --partition webapp_project --semantic "authentication"

# View mobile testing discussions
reservoir view --partition mobile_app --instance testing 20
```

### Team Collaboration

Organize by teams or functional areas:

```
engineering/
├── architecture/  # System architecture
├── reviews/      # Code reviews
├── planning/     # Sprint planning
└── incidents/    # Incident response

product/
├── requirements/ # Requirements gathering
├── research/     # User research
├── roadmap/      # Product roadmap
└── metrics/      # Analytics and metrics
```

**Usage Examples:**
```bash
# Architecture discussions
echo "Microservices vs serverless trade-offs" | reservoir ingest --partition engineering --instance architecture

# Product research notes
echo "User feedback on new feature" | reservoir ingest --partition product --instance research

# Search engineering incidents
reservoir search --partition engineering --instance incidents "database"

# View product roadmap discussions
reservoir view --partition product --instance roadmap 10
```

## Context Isolation

### How Partitioning Affects Context

Reservoir's context enrichment respects partition boundaries:

1. **Same Partition/Instance**: Full context sharing
2. **Same Partition, Different Instance**: Limited context sharing
3. **Different Partition**: Complete isolation

**Context Rules:**
```bash
# These will share context with each other
reservoir ingest --partition alice --instance coding "How do I use async/await?"
reservoir ingest --partition alice --instance coding "What about error handling?"

# This will have separate context
reservoir ingest --partition alice --instance personal "What should I cook for dinner?"

# This will be completely isolated
reservoir ingest --partition bob --instance coding "How do I use async/await?"
```

### Privacy and Separation

Partitions provide data privacy:

- **Search Isolation**: Searches are scoped to partitions
- **Context Isolation**: AI responses don't leak across partitions
- **Export Control**: Can selectively export partition data
- **Access Control**: Enables future per-partition access controls

## Partition Management

### Creating Partitions

Partitions are created automatically when first used:

```bash
# Creates "newproject" partition with "planning" instance
echo "Project kickoff meeting notes" | reservoir ingest --partition newproject --instance planning
```

### Viewing Partition Data

```bash
# View messages from specific partition/instance
reservoir view --partition alice --instance coding 15

# View without specifying instance (shows from all instances in partition)
reservoir view --partition alice 25

# Search within partition
reservoir search --partition engineering --semantic "deployment strategy"

# Search within specific instance
reservoir search --partition engineering --instance architecture --semantic "microservices"
```

### Partition Listing

Currently, there's no direct command to list all partitions, but you can discover them through data export and analysis:

```bash
# Export and analyze partition distribution
reservoir export | jq -r '.[] | .partition' | sort | uniq -c | sort -nr

# Find all instances within a partition
reservoir export | jq -r '.[] | select(.partition=="alice") | .instance' | sort | uniq -c
```

## Advanced Partitioning Strategies

### Time-Based Partitioning

Organize by time periods:

```
conversations_2024/
├── january/
├── february/
└── march/

conversations_2023/
├── q1/
├── q2/
├── q3/
└── q4/
```

```bash
# Current month's discussions
MONTH=$(date +%B | tr '[:upper:]' '[:lower:]')
echo "Today's important insight" | reservoir ingest --partition conversations_2024 --instance $MONTH
```

### Topic-Based Partitioning

Organize by subject matter:

```
machine_learning/
├── theory/       # Theoretical discussions
├── implementation/ # Code and implementation
├── papers/       # Research papers
└── experiments/  # Experimental results

web_development/
├── frontend/     # Frontend technologies
├── backend/      # Backend systems
├── databases/    # Database design
└── devops/       # Operations and deployment
```

### Environment-Based Partitioning

Separate by environment or context:

```
development/
├── local/        # Local development
├── testing/      # Testing environment
├── staging/      # Staging discussions
└── production/   # Production issues

personal/
├── learning/     # Educational content
├── projects/     # Personal projects
├── notes/        # General notes
└── ideas/        # Ideas and brainstorming
```

## Best Practices

### Naming Conventions

1. **Use Lowercase**: Partition and instance names should be lowercase
2. **Use Underscores**: Separate words with underscores: `machine_learning`
3. **Be Descriptive**: Choose clear, meaningful names
4. **Keep Consistent**: Maintain consistent naming across partitions

```bash
# Good naming
reservoir ingest --partition web_development --instance frontend
reservoir ingest --partition machine_learning --instance deep_learning

# Avoid these patterns
reservoir ingest --partition WebDev --instance FE  # Mixed case, abbreviated
reservoir ingest --partition "web development" --instance "front end"  # Spaces
```

### Partition Strategy

1. **Plan Your Structure**: Design partition hierarchy before heavy usage
2. **Balance Granularity**: Too many partitions reduce context benefits
3. **Consider Growth**: Design for future expansion
4. **Document Structure**: Keep a record of partition purposes

### Migration Between Partitions

Currently, partition migration requires export/import workflow:

```bash
# Export messages from one partition
reservoir export | jq '.[] | select(.partition=="old_partition")' > old_partition.json

# Edit JSON to change partition/instance names
sed 's/"partition":"old_partition"/"partition":"new_partition"/g' old_partition.json > new_partition.json

# Import to new structure
reservoir import new_partition.json

# Verify migration
reservoir view --partition new_partition 10
```

## Integration with Other Features

### Search Scoping

All search operations can be scoped to partitions:

```bash
# Search across all data
reservoir search --semantic "error handling"

# Search within partition  
reservoir search --partition engineering --semantic "error handling"

# Search within specific instance
reservoir search --partition engineering --instance backend --semantic "error handling"
```

### Data Export

Partitioning enables selective data export:

```bash
# Export everything
reservoir export > all_data.json

# Export specific partition (requires jq processing)
reservoir export | jq '.[] | select(.partition=="alice")' > alice_data.json

# Export specific instance
reservoir export | jq '.[] | select(.partition=="alice" and .instance=="coding")' > alice_coding.json
```

### Context Enrichment

Partitioning directly affects how context is built:

- **Semantic Search**: Limited to same partition/instance
- **Recent History**: Limited to same partition/instance  
- **Synapse Relationships**: Respect partition boundaries
- **Token Limits**: Applied per partition context

This partitioning system makes Reservoir suitable for multi-user environments, project-based work, and any scenario where logical separation of conversation contexts is beneficial.
