use crate::models::message_node::MessageNode;
use anyhow::Error;
use neo4rs::*;

use super::Neo4jMessageRepository;

pub trait MessageRepository {
    async fn get_messages_for_partition(
        &self,
        partition: Option<&str>,
    ) -> Result<Vec<MessageNode>, Error>;

    async fn get_last_messages_for_partition_and_instance(
        &self,
        partition: String,
        instance: String,
        count: usize,
    ) -> Result<Vec<MessageNode>, Error>;

    async fn find_connections_between_nodes(
        &self,
        nodes: &[MessageNode],
    ) -> Result<Vec<MessageNode>, Error>; // Changed return type
    async fn find_nodes_connected_to_node(
        &self,
        node: &MessageNode,
    ) -> Result<Vec<MessageNode>, Error>; // Changed return type
    async fn connect_synapses(&self) -> Result<(), Error>;
}

pub enum AnyMessageRepository {
    Neo4j(Neo4jMessageRepository),
}

impl AnyMessageRepository {
    pub fn new_neo4j() -> Self {
        AnyMessageRepository::Neo4j(Neo4jMessageRepository::default())
    }
}

impl MessageRepository for AnyMessageRepository {
    async fn get_messages_for_partition(
        &self,
        partition: Option<&str>,
    ) -> Result<Vec<MessageNode>, Error> {
        match self {
            AnyMessageRepository::Neo4j(repo) => repo.get_messages_for_partition(partition).await,
        }
    }

    async fn get_last_messages_for_partition_and_instance(
        &self,
        partition: String,
        instance: String,
        count: usize,
    ) -> Result<Vec<MessageNode>, Error> {
        match self {
            AnyMessageRepository::Neo4j(repo) => {
                repo.get_last_messages_for_partition_and_instance(partition, instance, count)
                    .await
            }
        }
    }

    async fn find_connections_between_nodes(
        &self,
        nodes: &[MessageNode],
    ) -> Result<Vec<MessageNode>, Error> {
        match self {
            AnyMessageRepository::Neo4j(repo) => repo.find_connections_between_nodes(nodes).await,
        }
    }

    async fn find_nodes_connected_to_node(
        &self,
        node: &MessageNode,
    ) -> Result<Vec<MessageNode>, Error> {
        match self {
            AnyMessageRepository::Neo4j(repo) => repo.find_nodes_connected_to_node(node).await,
        }
    }

    async fn connect_synapses(&self) -> Result<(), Error> {
        match self {
            AnyMessageRepository::Neo4j(repo) => repo.connect_synapses().await,
        }
    }
}

#[cfg(test)] // Ignoring tests as requested
mod tests {
    use crate::{
        models::message_node::MessageNode, repos::message::neo4j_message::save_message_node,
        utils::connector::connect,
    };
    use tracing::error;

    #[tokio::test]
    async fn test_save_message_node() {
        let message_node = MessageNode {
            id: None,
            embedding: vec![],
            trace_id: "12345".to_string(),
            partition: "default".to_string(),
            instance: "default".to_string(),
            role: "user".to_string(),
            content: Some("Hello, world!".to_string()),
            url: None,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };
        let result = save_message_node(connect, &message_node).await;
        if result.is_err() {
            error!("Error saving message node: {:?}", result);
        }
        assert!(result.is_ok());
    }
}
