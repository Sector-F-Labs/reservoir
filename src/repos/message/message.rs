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
}
