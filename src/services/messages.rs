use anyhow::Error;

use crate::{
    clients::embedding::EmbeddingInfo,
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
    embedding_info: &EmbeddingInfo,
    partition: &str,
    instance: &str,
    top_k: usize,
) -> Result<Vec<MessageNode>, Error> {
    let related_embedding_nodes = find_similar_embeddings(
        connect,
        embedding,
        embedding_info,
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
        get_messages_for_embedding_nodes(connect, embedding_node_ids, embedding_info).await;
    related_messages
}

pub async fn get_related_messages_with_strategy(
    embedding: Vec<f32>,
    embedding_info: &EmbeddingInfo,
    partition: &str,
    instance: &str,
    top_k: usize,
) -> Result<Vec<MessageNode>, Error> {
    let similar_messages =
        get_most_similar_messages(embedding, embedding_info, partition, instance, top_k).await?;
    let mut found_messages = vec![];
    for message in similar_messages.clone() {
        let mut connected = find_nodes_connected_to_node(connect, &message).await?;
        if found_messages.len() > top_k * 3 {
            break;
        }
        if connected.len() > 2 {
            found_messages.append(connected.as_mut());
        }
        found_messages = deduplicate_message_nodes(found_messages);
    }

    Ok(found_messages.into_iter().take(top_k).collect())
}
