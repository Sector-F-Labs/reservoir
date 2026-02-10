use std::future::Future;

use anyhow::Error;
use neo4rs::{query, Graph};
use tracing::info;

use crate::{
    clients::embedding::EmbeddingInfo,
    models::message_node::MessageNode,
    utils::{connector::{connect, AsyncGraphFuture}, deduplicate_message_nodes},
};

pub async fn get_messages_for_partition<C, FutC>(
    get_connector: C,
    partition: Option<&str>,
) -> Result<Vec<MessageNode>, Error>
where
    FutC: Future<Output = Result<Graph, Error>>,
    C: Fn() -> FutC,
{
    let graph = get_connector().await?;
    let q = if let Some(p) = partition {
        query("MATCH (m:MessageNode {partition: $partition}) RETURN id(m) AS id, m")
            .param("partition", p)
    } else {
        query("MATCH (m:MessageNode) RETURN id(m) AS id, m")
    };

    let mut result = graph.execute(q).await?;
    let mut messages = Vec::new();

    while let Some(row) = result.next().await? {
        // First, extract the MessageNode
        let mut node: MessageNode = row.get("m")?;
        // Then, override its id field with the database id
        node.id = Some(row.get::<i64>("id")?);
        messages.push(node);
    }

    Ok(messages)
}

pub async fn get_last_messages_for_partition_and_instance<C, FutC>(
    get_connector: C,
    partition: String,
    instance: String,
    count: usize,
) -> Result<Vec<MessageNode>, Error>
where
    FutC: Future<Output = Result<Graph, Error>>,
    C: Fn() -> FutC,
{
    let graph = get_connector().await?;
    let q = format!(
            "MATCH (m:MessageNode {{partition: '{}', instance: '{}'}}) RETURN m ORDER BY m.timestamp DESC LIMIT {}",
            partition, instance, count
        );
    let mut result = graph.execute(query(q.as_str())).await?;
    let mut messages = Vec::new();
    while let Some(row) = result.next().await? {
        let node: MessageNode = row.get("m")?;
        messages.push(node);
    }
    Ok(messages)
}
pub async fn get_messages<C, FutC>(get_connector: C) -> Result<Vec<MessageNode>, Error>
where
    FutC: Future<Output = Result<Graph, Error>>,
    C: Fn() -> FutC,
{
    let graph = get_connector().await?;
    let q = "MATCH (m:MessageNode) RETURN m, id(m) as id";
    let mut result = graph.execute(query(q)).await?;
    let mut messages = Vec::new();
    while let Some(row) = result.next().await? {
        // First, extract the MessageNode
        let mut node: MessageNode = row.get("m")?;
        // Then, override its id field with the database id
        node.id = Some(row.get::<i64>("id")?);
        messages.push(node);
    }
    Ok(messages)
}

pub async fn get_messages_for_embedding_nodes<C, FutC>(
    get_connector: C,
    embedding_nodes: Vec<i64>,
    embedding_info: &EmbeddingInfo,
) -> Result<Vec<MessageNode>, Error>
where
    FutC: Future<Output = Result<Graph, Error>>,
    C: Fn() -> FutC,
{
    let graph = get_connector().await?;
    let query_string = format!(
        r#"
            MATCH (e:{})-[:HAS_EMBEDDING]-(m:MessageNode)
            WHERE id(e) IN $embedding_nodes
            RETURN m
            "#,
        embedding_info.get_node_name()
    );
    let q = query(query_string.as_str()).param("embedding_nodes", embedding_nodes);

    let mut result = graph.execute(q).await?;
    let mut messages = Vec::new();
    while let Some(row) = result.next().await? {
        let node: MessageNode = row.get("m")?;
        messages.push(node);
    }
    Ok(messages)
}

