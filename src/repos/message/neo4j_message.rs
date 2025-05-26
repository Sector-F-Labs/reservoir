use std::future::Future;

use anyhow::Error;
use neo4rs::{query, Graph};
use tracing::{error, info};

use crate::{
    clients::embedding::EmbeddingInfo, models::message_node::MessageNode,
    utils::connector::AsyncGraphFuture,
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

    Ok(())
}

pub async fn init_vector_index<C, FutC>(get_connector: C) -> Result<(), Error>
where
    C: Fn() -> FutC,
    FutC: AsyncGraphFuture,
{
    let index_name = "messageEmbeddings";
    let emneddings_index_name = "embeddingEmbeddings";
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
    let create_query = format!(
        "CALL db.index.vector.createNodeIndex(
                '{}',
                'MessageNode',
                'embedding',
                1536,
                'cosine'
            );
            CALL db.index.vector.createNodeIndex(
                '{}',
                'Embedding',
                'embedding',
                1536,
                'cosine'
            )",
        index_name, emneddings_index_name
    );
    let result = graph.execute(query(&create_query)).await;
    match result {
        Ok(mut rows) => {
            while let Ok(Some(row)) = rows.next().await {
                let name: String = row.get("name")?;
                if name == index_name {
                    info!("Index {} created successfully", index_name);
                }
            }
        }
        Err(e) => {
            // Check if it's the "equivalent index already exists" error and suppress it
            if format!("{:?}", e).contains("EquivalentSchemaRuleAlreadyExistsException") {
                info!("Index '{}' already exists, skipping creation", index_name);
            } else {
                Err(e)?;
            }
        }
    }
    Ok(())
}

/// Finds nodes connected to a given node within a distance of 10 hops.
/// Returns a vector of `MessageNode` instances representing the connected nodes.
/// The distance is defined by the number of hops in the graph.
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

pub async fn connect_synapses<C, FutC>(get_connector: C) -> Result<(), Error>
where
    C: Fn() -> FutC,
    FutC: AsyncGraphFuture,
{
    let graph = get_connector().await?;
    let q = r#"
            MATCH (m:MessageNode)-[:HAS_EMBEDDING]->(e:Embedding1024)
            WHERE e.embedding IS NOT NULL

            WITH m, e.embedding AS messageEmbedding
            ORDER BY m.timestamp ASC

            WITH collect({message: m, embedding: messageEmbedding}) AS ordered_message_data

            UNWIND range(0, size(ordered_message_data) - 2) AS i
            WITH ordered_message_data[i] AS data1, ordered_message_data[i+1] AS data2

            WITH data1.message AS m1, data1.embedding AS emb1,
                 data2.message AS m2, data2.embedding AS emb2

            MERGE (m1)-[s:SYNAPSE]->(m2)
            ON CREATE SET s.score = vector.similarity.cosine(emb1, emb2),
                          s.model = 'embedding1536' 
            ON MATCH SET s.score = vector.similarity.cosine(emb1, emb2);
        "#;
    let mut result = graph.execute(query(q)).await?;
    while let Ok(Some(row)) = result.next().await {
        let node: MessageNode = row.get("m")?;
        info!("Connected nodes: {:?}", node);
    }
    let q = r#"
            MATCH (m1:MessageNode)-[r:SYNAPSE]->(m2:MessageNode)
            WHERE r.score < 0.85
            DELETE r
        "#;
    let mut result = graph.execute(query(q)).await?;
    while let Ok(Some(row)) = result.next().await {
        let node: MessageNode = row.get("m")?;
        error!("Deleted synapse: {:?}", node);
    }
    Ok(())
}
