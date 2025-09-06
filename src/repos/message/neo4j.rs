use anyhow::Error;
use async_trait::async_trait;
use neo4rs::query;
use tracing::{error, info};

use crate::{
    clients::embedding::EmbeddingInfo, models::message_node::MessageNode,
    repos::traits::MessageRepository, utils::connector::connect,
};

pub struct Neo4jMessageRepository;

impl Neo4jMessageRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageRepository for Neo4jMessageRepository {
    async fn get_messages_for_partition(
        &self,
        partition: Option<&str>,
    ) -> Result<Vec<MessageNode>, Error> {
        let graph = connect().await?;
        let q = if let Some(p) = partition {
            query("MATCH (m:MessageNode {partition: $partition}) RETURN id(m) AS id, m")
                .param("partition", p)
        } else {
            query("MATCH (m:MessageNode) RETURN id(m) AS id, m")
        };

        let mut result = graph.execute(q).await?;
        let mut messages = Vec::new();

        while let Some(row) = result.next().await? {
            let mut node: MessageNode = row.get("m")?;
            node.id = Some(row.get::<i64>("id")?);
            messages.push(node);
        }

        Ok(messages)
    }

    async fn get_messages(&self) -> Result<Vec<MessageNode>, Error> {
        let graph = connect().await?;
        let q = "MATCH (m:MessageNode) RETURN m, id(m) as id";
        let mut result = graph.execute(query(q)).await?;
        let mut messages = Vec::new();
        while let Some(row) = result.next().await? {
            let mut node: MessageNode = row.get("m")?;
            node.id = Some(row.get::<i64>("id")?);
            messages.push(node);
        }
        Ok(messages)
    }

    async fn get_last_messages_for_partition_and_instance(
        &self,
        partition: String,
        instance: String,
        count: usize,
    ) -> Result<Vec<MessageNode>, Error> {
        let graph = connect().await?;
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

    async fn get_messages_for_embedding_nodes(
        &self,
        embedding_nodes: Vec<i64>,
        embedding_info: &EmbeddingInfo,
    ) -> Result<Vec<MessageNode>, Error> {
        let graph = connect().await?;
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

    async fn save_message_node(
        &self,
        message_node: &MessageNode,
        embedding_info: &EmbeddingInfo,
    ) -> Result<(), Error> {
        if message_node.role.eq_ignore_ascii_case("system") {
            return Ok(());
        }

        let graph = connect().await?;
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

        let mut create_result = graph.execute(create_q).await?;
        let _ = create_result.next().await?;

        if message_node.role.eq_ignore_ascii_case("assistant") {
            let link_q = query(
                r#"MATCH (u:MessageNode {role: 'user', trace_id: $trace_id})
                       MATCH (a:MessageNode {role: 'assistant', trace_id: $trace_id})
                       MERGE (u)-[:RESPONDED_WITH]->(a)
                       RETURN count(*)"#,
            )
            .param("trace_id", message_node.trace_id.clone());

            let mut link_result = graph.execute(link_q).await?;
            let _ = link_result.next().await?;
        }

        Ok(())
    }

    async fn init_vector_index(&self) -> Result<(), Error> {
        let (res1, res2) = tokio::join!(
            self.create_index_with_size(1024),
            self.create_index_with_size(1536),
        );
        res1?;
        res2
    }

    async fn get_nodes_connected_by_synapses(
        &self,
        node: &MessageNode,
    ) -> Result<Vec<MessageNode>, Error> {
        let graph = connect().await?;
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

    async fn connect_synapses(&self) -> Result<(), Error> {
        let graph = connect().await?;
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
}

impl Neo4jMessageRepository {
    async fn create_index_for_node(&self, size: usize) -> Result<(), Error> {
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
        let graph = connect().await?;
        let mut result = graph.execute(query(&query_str)).await?;
        while let Ok(Some(row)) = result.next().await {
            let message: String = row.get("message")?;
            info!("Index creation message: {}", message);
        }
        info!("Created index: {}", index_name);
        Ok(())
    }

    async fn create_index_with_size(&self, size: usize) -> Result<(), Error> {
        let index_name = format!("embedding{}", size);
        let graph = connect().await?;
        let check_query = query("SHOW INDEXES YIELD name RETURN name");
        let mut result = graph.execute(check_query).await?;

        while let Ok(Some(row)) = result.next().await {
            let name: String = row.get("name")?;
            if name == index_name {
                return Ok(());
            }
        }

        self.create_index_for_node(size).await?;
        Ok(())
    }
}
