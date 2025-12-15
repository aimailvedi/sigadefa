use crate::app::{Config, Message, Role};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("OpenAI API key not configured")]
    MissingApiKey,
    #[error("Network error: {0}")]
    Network(String),
    #[error("Unexpected response: {0}")]
    Response(String),
}

pub async fn send_openai(config: &Config, history: Vec<Message>) -> Result<String, LlmError> {
    if config.openai_api_key.trim().is_empty() {
        return Err(LlmError::MissingApiKey);
    }

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(config.request_timeout_sec))
        .build()
        .map_err(|e| LlmError::Network(e.to_string()))?;

    let messages: Vec<OpenAiMessage> = history
        .into_iter()
        .map(|m| OpenAiMessage {
            role: map_role(&m.role),
            content: m.content,
        })
        .collect();

    let body = OpenAiRequest {
        model: config.openai_model.clone(),
        messages,
    };

    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(config.openai_api_key.trim())
        .json(&body)
        .send()
        .await
        .map_err(|e| LlmError::Network(e.to_string()))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| LlmError::Network(e.to_string()))?;
    if !status.is_success() {
        return Err(LlmError::Response(format!("HTTP {}: {}", status, text)));
    }

    let parsed: OpenAiResponse = serde_json::from_str(&text)
        .map_err(|e| LlmError::Response(format!("Parsing error: {}", e)))?;

    if let Some(choice) = parsed.choices.first() {
        if let Some(content) = &choice.message.content {
            return Ok(content.clone());
        }
    }

    Err(LlmError::Response("No content in response".to_string()))
}

pub async fn send_ollama(config: &Config, history: Vec<Message>) -> Result<String, LlmError> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(config.request_timeout_sec))
        .build()
        .map_err(|e| LlmError::Network(e.to_string()))?;

    let prompt = build_prompt(history);

    let req = OllamaRequest {
        model: config.ollama_model.clone(),
        prompt,
        stream: false,
    };

    let resp = client
        .post("http://localhost:11434/api/generate")
        .json(&req)
        .send()
        .await
        .map_err(|e| LlmError::Network(e.to_string()))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| LlmError::Network(e.to_string()))?;
    if !status.is_success() {
        return Err(LlmError::Response(format!("HTTP {}: {}", status, text)));
    }

    let parsed: OllamaResponse = serde_json::from_str(&text)
        .map_err(|e| LlmError::Response(format!("Parsing error: {}", e)))?;

    Ok(parsed.response)
}

fn build_prompt(history: Vec<Message>) -> String {
    let mut lines = Vec::new();
    for message in history {
        let role = match message.role {
            Role::User => "USER",
            Role::Assistant => "ASSISTANT",
            Role::System => "SYSTEM",
        };
        lines.push(format!("{}: {}", role, message.content));
    }
    lines.join("\n")
}

fn map_role(role: &Role) -> String {
    match role {
        Role::User => "user".to_string(),
        Role::Assistant => "assistant".to_string(),
        Role::System => "system".to_string(),
    }
}

#[derive(Debug, Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
}

#[derive(Debug, Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessageResponse,
}

#[derive(Debug, Deserialize)]
struct OpenAiMessageResponse {
    content: Option<String>,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}
