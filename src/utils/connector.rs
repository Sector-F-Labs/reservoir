use std::future::Future;

use anyhow::Error;
use neo4rs::{ConfigBuilder, Graph};

use crate::repos::config::{get_neo4j_password, get_neo4j_uri, get_neo4j_user};

pub async fn connect() -> Result<Graph, Error> {
    let config = ConfigBuilder::new()
        .uri(get_neo4j_uri())
        .user(get_neo4j_user())
        .password(get_neo4j_password())
        .build()?;
    let graph = Graph::connect(config).await?;
    Ok(graph)
}

pub trait AsyncGraphFuture: Future<Output = Result<Graph, Error>> + Send {}
impl<T> AsyncGraphFuture for T where T: Future<Output = Result<Graph, Error>> + Send {}
