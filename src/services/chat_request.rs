use crate::clients::embedding::{get_embeddings_for_txt, EmbeddingClient};
use crate::repos::embedding::neo4j_embedding::find_similar_embeddings;

use crate::repos::message::neo4j_message::{get_messages_for_embedding_nodes, save_message_node};
use crate::utils::connector::connect;
use anyhow::Error;
use tracing::info;

use crate::{clients::openai::types::ChatRequest, models::message_node::MessageNode};

pub struct ChatRequestService {}

impl ChatRequestService {
    pub fn new() -> Self {
        ChatRequestService {}
    }

    pub async fn save_chat_request(
        &self,
        chat_request: &ChatRequest,
        embedding_client: &EmbeddingClient,
        trace_id: &str,
        partition: &str,
        instance: &str,
    ) -> Result<(), Error> {
        for message in &chat_request.messages {
            let embedding =
                get_embeddings_for_txt(message.content.as_str(), embedding_client.to_owned())
                    .await?;
            let node = MessageNode::from_message(message, trace_id, partition, instance, embedding);
            save_message_node(connect, &node, embedding_client).await?;
        }
        Ok(())
    }

    /// Gets a list of similar messages based on the embeddings they are linked
    /// to. This is useful for finding related messages in a chat history.
    pub async fn find_semantically_similar_messages(
        &self,
        embedding: Vec<f32>,
        embedding_client: &EmbeddingClient,
        _trace_id: &str,
        partition: &str,
        instance: &str,
        top_k: usize,
    ) -> Result<Vec<MessageNode>, Error> {
        let embedding_result = find_similar_embeddings(
            connect,
            embedding.clone(),
            embedding_client,
            partition,
            instance,
            top_k,
        )
        .await;

        let embedding_result = match embedding_result {
            Ok(embeddings) => {
                if embeddings.is_empty() {
                    info!("No similar embeddings found");
                    return Ok(vec![]);
                }
                embeddings
            }
            Err(e) => {
                info!("Error finding similar embeddings: {}", e);
                return Err(e);
            }
        };

        let node_ids: Vec<i64> = embedding_result.iter().filter_map(|e| e.id).collect();
        let messages = get_messages_for_embedding_nodes(connect, node_ids, embedding_client).await;
        messages
    }
}
