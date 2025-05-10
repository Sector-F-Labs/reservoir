use anyhow::Error;
use http::header;
use tracing::{debug, error};

use crate::utils::compress_system_context;

use super::{model_info::ModelInfo, types::{ChatRequest, ChatResponse}};


pub async fn get_completion_message(
    model_info: &ModelInfo,
    chat_request: &ChatRequest,
) -> Result<ChatResponse, Error> {
    let client = reqwest::Client::new();

    let context = compress_system_context(&chat_request.messages);
    let chat_request = ChatRequest::new(model_info.name.clone(), context);

    let body = match serde_json::to_string(&chat_request) {
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
        body,
        model_info.name.clone(),
        model_info.base_url.clone(),
    );

    let response = client
        .post(model_info.base_url.clone())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", model_info.key))
        .body(body)
        .send()
        .await;

    let response = match response {
        Ok(resp) => resp,
        Err(e) => {
            error!("Error sending request to LLM API: {}", e);
            return Err(Error::msg(format!(
                "Failed to send request to LLM API: {}",
                e
            )));
        }
    };

    let status = response.status();
    let response_text = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            error!("Error reading response text: {}", e);
            return Err(Error::msg(format!("Failed to read response text: {}", e)));
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
