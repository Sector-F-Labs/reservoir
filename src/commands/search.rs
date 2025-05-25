use crate::clients::embedding::{get_embeddings_for_txt, EmbeddingClient};
use crate::clients::openai::types::Message;
use crate::services::chat_request::ChatRequestService;
use crate::utils::deduplicate_message_nodes;
use anyhow::Error;
use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about = "Search messages by keyword or semantic similarity", long_about = None)]
pub struct SearchSubCommand {
    /// The search term (keyword or semantic)
    pub term: String,
    /// Use semantic search instead of keyword search
    #[arg(long)]
    pub semantic: bool,
    /// Partition to search (defaults to "default")
    #[arg(short, long)]
    pub partition: Option<String>,
    /// Instance to search (defaults to partition)
    #[arg(short, long)]
    pub instance: Option<String>,
    /// Use the same search strategy as RAG does when injecting
    /// into the model
    #[arg(short, long)]
    pub link: bool,
    /// Deuplicate first similarity results
    #[arg(short, long)]
    pub deduplicate: bool,
}

pub async fn run(service: &ChatRequestService, cmd: &SearchSubCommand) -> Result<(), Error> {
    let partition = cmd
        .partition
        .clone()
        .unwrap_or_else(|| "default".to_string());
    let instance = cmd.instance.clone().unwrap_or_else(|| partition.clone());
    let count = 10; // Default count for CLI search
    let options = SearchOptions {
        count,
        semantic: cmd.semantic,
        link: cmd.link,
        deduplicate: cmd.deduplicate,
    };
    match execute(service, partition, instance, cmd.term.clone(), options).await {
        Ok(messages) => {
            for (i, msg) in messages.iter().enumerate() {
                println!("{}. {}: {}", i + 1, msg.role, msg.content);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

pub struct SearchOptions {
    pub count: usize,
    pub semantic: bool,
    pub link: bool,
    pub deduplicate: bool,
}

pub async fn execute(
    service: &ChatRequestService,
    partition: String,
    instance: String,
    term: String,
    options: SearchOptions,
) -> Result<Vec<Message>, Error> {
    if options.semantic {
        let client = EmbeddingClient::with_fastembed("bge-large-env15");
        let embedding = get_embeddings_for_txt(&term, client.clone()).await?;
        let mut similar = service
            .find_similar_messages(
                embedding,
                &client,
                "search-trace-id",
                &partition,
                &instance,
                options.count,
            )
            .await?;
        if options.deduplicate {
            similar = deduplicate_message_nodes(similar);
        }
        if options.link {
            let similar_pairs = service.find_connections_between_nodes(&similar).await?;
            similar.extend(similar_pairs);
            let first = similar.first().cloned();
            similar = match first {
                Some(first) => {
                    let nodes = service.find_nodes_connected_to_node(&first).await?;
                    let nodes = deduplicate_message_nodes(nodes);
                    if nodes.len() > 2 {
                        nodes
                    } else {
                        similar
                    }
                }
                None => similar,
            };
        }
        let messages: Vec<Message> = similar.iter().map(|m| m.to_message()).collect();
        Ok(messages)
    } else {
        info!(
            "Keyword search: fetching messages for partition {}",
            partition
        );
        let messages = service.get_messages_for_partition(&partition).await?;
        let filtered: Vec<Message> = messages
            .iter()
            .filter(|m| {
                m.content
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&term.to_lowercase())
            })
            .take(options.count)
            .map(|m| m.to_message())
            .collect();
        Ok(filtered)
    }
}
