use crate::clients::embedding::{get_embeddings_for_txt, EmbeddingInfo};
use crate::repos::message::neo4j_message::save_message_node;
use crate::utils::connector::connect;
use anyhow::Error;

use crate::{clients::openai::types::ChatRequest, models::message_node::MessageNode};

pub struct ChatRequestService {}

impl ChatRequestService {
    pub fn new() -> Self {
        ChatRequestService {}
    }

    pub async fn save_chat_request(
        &self,
        chat_request: &ChatRequest,
        embedding_info: &EmbeddingInfo,
        trace_id: &str,
        partition: &str,
        instance: &str,
    ) -> Result<(), Error> {
        for message in &chat_request.messages {
            let embedding =
                get_embeddings_for_txt(message.content.as_str(), embedding_info.to_owned()).await?;
            let node = MessageNode::from_message(message, trace_id, partition, instance, embedding);
            save_message_node(connect, &node, embedding_info).await?;
        }
        Ok(())
    }
}
