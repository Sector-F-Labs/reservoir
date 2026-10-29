use anyhow::Error;
use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use clap::Parser;
use tracing::{error, info};

use crate::repos::message::neo4j_message::{get_graph, GraphNode, GraphSynapse};
use crate::utils::connector::connect;

const INDEX_HTML: &[u8] = include_bytes!("../../admin-ui/dist/index.html");
const APP_JS: &[u8] = include_bytes!("../../admin-ui/dist/pkg/reservoir_admin_ui.js");
const APP_WASM: &[u8] = include_bytes!("../../admin-ui/dist/pkg/reservoir_admin_ui_bg.wasm");

/// Message content is cut to this many characters in the graph view.
const MAX_CONTENT_CHARS: usize = 2000;

#[derive(Parser, Debug)]
#[command(author, version, about = "Serve the node graph view in a browser", long_about = None)]
pub struct AdminSubCommand {
    /// Address to listen on (localhost only by default)
    #[arg(short, long, default_value = "127.0.0.1")]
    pub bind: String,
    /// Port to listen on
    #[arg(short, long, default_value = "3939")]
    pub port: u16,
}

#[derive(serde::Serialize)]
struct GraphResponse {
    nodes: Vec<GraphNode>,
    synapses: Vec<GraphSynapse>,
}

fn truncate_content(nodes: &mut [GraphNode]) {
    for node in nodes {
        if let Some(content) = &mut node.content {
            if let Some((i, _)) = content.char_indices().nth(MAX_CONTENT_CHARS) {
                content.truncate(i);
            }
        }
    }
}

fn asset(content_type: &'static str, body: &'static [u8]) -> Response {
    ([(header::CONTENT_TYPE, content_type)], body).into_response()
}

async fn graph() -> Result<Json<GraphResponse>, (StatusCode, String)> {
    let (mut nodes, synapses) = get_graph(connect).await.map_err(|e| {
        error!("Error loading graph: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    truncate_content(&mut nodes);
    Ok(Json(GraphResponse { nodes, synapses }))
}

fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { asset("text/html; charset=utf-8", INDEX_HTML) }),
        )
        .route(
            "/pkg/reservoir_admin_ui.js",
            get(|| async { asset("text/javascript", APP_JS) }),
        )
        .route(
            "/pkg/reservoir_admin_ui_bg.wasm",
            get(|| async { asset("application/wasm", APP_WASM) }),
        )
        .route("/api/graph", get(graph))
}

pub async fn run(cmd: &AdminSubCommand) -> Result<(), Error> {
    let listener = tokio::net::TcpListener::bind((cmd.bind.as_str(), cmd.port)).await?;
    info!("Admin view on http://{}", listener.local_addr()?);
    axum::serve(listener, router()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    async fn get_path(path: &str) -> Response {
        router()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn serves_embedded_frontend() {
        for (path, content_type) in [
            ("/", "text/html; charset=utf-8"),
            ("/pkg/reservoir_admin_ui.js", "text/javascript"),
            ("/pkg/reservoir_admin_ui_bg.wasm", "application/wasm"),
        ] {
            let res = get_path(path).await;
            assert_eq!(res.status(), StatusCode::OK, "{path}");
            assert_eq!(res.headers()[header::CONTENT_TYPE], content_type, "{path}");
        }
    }

    #[tokio::test]
    async fn unknown_path_is_not_found() {
        assert_eq!(get_path("/nope").await.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn long_content_is_truncated_on_a_char_boundary() {
        let mut nodes = vec![GraphNode {
            id: 1,
            trace_id: "t".into(),
            partition: "p".into(),
            instance: "i".into(),
            role: "user".into(),
            content: Some("é".repeat(MAX_CONTENT_CHARS + 10)),
            timestamp: 0,
        }];
        truncate_content(&mut nodes);
        assert_eq!(
            nodes[0].content.as_ref().unwrap().chars().count(),
            MAX_CONTENT_CHARS
        );
    }

    #[test]
    fn parse_admin_defaults() {
        use crate::args::{Args, SubCommands};
        let args = Args::try_parse_from(["reservoir", "admin"]).unwrap();
        match args.subcmd {
            Some(SubCommands::Admin(cmd)) => {
                assert_eq!(cmd.bind, "127.0.0.1");
                assert_eq!(cmd.port, 3939);
            }
            _ => panic!("expected admin subcommand"),
        }
    }
}