pub async fn save_message_node<C, FutC>(
    get_connector: C,
    message_node: &MessageNode,
    embedding_info: &EmbeddingInfo,
) -> Result<(), Error>
where
    C: Fn() -> FutC,
    FutC: AsyncGraphFuture,
{
    if message_node.role.eq_ignore_ascii_case("system") {
        return Ok(());
    }

    let graph = get_connector().await?;
    let query_string = format!(
        r#"
            CREATE (m:MessageNode {{
                trace_id: $trace_id,
                content: $content,
                role: $role,
                timestamp: $timestamp,
                partition: $partition,
                instance: $instance,
                embedding: $embedding,
                url: $url
            }})
            CREATE (e:{} {{
                model: '{}',
                embedding: $embedding,
                partition: $partition,
                instance: $instance
            }})
            CREATE (m)-[:HAS_EMBEDDING]->(e)
            RETURN id(m) AS nodeId, id(e) AS embeddingId
            "#,
        embedding_info.get_node_name(),
        embedding_info.get_model_name()
    );
    let create_q = query(query_string.as_str())
        .param("trace_id", message_node.trace_id.clone())
        .param("content", message_node.content.clone())
        .param("timestamp", message_node.timestamp)
        .param("role", message_node.role.clone())
        .param("partition", message_node.partition.clone())
        .param("instance", message_node.instance.clone())
        .param("embedding", message_node.embedding.clone())
        .param("url", message_node.url.clone());

    // Execute the CREATE query
    let mut create_result = graph.execute(create_q).await?;
    // Consume the result to ensure the node is created before potentially linking it
    let _ = create_result.next().await?;

    // If the saved message is an assistant message, try to link it to the corresponding user message
    if message_node.role.eq_ignore_ascii_case("assistant") {
        let link_q = query(
            r#"MATCH (u:MessageNode {role: 'user', trace_id: $trace_id})
                   MATCH (a:MessageNode {role: 'assistant', trace_id: $trace_id})
                   MERGE (u)-[:RESPONDED_WITH]->(a)
                   RETURN count(*)"#,
        )
        .param("trace_id", message_node.trace_id.clone());

        // Execute the MERGE query
        let mut link_result = graph.execute(link_q).await?;
        // Consume the result
        let _ = link_result.next().await?;
    }

    // Create synapse to previous message if similarity is above threshold
    if let Some(prev) = get_previous_message(
        get_connector,
        &message_node.partition,
        &message_node.instance,
        message_node.timestamp,
    )
    .await?
    {
        let _ = maybe_create_synapse(
            connect,
            prev.timestamp,
            message_node.timestamp,
            &message_node.partition,
            &message_node.instance,
        )
        .await?;
    }

    Ok(())
}

async fn create_index_for_node<C, FutC>(get_connector: C, size: usize) -> Result<(), Error>
where
    C: Fn() -> FutC,
    FutC: AsyncGraphFuture,
{
    let index_name = format!("embedding{}", size);
    let node_name = format!("Embedding{}", size);
    let query_str = format!(
        r#"
            CREATE VECTOR INDEX {} 
            FOR (n:{})
            ON (n.embedding)
            OPTIONS {{
              indexConfig: {{
                `vector.dimensions`: {},
                `vector.similarity_function`: 'cosine'
              }}
            }};
        "#,
        index_name, node_name, size
    );
    let graph = get_connector().await?;
    let mut result = graph.execute(query(&query_str)).await?;
    while let Ok(Some(row)) = result.next().await {
        let message: String = row.get("message")?;
        info!("Index creation message: {}", message);
    }
    info!("Created index: {}", index_name);
    Ok(())
}

async fn create_index_with_size<C, FutC>(get_connector: C, size: usize) -> Result<(), Error>
where
    C: Fn() -> FutC,
    FutC: AsyncGraphFuture,
{
    let index_name = format!("embedding{}", size);
    let graph = get_connector().await?;
    // Check if index already exists
    let check_query = query("SHOW INDEXES YIELD name RETURN name");
    let mut result = graph.execute(check_query).await?;

    while let Ok(Some(row)) = result.next().await {
        let name: String = row.get("name")?;
        if name == index_name {
            // Index already exists, nothing to do
            return Ok(());
        }
    }

    // Create the index if it doesn't exist
    create_index_for_node(get_connector, size).await?;
    Ok(())
}

pub async fn init_vector_index() -> Result<(), Error> {
    let (res1, res2) = tokio::join!(
        create_index_with_size(connect, 1024),
        create_index_with_size(connect, 1536),
    );
    res1?;
    res2
}

/// Finds nodes connected to a given node within a distance of 10 hops.
/// Returns a vector of `MessageNode` instances representing the connected nodes.
/// The distance is defined by the number of hops in the graph.
const SYNAPSE_THRESHOLD: f64 = 0.85;

/// Minimum number of recent messages to always include in context,
/// regardless of synapse state. This ensures basic continuity even after
/// topic changes or "brb" interruptions.
const MIN_RECENT_MESSAGES: usize = 5;

