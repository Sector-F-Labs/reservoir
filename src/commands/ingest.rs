use crate::args::IngestSubCommand;
use crate::clients::embedding::{get_embeddings_for_txt, EmbeddingInfo};
use crate::clients::openai::types::Message;
use crate::models::message_node::MessageNode;
use crate::repos::message::neo4j_message::save_message_node;
use crate::utils::connector::connect;
use anyhow::Error;
use std::io::{self, Read};
use tracing::{debug, info};
use uuid::Uuid;

pub async fn run(cmd: &IngestSubCommand) -> Result<(), Error> {
    // Read stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let content = buffer.trim().to_string();
    if content.is_empty() {
        println!("No input provided on stdin");
        return Ok(());
    }
    let partition = cmd
        .partition
        .clone()
        .unwrap_or_else(|| "default".to_string());
    let instance = cmd.instance.clone().unwrap_or_else(|| partition.clone());
    let trace_id = Uuid::new_v4().to_string();
    let role = cmd.role.clone().unwrap_or_else(|| "user".to_string());
    let allowed_roles = ["user", "assistant", "system"];
    if !allowed_roles.contains(&role.as_str()) {
        eprintln!("Error: role must be one of: user, assistant, system");
        return Ok(());
    }
    let message = Message {
        role,
        content: content.clone(),
    };
    let embedding_info = EmbeddingInfo::with_fastembed("");
    let embedding = get_embeddings_for_txt(&content, embedding_info.clone()).await?;
    let embedding_test = get_embeddings_for_txt(&content, embedding_info.clone()).await?;

    info!("Embedding test: {:?}", embedding_test);

    let node = MessageNode::from_message(&message, &trace_id, &partition, &instance, embedding);
    save_message_node(connect, &node, &embedding_info).await?;
    debug!("Saved message with trace_id: {}", trace_id);
    Ok(())
}
