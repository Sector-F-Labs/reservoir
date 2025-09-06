use anyhow::Error;
use async_trait::async_trait;
use neo4rs::query;
use tracing::{error, info};

use crate::{
    clients::embedding::EmbeddingInfo,
    models::{embedding_node::EmbeddingNode, message_node::MessageNode},
    repos::traits::EmbeddingRepository,
    utils::connector::connect,
};

pub struct Neo4jEmbeddingRepository;

impl Neo4jEmbeddingRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EmbeddingRepository for Neo4jEmbeddingRepository {
    async fn attach_embedding_to_message(
        &self,
        message: &MessageNode,
        embedding: Vec<f32>,
        embedding_info: &EmbeddingInfo,
        model: &str,
    ) -> Result<(), Error> {
        let message_id = message.id.unwrap_or_default();
        let partition = message.partition.clone();
        let instance = message.instance.clone();
        let trace_id = message.trace_id.clone();
        let role = message.role.clone();
        let timestamp = message.timestamp;

        info!("Attaching embedding to message with ID: {}", message_id);
        info!("Model: {}", model);
        info!("Partition: {:?}", partition);
        info!("Instance: {:?}", instance);
        info!("Trace ID: {}", trace_id);
        info!("Role: {}", role);

        let graph = connect().await?;
        let query_string = format!(
            r#"
                MATCH (m:MessageNode)
                WHERE m.trace_id = $trace_id
                AND m.role = $role
                CREATE (e:{} {{
                    embedding: $embedding,
                    model: $model,
                    partition: $partition,
                    instance: $instance,
                    timestamp: $timestamp
                }})
                CREATE (m)-[:HAS_EMBEDDING]->(e)
                "#,
            embedding_info.get_node_name()
        );
        let q = query(query_string.as_str())
            .param("embedding", embedding)
            .param("timestamp", timestamp)
            .param("partition", partition)
            .param("model", model)
            .param("trace_id", trace_id)
            .param("role", role)
            .param("instance", instance);

        let mut r = graph.execute(q).await?;
        r.next().await?;

        Ok(())
    }

    async fn find_similar_embeddings(
        &self,
        embedding: Vec<f32>,
        embedding_info: &EmbeddingInfo,
        partition: &str,
        instance: &str,
        top_k: usize,
    ) -> Result<Vec<EmbeddingNode>, Error> {
        let top_k_extended = (top_k * 3) as i64;
        let graph = connect().await?;
        let query_string = format!(
            r#"
                    CALL db.index.vector.queryNodes(
                        '{}',
                        $topKExtended,
                        $embedding
                    ) YIELD node, score
                    WITH node, score
                    WHERE node.partition = $partition
                      AND node.instance = $instance
                    RETURN node.partition AS partition,
                           node.instance AS instance,
                           node.embedding AS embedding,
                           node.model AS model,
                           id(node) AS id,
                           score
                    ORDER BY score DESC
                    "#,
            embedding_info.get_index_name()
        );
        let q = query(query_string.as_str())
            .param("embedding", embedding)
            .param("topKExtended", top_k_extended)
            .param("partition", partition)
            .param("instance", instance);

        let result = graph.execute(q).await;

        let mut result = match result {
            Ok(r) => r,
            Err(e) => {
                error!("Error executing query: {}", e);
                return Err(Error::msg(format!("Error executing query: {}", e)));
            }
        };
        info!("Query executed successfully");

        let mut similar_embeddings = Vec::new();
        while let Some(row) = result.next().await? {
            let id = row.get::<i64>("id")?;
            let model = row.get::<String>("model")?;

            let node = EmbeddingNode {
                id: Some(id),
                model,
                embedding: vec![],
                partition: Some(partition.to_string()),
                instance: Some(instance.to_string()),
            };

            similar_embeddings.push(node);
        }

        Ok(similar_embeddings)
    }
}
