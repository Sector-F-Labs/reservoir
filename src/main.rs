use anyhow::Error;
use args::{Args, SubCommands};
use clap::Parser;
use repos::message::neo4j_message::init_vector_index;

mod args;
mod clients;
mod commands;
mod models;
mod repos;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "reservoir=info".to_string()))
        .init();

    let args = Args::parse();
    init_vector_index().await?;

    match args.subcmd {
        Some(SubCommands::Config(_config_subcmd)) => {
            commands::config::run().await?;
        }
        Some(SubCommands::Export) => {
            commands::export::run().await?;
        }
        Some(SubCommands::Import(import_cmd)) => {
            commands::import::run(&import_cmd.file).await?;
        }
        Some(SubCommands::View(ref view_cmd)) => {
            commands::view::run(view_cmd).await?;
        }
        Some(SubCommands::Search(ref search_cmd)) => {
            commands::search::run(search_cmd).await?;
        }
        Some(SubCommands::Ingest(ref ingest_cmd)) => {
            commands::ingest::run(ingest_cmd).await?;
        }
        Some(SubCommands::Replay(ref r_cmd)) => {
            commands::replay::run(r_cmd).await?;
        }
        Some(SubCommands::Thread(ref thread_cmd)) => {
            commands::thread::run(thread_cmd).await?;
        }
        None => {
            // No subcommand provided, show help
            Args::parse_from(["reservoir", "--help"]);
        }
    };
    Ok(())
}
