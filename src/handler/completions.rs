use anyhow::Error;

use crate::clients::embedding::{get_embeddings_for_txt, EmbeddingInfo};
use crate::clients::openai::chat_completions::get_completion_message;
use crate::clients::openai::model_info::ModelInfo;
use crate::clients::openai::types::{
    enrich_chat_request, ChatRequest, ChatResponse, Choice, Message,
};
use crate::models::message_node::MessageNode;
use crate::repos::config;
use crate::repos::message::neo4j_message::{
    connect_synapses, get_last_messages_for_partition_and_instance, save_message_node,
};
use crate::services::chat_request::ChatRequestService;
use crate::services::messages::get_related_messages_with_strategy;
use crate::utils::connector::connect;
use crate::utils::{
    count_single_message_tokens, get_last_message_in_chat_request, truncate_messages_if_needed,
};
use bytes::Bytes;
use uuid::Uuid;

use tracing::{error, info};

const LAST_MESSAGES_LIMIT: usize = 15;

pub async fn is_last_message_too_big(last_message: &Message, model: &ModelInfo) -> Option<Bytes> {
    let input_token_limit = model.input_tokens;
    let last_message_tokens = count_single_message_tokens(last_message);
    if last_message_tokens > input_token_limit {
        info!(
            "Last message token count ({}) exceeds limit ({}), returning error response.",
            last_message_tokens, input_token_limit
        );

        let error_content = format!(
                "Your last message is too long. It contains approximately {} tokens, which exceeds the maximum limit of {}. Please shorten your message.",
                last_message_tokens, input_token_limit
            );
        let error_message = Message {
            role: "assistant".to_string(),
            content: error_content,
        };

        let error_choice = Choice {
            index: 0,
            message: error_message,
            finish_reason: "length".to_string(), // Indicate truncation due to length
        };
        let error_response = ChatResponse {
            id: None,
            object: None,
            created: None,
            model: None,
            choices: vec![error_choice],
            usage: None,
        };

        // Serialize and return the error response
        let response_bytes = serde_json::to_vec(&error_response).unwrap();
        Some(Bytes::from(response_bytes))
    } else {
        info!(
            "Last message token count ({}) is within limit ({}).",
            last_message_tokens, input_token_limit
        );
        None
    }
}

/// Handles a chat request with partition and instance information.
pub async fn handle_chat_with_partition(
    partition: &str,
    instance: &str,
    whole_body: Bytes,
) -> Result<Bytes, Error> {
    let json_string = String::from_utf8_lossy(&whole_body).to_string();
    let chat_request_model = ChatRequest::from_json(json_string.as_str()).expect("Valid JSON");
    let model_info = ModelInfo::new(chat_request_model.model.clone());

    let trace_id = Uuid::new_v4().to_string();
    let service = ChatRequestService::new();

    let last_message = chat_request_model
        .messages
        .last()
        .ok_or_else(|| anyhow::anyhow!("There are no messages in the request"))?;

    let too_big = is_last_message_too_big(last_message, &model_info).await;
    if let Some(bytes) = too_big {
        return Ok(bytes);
    }

    let search_term = last_message.content.as_str();
    get_last_message_in_chat_request(&chat_request_model)?;

    info!("Using search term: {}", search_term);
    let embedding_info = EmbeddingInfo::with_fastembed("bge-large-en-v15");
    let embeddings = get_embeddings_for_txt(search_term, embedding_info.clone()).await?;

    let context_size = config::get_context_size();
    let similar = get_related_messages_with_strategy(
        embeddings,
        &embedding_info,
        partition,
        instance,
        context_size,
    )
    .await?;

    let last_messages = get_last_messages_for_partition_and_instance(
        connect,
        partition.to_string(),
        instance.to_string(),
        LAST_MESSAGES_LIMIT,
    )
    .await
    .unwrap_or_else(|e| {
        error!("Error finding last messages: {}", e);
        Vec::new()
    });
    let embedding_info = EmbeddingInfo::with_fastembed("bge-large-en-v15");
    service
        .save_chat_request(
            &chat_request_model,
            &embedding_info.clone(),
            trace_id.as_str(),
            partition,
            instance,
        )
        .await
        .expect("Could not save the request");

    let mut enriched_chat_request =
        enrich_chat_request(similar, last_messages, &chat_request_model);
    truncate_messages_if_needed(&mut enriched_chat_request.messages, model_info.input_tokens);

    let chat_response = match get_completion_message(&model_info, &enriched_chat_request).await {
        Ok(response) => response,
        Err(e) => {
            error!(
                "Failed to get completion message from {}: {}",
                model_info.base_url, e
            );
            return Err(Error::msg(format!(
                "LLM API request failed for model '{}' at '{}': {}. Please check if the service is running and accessible.",
                model_info.name, model_info.base_url, e
            )));
        }
    };
    let message_node = chat_response.choices.first().unwrap().message.clone();
    let embedding =
        get_embeddings_for_txt(message_node.content.as_str(), embedding_info.clone()).await?;
    let message_node = MessageNode::from_message(
        &message_node,
        trace_id.as_str(),
        partition,
        instance,
        embedding,
    );
    save_message_node(connect, &message_node, &embedding_info)
        .await
        .expect("Failed to save message node");

    connect_synapses(connect)
        .await
        .expect("Failed to connect synapses");

    let response_text =
        serde_json::to_string(&chat_response).expect("Failed to serialize chat response");
    Ok(Bytes::from(response_text))
}