/// Gets the most recent message before the given timestamp in the same partition/instance.
pub async fn get_previous_message<C, FutC>(
    get_connector: C,
    partition: &str,
    instance: &str,
    before_timestamp: i64,
) -> Result<Option<MessageNode>, Error>
where
    C: Fn() -> FutC,
    FutC: AsyncGraphFuture,
{
    let graph = get_connector().await?;
    let q = query(
        r#"
        MATCH (m:MessageNode {partition: $partition, instance: $instance})
        WHERE m.timestamp < $before_timestamp
        RETURN m
        ORDER BY m.timestamp DESC
        LIMIT 1
        "#,
    )
    .param("partition", partition)
    .param("instance", instance)
    .param("before_timestamp", before_timestamp);

    let mut result = graph.execute(q).await?;
    if let Some(row) = result.next().await? {
        let node: MessageNode = row.get("m")?;
        Ok(Some(node))
    } else {
        Ok(None)
    }
}

/// Creates a SYNAPSE relationship between two messages if their similarity is above threshold.
/// Returns true if synapse was created, false if below threshold.
pub async fn maybe_create_synapse<C, FutC>(
    get_connector: C,
    from_timestamp: i64,
    to_timestamp: i64,
    partition: &str,
    instance: &str,
) -> Result<bool, Error>
where
    C: Fn() -> FutC,
    FutC: AsyncGraphFuture,
{
    let graph = get_connector().await?;
    let q = query(
        r#"
        MATCH (m1:MessageNode {partition: $partition, instance: $instance, timestamp: $from_ts})
        MATCH (m2:MessageNode {partition: $partition, instance: $instance, timestamp: $to_ts})
        WITH m1, m2, vector.similarity.cosine(m1.embedding, m2.embedding) AS score
        WHERE score >= $threshold
        CREATE (m1)-[:SYNAPSE {score: score, model: 'inline'}]->(m2)
        RETURN score
        "#,
    )
    .param("partition", partition)
    .param("instance", instance)
    .param("from_ts", from_timestamp)
    .param("to_ts", to_timestamp)
    .param("threshold", SYNAPSE_THRESHOLD);

    let mut result = graph.execute(q).await?;
    if let Some(row) = result.next().await? {
        let score: f64 = row.get("score")?;
        info!(
            "Created synapse: {} -> {} (score: {:.3})",
            from_timestamp, to_timestamp, score
        );
        Ok(true)
    } else {
        info!(
            "No synapse created: {} -> {} (below threshold {})",
            from_timestamp, to_timestamp, SYNAPSE_THRESHOLD
        );
        Ok(false)
    }
}

/// Gets the current thread by following SYNAPSE relationships backward from the most recent message.
/// Returns messages in chronological order (oldest first).
pub async fn get_thread_from_latest<C, FutC>(
    get_connector: C,
    partition: &str,
    instance: &str,
    max_count: usize,
) -> Result<Vec<MessageNode>, Error>
where
    C: Fn() -> FutC,
    FutC: AsyncGraphFuture,
{
    let graph = get_connector().await?;

    // Find the most recent message and follow synapses backward
    // Synapses point forward (old)-[:SYNAPSE]->(new), so we traverse in reverse
    let q = query(
        r#"
        MATCH (latest:MessageNode {partition: $partition, instance: $instance})
        WHERE NOT (latest)-[:SYNAPSE]->()
        WITH latest
        ORDER BY latest.timestamp DESC
        LIMIT 1

        MATCH path = (earliest)-[:SYNAPSE*0..]->(latest)
        WHERE earliest.partition = $partition AND earliest.instance = $instance
        WITH path
        ORDER BY length(path) DESC
        LIMIT 1

        UNWIND nodes(path) AS m
        RETURN m
        ORDER BY m.timestamp ASC
        "#,
    )
    .param("partition", partition)
    .param("instance", instance);

    let mut result = graph.execute(q).await?;
    let mut messages = Vec::new();

    while let Some(row) = result.next().await? {
        let node: MessageNode = row.get("m")?;
        messages.push(node);
        if messages.len() >= max_count {
            break;
        }
    }

    Ok(messages)
}

/// Gets hybrid context: synapse-connected thread merged with minimum recent messages.
/// This ensures continuity even when synapses break (topic changes, "brb", etc.).
/// Returns messages in chronological order (oldest first).
pub async fn get_context<C, FutC>(
    get_connector: C,
    partition: &str,
    instance: &str,
    max_count: usize,
) -> Result<Vec<MessageNode>, Error>
where
    C: Fn() -> FutC + Clone,
    FutC: AsyncGraphFuture,
{
    // Get the synapse-connected thread
    let thread = get_thread_from_latest(
        get_connector.clone(),
        partition,
        instance,
        max_count,
    )
    .await?;

    // Get the last N recent messages (returns DESC order, most recent first)
    let mut recent = get_last_messages_for_partition_and_instance(
        get_connector,
        partition.to_string(),
        instance.to_string(),
        MIN_RECENT_MESSAGES,
    )
    .await?;

    // Reverse recent to chronological order (oldest first)
    recent.reverse();

    // Merge thread and recent, deduplicate
    let mut combined = thread;
    combined.extend(recent);

    // Sort by timestamp to ensure chronological order
    combined.sort_by_key(|m| m.timestamp);

    // Deduplicate (removes messages with same content)
    let deduplicated = deduplicate_message_nodes(combined);

    // Respect max_count limit
    let result: Vec<MessageNode> = deduplicated.into_iter().take(max_count).collect();

    Ok(result)
}

