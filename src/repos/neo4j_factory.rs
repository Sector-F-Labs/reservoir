use crate::repos::{
    embedding::neo4j::Neo4jEmbeddingRepository,
    message::neo4j::Neo4jMessageRepository,
    traits::RepositoryFactory,
};

pub struct Neo4jRepositoryFactory;

impl Neo4jRepositoryFactory {
    pub fn new() -> Self {
        Self
    }
}

impl RepositoryFactory for Neo4jRepositoryFactory {
    type MessageRepo = Neo4jMessageRepository;
    type EmbeddingRepo = Neo4jEmbeddingRepository;

    fn create_message_repository(&self) -> Self::MessageRepo {
        Neo4jMessageRepository::new()
    }

    fn create_embedding_repository(&self) -> Self::EmbeddingRepo {
        Neo4jEmbeddingRepository::new()
    }
}
