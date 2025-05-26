use anyhow::Error;
use tracing::info;

use crate::{
    args::ReplaySubCommand,
    clients::embedding::{get_embeddings_for_txt, EmbeddingClient},
    repos::{
        embedding::neo4j_embedding::attach_embedding_to_message,
        message::neo4j_message::get_messages,
    },
    utils::connector::connect,
};

pub async fn execute(model: &str) -> Result<(), Error> {
    let messages = get_messages(connect).await?;
    info!("Found {} messages to process", messages.len());

    for message in messages {
        let ec: EmbeddingClient = EmbeddingClient::with_fastembed(model);
        println!("message id : {:?}", message.id);

        match message.content.clone() {
            Some(content) => match get_embeddings_for_txt(content.as_str(), ec.clone()).await {
                Ok(embeddings) => {
                    info!("attaching to message: {:?}", message.id);
                    let r = attach_embedding_to_message(connect, &message, embeddings, &ec, model)
                        .await;
                    match r {
                        Ok(_) => {
                            println!(
                                "Successfully attached embeddings to message with trace ID: {}",
                                message.trace_id
                            );
                        }
                        Err(e) => {
                            eprintln!(
                                "Failed to attach embeddings to message with trace ID: {}. Error: {}",
                                message.trace_id, e
                            );
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching embeddings: {}", e),
            },
            None => {
                println!(
                    "No content found for message with trace ID: {}",
                    message.trace_id
                );
            }
        }
    }

    Ok(())
}

pub async fn run(replay_sub_command: &ReplaySubCommand) -> Result<(), Error> {
    info!("specified model: {:?}", replay_sub_command.model);
    let model = "bge-large-en-v15";
    execute(model).await
}
