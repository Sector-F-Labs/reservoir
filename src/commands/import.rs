use crate::clients::embedding::EmbeddingClient;
use crate::models::message_node::MessageNode;
use crate::repos::message::neo4j_message::save_message_node;
use crate::utils::connector::connect;
use anyhow::Error;
use serde_json;
use std::fs;

pub async fn run(file: &str) -> Result<(), Error> {
    let file_content = fs::read_to_string(file)?;
    let messages: Vec<MessageNode> = serde_json::from_str(&file_content)?;
    let client = EmbeddingClient::default();
    for message in &messages {
        save_message_node(connect, message, &client).await?;
    }
    println!("Imported {} message nodes from {}", messages.len(), file);
    Ok(())
}
