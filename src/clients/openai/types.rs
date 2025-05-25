use serde::{Deserialize, Serialize};

use crate::models::message_node::MessageNode;

/// Represents a message in the OpenAI chat completions API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u64,
}

/// Represents a chat request to an OpenAI chat completions API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[allow(dead_code)]
impl ChatRequest {
    pub fn new(model: String, messages: Vec<Message>) -> Self {
        ChatRequest { model, messages }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

pub fn enrich_chat_request(
    similar_messages: Vec<MessageNode>,
    mut last_messages: Vec<MessageNode>, // Add `mut` here
    chat_request: &ChatRequest,
) -> ChatRequest {
    let mut chat_request = chat_request.clone();

    let semantic_prompt = r#"The following is the result of a semantic search 
        of the most related messages by cosine similarity to previous 
        conversations"#;
    let recent_prompt = r#"The following are the most recent messages in the 
        conversation in chronological order"#;

    last_messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut enrichment_block = Vec::new();

    enrichment_block.push(Message {
        role: "system".to_string(),
        content: semantic_prompt.to_string(),
    });
    enrichment_block.extend(similar_messages.iter().map(MessageNode::to_message));
    enrichment_block.push(Message {
        role: "system".to_string(),
        content: recent_prompt.to_string(),
    });
    enrichment_block.extend(last_messages.iter().map(MessageNode::to_message));

    enrichment_block.retain(|m| !m.content.is_empty());

    let insert_index = if chat_request
        .messages
        .first()
        .is_some_and(|m| m.role == "system")
    {
        1
    } else {
        0
    };

    // Insert enrichment block
    chat_request
        .messages
        .splice(insert_index..insert_index, enrichment_block);
    chat_request
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub model: Option<String>,
    pub usage: Option<Usage>,
    pub choices: Vec<Choice>,
}

#[allow(dead_code)]
impl ChatResponse {
    pub fn new(
        id: Option<String>,
        object: Option<String>,
        created: Option<i64>,
        model: Option<String>,
        usage: Option<Usage>,
        choices: Vec<Choice>,
    ) -> Self {
        ChatResponse {
            id,
            object,
            created,
            model,
            usage,
            choices,
        }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_request_serialization() {
        let messages = vec![
            Message {
                role: "user".to_string(),
                content: "Hello, how are you?".to_string(),
            },
            Message {
                role: "assistant".to_string(),
                content: "I'm fine, thank you!".to_string(),
            },
        ];
        let chat_request = ChatRequest::new("gpt-3.5-turbo".to_string(), messages);
        let json = serde_json::to_string(&chat_request).unwrap();
        assert!(json.contains("gpt-3.5-turbo"));
        assert!(json.contains("Hello, how are you?"));
    }

    //test that deserialisation from a json request occurs properly
    #[test]
    fn test_chat_request_deserialization() {
        let json = r#"{
            "model": "gpt-3.5-turbo",
            "messages": [
                {"role": "user", "content": "Hello, how are you?"},
                {"role": "assistant", "content": "I'm fine, thank you!"}
            ]
        }"#;
        let chat_request: ChatRequest = serde_json::from_str(json).unwrap();
        assert_eq!(chat_request.model, "gpt-3.5-turbo");
        assert_eq!(chat_request.messages.len(), 2);
        assert_eq!(chat_request.messages[0].role, "user");
        assert_eq!(chat_request.messages[1].role, "assistant");
    }
}
