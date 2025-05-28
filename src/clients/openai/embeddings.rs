use anyhow::Error;
use http::header;
use hyper::{Request, Method, Uri};
use http_body_util::{BodyExt, Full};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{error};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/embeddings"; // Assuming you meant the embeddings endpoint

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Embedding {
    object: String,
    index: i32,
    pub embedding: Vec<f32>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct EmbeddingResponse {
    // Define this struct according to the API response structure for embeddings
    object: String,
    pub data: Vec<Embedding>,
}

#[derive(Serialize)]
struct EmbeddingRequest {
    input: String,
    model: String,
}

pub async fn get_embeddings_for_text(text: &str) -> Result<Vec<Embedding>, Error> {
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new()).build_http();
    let api_key = env::var("OPENAI_API_KEY")?;

    // Set up the request payload
    let request_body = EmbeddingRequest {
        input: text.to_string(),
        model: "text-embedding-ada-002".to_string(), // Replace with the appropriate model name
    };

    let body_json = serde_json::to_string(&request_body)?;
    let uri: Uri = OPENAI_API_URL.parse()?;

    // Send the request and handle response
    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .body(Full::new(Bytes::from(body_json)))?;

    let response = client.request(req).await;

    match response {
        Ok(res) => {
            let status = res.status();
            let body_bytes = match res.into_body().collect().await {
                Ok(collected) => collected.to_bytes(),
                Err(e) => {
                    error!("Error reading response body: {}", e);
                    return Err(Error::msg(format!("Failed to read response body: {}", e)));
                }
            };

            let response_text = match String::from_utf8(body_bytes.to_vec()) {
                Ok(text) => text,
                Err(e) => {
                    error!("Error converting response to string: {}", e);
                    return Err(Error::msg(format!("Failed to convert response to string: {}", e)));
                }
            };

            if status.is_success() {
                let embeddings: EmbeddingResponse = serde_json::from_str(&response_text)?;
                Ok(embeddings.data)
            } else {
                error!("Error: Received non-success status code {}", status);
                let error_response: serde_json::Value = serde_json::from_str(&response_text)
                    .unwrap_or_else(|_| serde_json::json!({"error": "Failed to parse error response"}));
                error!("Error response: {:?}", error_response);
                return Err(Error::msg(format!("API error {}: {}", status, response_text)));
            }
        }
        Err(e) => {
            error!("Error sending request: {}", e);
            return Err(Error::msg(format!("Failed to send request to OpenAI API: {}", e)));
        }
    }
}
