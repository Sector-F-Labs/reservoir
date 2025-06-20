use anyhow::Error;
use bytes::Bytes;
use http::{header, Method, Request, Uri};
use http_body_util::{BodyExt, Full};
use hyper_tls::HttpsConnector;
use hyper::Client;
use std::env;
use tracing::error;

use crate::clients::openai::model_info::openai_embeddings_url;

pub async fn handle_embeddings(body: Bytes) -> Result<Bytes, Error> {
    let https = HttpsConnector::new();
    let client: Client<_, _> = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new()).build(https);

    let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
    let url = openai_embeddings_url();
    let uri: Uri = url.parse()?;
    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .body(Full::new(body))?;

    let resp = client.request(req).await?;
    let status = resp.status();
    let bytes = resp.into_body().collect().await?.to_bytes();

    if !status.is_success() {
        let text = String::from_utf8_lossy(&bytes);
        error!("Embeddings error {}: {}", status, text);
        return Err(Error::msg(format!("Embeddings API error {}: {}", status, text)));
    }
    Ok(bytes)
}
