use anyhow::Error;
use bytes::Bytes;
use http::header;
use http_body_util::{BodyExt, Full};
use hyper::{Method, Request, Uri};
use hyper_tls::HttpsConnector;
use tracing::{debug, error, info};

use crate::utils::compress_system_context;

use super::{
    model_info::ModelInfo,
    types::{ChatRequest, ChatResponse},
};

pub async fn get_completion_message(
    model_info: &ModelInfo,
    chat_request: &ChatRequest,
) -> Result<ChatResponse, Error> {
    info!("Getting completion with model {}", model_info.name);

    // Validate OpenAI model names
    if model_info.base_url.contains("api.openai.com") {
        let valid_openai_models = [
            "gpt-4",
            "gpt-4-turbo",
            "gpt-4o",
            "gpt-4o-mini",
            "gpt-3.5-turbo",
            "gpt-4o-search-preview",
        ];
        if !valid_openai_models.contains(&model_info.name.as_str()) {
            error!(
                "Invalid OpenAI model name: '{}'. Valid models are: {:?}",
                model_info.name, valid_openai_models
            );
            return Err(Error::msg(format!(
                "Invalid OpenAI model name: '{}'. Valid models are: {:?}",
                model_info.name, valid_openai_models
            )));
        }
    }

    let https = HttpsConnector::new();
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(https);

    let context = compress_system_context(&chat_request.messages);
    let chat_request = ChatRequest::new(model_info.name.clone(), context);

    let input_body = match serde_json::to_string(&chat_request) {
        Ok(b) => b,
        Err(e) => {
            error!("Failed to serialize chat request model: {}", e);
            return Err(Error::msg(format!(
                "Failed to serialize chat request: {}",
                e
            )));
        }
    };

    debug!(
        "Sending request to LLM API: {} -  {}\nbody:\n{}",
        input_body,
        model_info.name.clone(),
        model_info.base_url.clone(),
    );

    println!(
        "DEBUG: Attempting to connect to URL: {}",
        model_info.base_url
    );
    println!(
        "DEBUG: Model name: '{}', API key length: {}",
        model_info.name,
        if model_info.key.is_empty() {
            0
        } else {
            model_info.key.len()
        }
    );

    let uri: Uri = model_info.base_url.parse().map_err(|e| {
        error!("Failed to parse URL '{}': {}", model_info.base_url, e);
        Error::msg(format!("Invalid URL '{}': {}", model_info.base_url, e))
    })?;

    let req = Request::builder()
        .method(Method::POST)
        .uri(&uri)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", model_info.key))
        .body(Full::new(Bytes::from(input_body)))?;

    let response = client.request(req).await;

    let response = match response {
        Ok(resp) => resp,
        Err(e) => {
            error!("Error sending request to LLM API: {}", e);
            let error_msg = if model_info.base_url.contains("api.openai.com") {
                if model_info.key.is_empty() {
                    format!(
                        "Failed to connect to OpenAI API: {}. Missing API key! Please set OPENAI_API_KEY environment variable.",
                        e
                    )
                } else {
                    format!(
                        "Failed to connect to OpenAI API: {}. Check your API key and network connection. Using model '{}' at '{}'",
                        e, model_info.name, model_info.base_url
                    )
                }
            } else {
                format!(
                    "Failed to send request to LLM API: {}. Please check if the service is running at {}",
                    e, model_info.base_url
                )
            };
            return Err(Error::msg(error_msg));
        }
    };

    let status = response.status();
    let body_bytes = match response.into_body().collect().await {
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
            return Err(Error::msg(format!(
                "Failed to convert response to string: {}",
                e
            )));
        }
    };

    if !status.is_success() {
        error!(
            "LLM API returned error status {}: {}",
            status, response_text
        );
        return Err(Error::msg(format!(
            "LLM API error {}: {}",
            status, response_text
        )));
    }

    match ChatResponse::from_json(&response_text) {
        Ok(r) => Ok(r),
        Err(e) => {
            error!(
                "Error parsing response JSON: {}\nRaw response: {}",
                e, response_text
            );
            Err(Error::msg(format!(
                "Failed to parse response JSON: {}\nRaw response: {}",
                e, response_text
            )))
        }
    }
}
