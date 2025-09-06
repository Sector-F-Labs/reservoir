use anyhow::Error;
use async_trait::async_trait;

use crate::{
    clients::embedding::EmbeddingInfo,
    models::{embedding_node::EmbeddingNode, message_node::MessageNode},
};

/// Trait defining the interface for message storage operations
#[async_trait]
pub trait MessageRepository {
    /// Get all messages for a specific partition, or all messages if partition is None
    async fn get_messages_for_partition(
        &self,
        partition: Option<&str>,
    ) -> Result<Vec<MessageNode>, Error>;

    /// Get all messages in the repository
    async fn get_messages(&self) -> Result<Vec<MessageNode>, Error>;

    /// Get the last N messages for a specific partition and instance
    async fn get_last_messages_for_partition_and_instance(
        &self,
        partition: String,
        instance: String,
        count: usize,
    ) -> Result<Vec<MessageNode>, Error>;

    /// Get messages associated with specific embedding node IDs
    async fn get_messages_for_embedding_nodes(
        &self,
        embedding_nodes: Vec<i64>,
        embedding_info: &EmbeddingInfo,
    ) -> Result<Vec<MessageNode>, Error>;

    /// Save a message node to storage
    async fn save_message_node(
        &self,
        message_node: &MessageNode,
        embedding_info: &EmbeddingInfo,
    ) -> Result<(), Error>;

    /// Initialize vector indexes for embeddings (implementation-specific)
    async fn init_vector_index(&self) -> Result<(), Error>;

    /// Find nodes connected to a given node through synapse relationships
    async fn get_nodes_connected_by_synapses(
        &self,
        node: &MessageNode,
    ) -> Result<Vec<MessageNode>, Error>;

    /// Create synapse connections between messages based on embedding similarity
    async fn connect_synapses(&self) -> Result<(), Error>;
}

/// Trait defining the interface for embedding storage operations
#[async_trait]
pub trait EmbeddingRepository {
    /// Attach an embedding to an existing message
    async fn attach_embedding_to_message(
        &self,
        message: &MessageNode,
        embedding: Vec<f32>,
        embedding_info: &EmbeddingInfo,
        model: &str,
    ) -> Result<(), Error>;

    /// Find embedding nodes similar to the provided embedding vector
    async fn find_similar_embeddings(
        &self,
        embedding: Vec<f32>,
        embedding_info: &EmbeddingInfo,
        partition: &str,
        instance: &str,
        top_k: usize,
    ) -> Result<Vec<EmbeddingNode>, Error>;
}

/// Repository factory trait for creating repository instances
pub trait RepositoryFactory {
    type MessageRepo: MessageRepository + Send + Sync;
    type EmbeddingRepo: EmbeddingRepository + Send + Sync;

    fn create_message_repository(&self) -> Self::MessageRepo;
    fn create_embedding_repository(&self) -> Self::EmbeddingRepo;
}
