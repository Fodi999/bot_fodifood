//! Groq API Integration - Llama 3.1 70B
//! Ultra-fast LLM inference for FodiFood AI

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use anyhow::{Result, Context};

/// Groq chat request structure
#[derive(Serialize, Debug)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    top_p: Option<f32>,
}

/// Message in conversation
#[derive(Serialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Groq API response
#[derive(Deserialize, Debug)]
struct GroqResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

/// Individual choice in response
#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageResponse,
    finish_reason: Option<String>,
}

/// Message content from Groq
#[derive(Deserialize, Debug)]
struct MessageResponse {
    role: String,
    content: String,
}

/// Token usage statistics
#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// Available Groq models
#[derive(Debug, Clone)]
pub enum GroqModel {
    /// Llama 3.3 70B - Most powerful, best for complex reasoning
    Llama70B,
    /// Llama 3.1 8B - Faster, good for simple tasks
    Llama8B,
    /// Mixtral 8x7B - Good balance
    Mixtral,
}

impl GroqModel {
    fn as_str(&self) -> &str {
        match self {
            GroqModel::Llama70B => "llama-3.3-70b-versatile",  // Updated to 3.3
            GroqModel::Llama8B => "llama-3.1-8b-instant",
            GroqModel::Mixtral => "mixtral-8x7b-32768",
        }
    }
}

/// Groq client configuration
pub struct GroqConfig {
    pub model: GroqModel,
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
}

impl Default for GroqConfig {
    fn default() -> Self {
        Self {
            model: GroqModel::Llama70B,
            temperature: 0.7,
            max_tokens: 2048,
            top_p: 0.9,
        }
    }
}

/// Query Groq with simple text prompt
pub async fn query_groq(prompt: &str) -> Result<String> {
    query_groq_with_config(prompt, &GroqConfig::default()).await
}

/// Query Groq with custom configuration
pub async fn query_groq_with_config(prompt: &str, config: &GroqConfig) -> Result<String> {
    let messages = vec![Message {
        role: "user".to_string(),
        content: prompt.to_string(),
    }];
    
    query_groq_messages(&messages, config).await
}

/// Query Groq with conversation history
pub async fn query_groq_messages(messages: &[Message], config: &GroqConfig) -> Result<String> {
    dotenvy::dotenv().ok();
    
    let api_key = env::var("GROQ_API_KEY")
        .context("GROQ_API_KEY not found in environment. Add it to .env or Secrets.toml")?;

    tracing::debug!("🧠 Querying Groq {} with {} messages", config.model.as_str(), messages.len());

    let client = Client::new();
    let body = GroqRequest {
        model: config.model.as_str().to_string(),
        messages: messages.to_vec(),
        temperature: Some(config.temperature),
        max_tokens: Some(config.max_tokens),
        top_p: Some(config.top_p),
    };

    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .context("Failed to send request to Groq API")?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        tracing::error!("❌ Groq API error {}: {}", status, text);
        return Err(anyhow::anyhow!("Groq API error {}: {}", status, text));
    }

    let response_json: GroqResponse = res.json().await
        .context("Failed to parse Groq response")?;

    if let Some(usage) = &response_json.usage {
        tracing::debug!(
            "📊 Groq usage: {} prompt + {} completion = {} total tokens",
            usage.prompt_tokens,
            usage.completion_tokens,
            usage.total_tokens
        );
    }

    let content = response_json.choices
        .first()
        .map(|c| c.message.content.clone())
        .context("No response from Groq")?;

    tracing::info!("✅ Groq response received ({} chars)", content.len());
    Ok(content)
}

/// Query Groq with system prompt for better context
pub async fn query_groq_with_system(
    system_prompt: &str,
    user_prompt: &str,
    config: &GroqConfig
) -> Result<String> {
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: user_prompt.to_string(),
        },
    ];
    
    query_groq_messages(&messages, config).await
}

/// Stream response from Groq (for future real-time features)
/// Note: Currently returns full response, streaming to be implemented
pub async fn query_groq_stream(prompt: &str) -> Result<String> {
    // TODO: Implement Server-Sent Events streaming
    tracing::warn!("⚠️ Streaming not yet implemented, using regular query");
    query_groq(prompt).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires GROQ_API_KEY
    async fn test_groq_query() {
        let result = query_groq("Say hello in 5 words").await;
        assert!(result.is_ok());
        println!("Groq response: {}", result.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn test_groq_with_system() {
        let config = GroqConfig {
            model: GroqModel::Llama8B,
            temperature: 0.5,
            max_tokens: 100,
            top_p: 0.9,
        };

        let result = query_groq_with_system(
            "You are a helpful restaurant AI assistant",
            "What's the best pizza topping?",
            &config
        ).await;

        assert!(result.is_ok());
        println!("Groq response: {}", result.unwrap());
    }
}
