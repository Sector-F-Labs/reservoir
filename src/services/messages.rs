use anyhow::Error;
use tracing::info;

use crate::{
    clients::embedding::EmbeddingClient,
    models::message_node::MessageNode,
    repos::{
        embedding::neo4j_embedding::find_similar_embeddings,
        message::neo4j_message::{find_nodes_connected_to_node, get_messages_for_embedding_nodes},
    },
    utils::{connector::connect, deduplicate_message_nodes},
};

/// Gets the messages closest to the input embedding
/// given a specific embedding model
pub async fn get_most_similar_messages(
    embedding: Vec<f32>,
    embedding_client: &EmbeddingClient,
    partition: &str,
    instance: &str,
    top_k: usize,
) -> Result<Vec<MessageNode>, Error> {
    let related_embedding_nodes = find_similar_embeddings(
        connect,
        embedding,
        embedding_client,
        partition,
        instance,
        top_k,
    )
    .await?;

    let embedding_node_ids: Vec<i64> = related_embedding_nodes
        .iter()
        .filter_map(|node| node.id)
        .collect();

    let related_messages =
        get_messages_for_embedding_nodes(connect, embedding_node_ids, embedding_client).await;
    related_messages
}

pub async fn get_related_messages_with_strategy(
    embedding: Vec<f32>,
    embedding_client: &EmbeddingClient,
    partition: &str,
    instance: &str,
    top_k: usize,
) -> Result<Vec<MessageNode>, Error> {
    let similar_messages =
        get_most_similar_messages(embedding, embedding_client, partition, instance, top_k).await?;
    let mut found_messages = vec![];
    for message in similar_messages.clone() {
        let mut connected = find_nodes_connected_to_node(connect, &message).await?;
        if found_messages.len() > top_k * 3 {
            break;
        }
        if connected.len() > 1 {
            found_messages.append(connected.as_mut());
            found_messages = deduplicate_message_nodes(found_messages);
        }
    }
    if similar_messages.is_empty() {
        info!("No related messages found for the given embedding.");
        return Ok(vec![]);
    } else {
        info!(
            "Found {} related messages for the given embedding.",
            similar_messages.len()
        );
    }

    Ok(found_messages.into_iter().take(top_k).collect())
}
