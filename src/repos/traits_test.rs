#[cfg(test)]
mod tests {
    use super::super::neo4j_factory::Neo4jRepositoryFactory;
    use super::super::traits::{EmbeddingRepository, MessageRepository, RepositoryFactory};
    use crate::{
        clients::embedding::EmbeddingInfo,
        models::{embedding_node::EmbeddingNode, message_node::MessageNode},
    };
    use anyhow::Error;
    use async_trait::async_trait;

    // Mock repository implementations for testing
    struct MockMessageRepository;
    struct MockEmbeddingRepository;
    struct MockRepositoryFactory;

    #[async_trait]
    impl MessageRepository for MockMessageRepository {
        async fn get_messages_for_partition(
            &self,
            _partition: Option<&str>,
        ) -> Result<Vec<MessageNode>, Error> {
            Ok(vec![])
        }

        async fn get_messages(&self) -> Result<Vec<MessageNode>, Error> {
            Ok(vec![])
        }

        async fn get_last_messages_for_partition_and_instance(
            &self,
            _partition: String,
            _instance: String,
            _count: usize,
        ) -> Result<Vec<MessageNode>, Error> {
            Ok(vec![])
        }

        async fn get_messages_for_embedding_nodes(
            &self,
            _embedding_nodes: Vec<i64>,
            _embedding_info: &EmbeddingInfo,
        ) -> Result<Vec<MessageNode>, Error> {
            Ok(vec![])
        }

        async fn save_message_node(
            &self,
            _message_node: &MessageNode,
            _embedding_info: &EmbeddingInfo,
        ) -> Result<(), Error> {
            Ok(())
        }

        async fn init_vector_index(&self) -> Result<(), Error> {
            Ok(())
        }

        async fn get_nodes_connected_by_synapses(
            &self,
            _node: &MessageNode,
        ) -> Result<Vec<MessageNode>, Error> {
            Ok(vec![])
        }

        async fn connect_synapses(&self) -> Result<(), Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl EmbeddingRepository for MockEmbeddingRepository {
        async fn attach_embedding_to_message(
            &self,
            _message: &MessageNode,
            _embedding: Vec<f32>,
            _embedding_info: &EmbeddingInfo,
            _model: &str,
        ) -> Result<(), Error> {
            Ok(())
        }

        async fn find_similar_embeddings(
            &self,
            _embedding: Vec<f32>,
            _embedding_info: &EmbeddingInfo,
            _partition: &str,
            _instance: &str,
            _top_k: usize,
        ) -> Result<Vec<EmbeddingNode>, Error> {
            Ok(vec![])
        }
    }

    impl RepositoryFactory for MockRepositoryFactory {
        type MessageRepo = MockMessageRepository;
        type EmbeddingRepo = MockEmbeddingRepository;

        fn create_message_repository(&self) -> Self::MessageRepo {
            MockMessageRepository
        }

        fn create_embedding_repository(&self) -> Self::EmbeddingRepo {
            MockEmbeddingRepository
        }
    }

    #[tokio::test]
    async fn test_mock_message_repository() {
        let repo = MockMessageRepository;

        // Test that all methods can be called without panicking
        let _ = repo.get_messages_for_partition(None).await;
        let _ = repo.get_messages().await;
        let _ = repo
            .get_last_messages_for_partition_and_instance("test".to_string(), "test".to_string(), 5)
            .await;
        let _ = repo
            .get_messages_for_embedding_nodes(vec![], &EmbeddingInfo::with_fastembed("test"))
            .await;

        let test_message = MessageNode {
            id: None,
            trace_id: "test".to_string(),
            partition: "test".to_string(),
            instance: "test".to_string(),
            content: Some("test".to_string()),
            role: "user".to_string(),
            embedding: vec![],
            url: None,
            timestamp: 0,
        };

        let _ = repo
            .save_message_node(&test_message, &EmbeddingInfo::with_fastembed("test"))
            .await;
        let _ = repo.init_vector_index().await;
        let _ = repo.get_nodes_connected_by_synapses(&test_message).await;
        let _ = repo.connect_synapses().await;

        // If we get here without panicking, the trait is working
        assert!(true);
    }

    #[tokio::test]
    async fn test_mock_embedding_repository() {
        let repo = MockEmbeddingRepository;

        let test_message = MessageNode {
            id: Some(1),
            trace_id: "test".to_string(),
            partition: "test".to_string(),
            instance: "test".to_string(),
            content: Some("test".to_string()),
            role: "user".to_string(),
            embedding: vec![],
            url: None,
            timestamp: 0,
        };

        // Test that all methods can be called without panicking
        let _ = repo
            .attach_embedding_to_message(
                &test_message,
                vec![0.1, 0.2, 0.3],
                &EmbeddingInfo::with_fastembed("test"),
                "test-model",
            )
            .await;

        let _ = repo
            .find_similar_embeddings(
                vec![0.1, 0.2, 0.3],
                &EmbeddingInfo::with_fastembed("test"),
                "test",
                "test",
                5,
            )
            .await;

        // If we get here without panicking, the trait is working
        assert!(true);
    }

    #[test]
    fn test_repository_factory() {
        let factory = MockRepositoryFactory;

        // Test that factory can create repositories
        let _message_repo = factory.create_message_repository();
        let _embedding_repo = factory.create_embedding_repository();

        // Test that Neo4j factory can be instantiated
        let _neo4j_factory = Neo4jRepositoryFactory::new();

        assert!(true);
    }

    #[test]
    fn test_trait_object_compatibility() {
        // Test that we can use trait objects (dynamic dispatch)
        let factory: Box<
            dyn RepositoryFactory<
                MessageRepo = MockMessageRepository,
                EmbeddingRepo = MockEmbeddingRepository,
            >,
        > = Box::new(MockRepositoryFactory);

        let _message_repo = factory.create_message_repository();
        let _embedding_repo = factory.create_embedding_repository();

        assert!(true);
    }
}
