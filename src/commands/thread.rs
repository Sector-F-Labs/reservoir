use crate::clients::openai::types::Message;
use crate::repos::message::neo4j_message::get_context;
use crate::utils::connector::connect;
use anyhow::Error;
use clap::Parser;
use tracing::error;

#[derive(Parser, Debug)]
#[command(author, version, about = "View thread context (synapse-connected + recent messages)", long_about = None)]
pub struct ThreadSubCommand {
    /// Partition (defaults to "default")
    #[arg(short, long)]
    pub partition: Option<String>,
    /// Instance (defaults to partition)
    #[arg(short, long)]
    pub instance: Option<String>,
    /// Max messages to return
    #[arg(short, long, default_value = "50")]
    pub count: usize,
}

pub async fn execute(
    partition: String,
    instance: String,
    count: usize,
) -> Result<Vec<Message>, Error> {
    let messages = get_context(connect, &partition, &instance, count).await?;

    let messages: Vec<Message> = messages.iter().map(|m| m.to_message()).collect();
    Ok(messages)
}

pub async fn run(cmd: &ThreadSubCommand) -> Result<(), Error> {
    let partition = cmd
        .partition
        .clone()
        .unwrap_or_else(|| "default".to_string());
    let instance = cmd.instance.clone().unwrap_or_else(|| partition.clone());

    match execute(partition, instance, cmd.count).await {
        Ok(output) => {
            if output.is_empty() {
                println!("No context found");
            } else {
                println!("Context ({} messages):", output.len());
                println!("---");
                for message in output {
                    println!("{}: {}", message.role, message.content);
                    println!("---");
                }
            }
            Ok(())
        }
        Err(e) => {
            error!("Error executing thread command: {:?}", e);
            Err(e)
        }
    }
}
