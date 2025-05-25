use crate::{repos::message::neo4j_message::get_messages_for_partition, utils::connector::connect};
use anyhow::Error;
use serde_json;

pub async fn run() -> Result<(), Error> {
    let messages = get_messages_for_partition(connect, None).await?;
    let json = serde_json::to_string_pretty(&messages)?;
    println!("{}", json);
    Ok(())
}