pub async fn get_nodes_connected_by_synapses<C, FutC>(
    get_connector: C,
    node: &MessageNode,
) -> Result<Vec<MessageNode>, Error>
where
    C: Fn() -> FutC,
    FutC: AsyncGraphFuture,
{
    let graph = get_connector().await?;
    let q = r#"
            MATCH p=(m:MessageNode {trace_id: $trace_id})-[:SYNAPSE*1..10]-(n:MessageNode)
            RETURN nodes(p) AS allNodes
        "#;
    let mut result = graph
        .execute(query(q).param("trace_id", node.trace_id.clone()))
        .await?;
    let mut connected_nodes = Vec::new();
    while let Ok(Some(row)) = result.next().await {
        let nodes: Vec<MessageNode> = row.get("allNodes")?;
        connected_nodes.extend(nodes);
    }
    Ok(connected_nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synapse_threshold_is_reasonable() {
        // Threshold should be between 0.5 (very lenient) and 0.95 (very strict)
        assert!(SYNAPSE_THRESHOLD >= 0.5, "threshold too low");
        assert!(SYNAPSE_THRESHOLD <= 0.95, "threshold too high");
        // Current value is 0.85 - a reasonable default for topic continuity
        assert_eq!(SYNAPSE_THRESHOLD, 0.85);
    }

    #[test]
    fn synapse_threshold_matches_legacy_pruning() {
        // The legacy connect_synapses() uses 0.85 hardcoded in the Cypher query
        // Our new per-message threshold should match
        // If this test fails, update either SYNAPSE_THRESHOLD or the legacy query
        assert_eq!(SYNAPSE_THRESHOLD, 0.85);
    }

    /// Helper to check if similarity would create a synapse
    fn would_create_synapse(similarity: f64) -> bool {
        similarity >= SYNAPSE_THRESHOLD
    }

    // Integration tests using real FastEmbed embeddings
    // These are slower but test actual semantic similarity behavior
    mod embedding_integration {
        use super::*;
        use crate::clients::embedding::{get_embeddings_for_txt, EmbeddingInfo};
        use crate::utils::cosine_similarity;

        async fn get_embedding(text: &str) -> Vec<f32> {
            let info = EmbeddingInfo::with_fastembed("bge-large-en-v15");
            get_embeddings_for_txt(text, info).await.unwrap()
        }

        #[tokio::test]
        async fn similar_topic_messages_create_synapse() {
            // Two messages about the same topic should have high similarity
            let emb1 = get_embedding("How do I set up Neo4j on Ubuntu?").await;
            let emb2 = get_embedding("What's the configuration for Neo4j?").await;

            let similarity = cosine_similarity(&emb1, &emb2);
            println!("Similar topic similarity: {:.3}", similarity);

            assert!(
                would_create_synapse(similarity),
                "Related Neo4j messages should create synapse (similarity: {:.3})",
                similarity
            );
        }

        #[tokio::test]
        async fn unrelated_topic_messages_no_synapse() {
            // Completely unrelated topics should have low similarity
            let emb1 = get_embedding("How do I set up Neo4j on Ubuntu?").await;
            let emb2 = get_embedding("What's the weather like in Stockholm today?").await;

            let similarity = cosine_similarity(&emb1, &emb2);
            println!("Unrelated topic similarity: {:.3}", similarity);

            assert!(
                !would_create_synapse(similarity),
                "Unrelated messages should NOT create synapse (similarity: {:.3})",
                similarity
            );
        }

        #[tokio::test]
        async fn followup_question_may_not_connect() {
            // A followup that references "that config file" doesn't share enough
            // semantic content with the original statement
            let emb1 = get_embedding("The config file is at /etc/neo4j/neo4j.conf").await;
            let emb2 = get_embedding("How do I change the password in that config file?").await;

            let similarity = cosine_similarity(&emb1, &emb2);
            println!("Followup question similarity: {:.3}", similarity);

            // At 0.85 threshold, this doesn't connect (~0.63)
            // This documents a limitation: pronoun references don't create semantic similarity
            assert!(
                similarity > 0.5 && similarity < SYNAPSE_THRESHOLD,
                "Followup with pronoun reference has moderate similarity: {:.3}",
                similarity
            );
        }

        #[tokio::test]
        async fn configuration_questions_similarity() {
            // Even explicit Neo4j config questions may not reach 0.85
            let emb1 = get_embedding("How do I configure Neo4j on Ubuntu?").await;
            let emb2 = get_embedding("What Neo4j configuration settings control memory?").await;

            let similarity = cosine_similarity(&emb1, &emb2);
            println!("Configuration questions similarity: {:.3}", similarity);

            // At 0.85 threshold, these don't connect (~0.78)
            // Documents that 0.85 may be too strict for natural conversation flow
            assert!(
                similarity > 0.7,
                "Related config questions should have reasonable similarity: {:.3}",
                similarity
            );
        }

        #[tokio::test]
        async fn topic_change_breaks_chain() {
            // Simulates: discussing Rust, then asking about dinner
            let emb1 = get_embedding("Rust's ownership system prevents memory leaks").await;
            let emb2 = get_embedding("What should I make for dinner tonight?").await;

            let similarity = cosine_similarity(&emb1, &emb2);
            println!("Topic change similarity: {:.3}", similarity);

            assert!(
                !would_create_synapse(similarity),
                "Topic change should NOT create synapse (similarity: {:.3})",
                similarity
            );
        }

        #[tokio::test]
        async fn gradual_drift_may_break_chain() {
            // Generic "databases" to specific "graph databases" doesn't share enough content
            let emb1 = get_embedding("Tell me about databases").await;
            let emb2 = get_embedding("What about graph databases specifically?").await;
            let emb3 = get_embedding("How do graph databases use edges?").await;

            let sim_1_2 = cosine_similarity(&emb1, &emb2);
            let sim_2_3 = cosine_similarity(&emb2, &emb3);
            println!("Drift step 1->2 similarity: {:.3}", sim_1_2);
            println!("Drift step 2->3 similarity: {:.3}", sim_2_3);

            // At 0.85 threshold: generic->specific breaks, specific->specific connects
            // This is actually reasonable: "databases" is too generic
            assert!(
                sim_1_2 < SYNAPSE_THRESHOLD,
                "Generic 'databases' -> specific 'graph databases' may not connect (similarity: {:.3})",
                sim_1_2
            );
            assert!(
                would_create_synapse(sim_2_3),
                "Specific 'graph databases' -> 'edges' should connect (similarity: {:.3})",
                sim_2_3
            );
        }

        #[tokio::test]
        async fn neo4j_topic_similarity_varies() {
            // Even within the same general topic, similarity varies
            let emb1 = get_embedding("How do I create nodes in Neo4j?").await;
            let emb2 = get_embedding("How do I create relationships between Neo4j nodes?").await;
            let emb3 = get_embedding("What properties can Neo4j nodes have?").await;

            let sim_1_2 = cosine_similarity(&emb1, &emb2);
            let sim_2_3 = cosine_similarity(&emb2, &emb3);
            println!("Neo4j: create nodes -> relationships: {:.3}", sim_1_2);
            println!("Neo4j: relationships -> properties: {:.3}", sim_2_3);

            // nodes -> relationships: ~0.90 (connected!)
            // relationships -> properties: ~0.78 (not connected at 0.85)
            // This shows 0.85 can break chains even within same topic
            assert!(
                would_create_synapse(sim_1_2),
                "create nodes -> relationships should connect (similarity: {:.3})",
                sim_1_2
            );
            assert!(
                sim_2_3 > 0.7 && sim_2_3 < SYNAPSE_THRESHOLD,
                "relationships -> properties has moderate similarity: {:.3}",
                sim_2_3
            );
        }

        #[tokio::test]
        async fn low_content_message_breaks_chain() {
            // Short/low-content messages like "brb" often break chains
            let emb1 = get_embedding("Rust's ownership system prevents memory leaks").await;
            let emb2 = get_embedding("brb").await;

            let similarity = cosine_similarity(&emb1, &emb2);
            println!("Low content 'brb' similarity: {:.3}", similarity);

            // This documents current behavior - low content messages break chains
            // We might want to handle this differently in the future
            assert!(
                !would_create_synapse(similarity),
                "Low content 'brb' should NOT create synapse (similarity: {:.3})",
                similarity
            );
        }
    }
}
