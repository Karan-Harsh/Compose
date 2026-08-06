use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::error::app_error::{AppError, AppResult};
use crate::services::ai::provider::AiProvider;
use crate::services::ai::types::{
    AiCompletionRequest, AiCompletionResponse, ProviderCapabilities, ProviderInfo, ProviderKind,
};

const DEFAULT_BASE_URL: &str = "https://openrouter.ai/api/v1";
const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";

#[derive(Debug, Clone)]
pub struct OpenRouterConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
    pub app_name: String,
    pub site_url: String,
}

impl OpenRouterConfig {
    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("OPENROUTER_API_KEY").ok()?;
        let api_key = api_key.trim().to_string();

        if api_key.is_empty() || api_key.contains("your-key-here") {
            return None;
        }

        Some(Self {
            api_key,
            model: std::env::var("OPENROUTER_MODEL")
                .unwrap_or_else(|_| DEFAULT_MODEL.to_string())
                .trim()
                .to_string(),
            base_url: std::env::var("OPENROUTER_BASE_URL")
                .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
                .trim()
                .trim_end_matches('/')
                .to_string(),
            app_name: std::env::var("OPENROUTER_APP_NAME")
                .unwrap_or_else(|_| "TypeFlow".to_string()),
            site_url: std::env::var("OPENROUTER_SITE_URL")
                .unwrap_or_else(|_| "https://github.com/typeflow/typeflow".to_string()),
        })
    }
}

#[derive(Debug)]
pub struct OpenRouterProvider {
    config: OpenRouterConfig,
    client: Client,
}

impl OpenRouterProvider {
    pub fn new(config: OpenRouterConfig) -> AppResult<Self> {
        let client = Client::builder()
            .build()
            .map_err(|error| AppError::Ai(format!("failed to build HTTP client: {error}")))?;

        Ok(Self { config, client })
    }
}

impl AiProvider for OpenRouterProvider {
    fn id(&self) -> &str {
        "openrouter"
    }

    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            id: self.id().to_string(),
            kind: ProviderKind::OpenRouter,
            display_name: "OpenRouter".to_string(),
            configured: true,
            capabilities: ProviderCapabilities {
                supports_streaming: false,
                supports_system_prompt: true,
            },
        }
    }

    fn complete(&self, request: &AiCompletionRequest) -> AppResult<AiCompletionResponse> {
        let model = request
            .model
            .clone()
            .unwrap_or_else(|| self.config.model.clone());

        let payload = OpenRouterChatRequest {
            model: model.clone(),
            messages: request
                .messages
                .iter()
                .map(|message| OpenRouterMessage {
                    role: message.role.clone(),
                    content: message.content.clone(),
                })
                .collect(),
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.config.api_key)).map_err(|_| {
                AppError::Ai("invalid OPENROUTER_API_KEY header value".to_string())
            })?,
        );
        if let Ok(value) = HeaderValue::from_str(&self.config.site_url) {
            headers.insert("HTTP-Referer", value);
        }
        if let Ok(value) = HeaderValue::from_str(&self.config.app_name) {
            headers.insert("X-Title", value);
        }

        let url = format!("{}/chat/completions", self.config.base_url);
        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .map_err(|error| AppError::Ai(format!("OpenRouter request failed: {error}")))?;

        let status = response.status();
        let body = response
            .text()
            .map_err(|error| AppError::Ai(format!("failed to read OpenRouter response: {error}")))?;

        if !status.is_success() {
            return Err(AppError::Ai(format!(
                "OpenRouter returned {status}: {body}"
            )));
        }

        let parsed: OpenRouterChatResponse = serde_json::from_str(&body).map_err(|error| {
            AppError::Ai(format!("failed to parse OpenRouter response: {error}"))
        })?;

        let choice = parsed.choices.first().ok_or_else(|| {
            AppError::Ai("OpenRouter response contained no choices".to_string())
        })?;

        let content = choice
            .message
            .content
            .clone()
            .unwrap_or_default()
            .trim()
            .to_string();

        if content.is_empty() {
            return Err(AppError::Ai(
                "OpenRouter returned an empty completion".to_string(),
            ));
        }

        Ok(AiCompletionResponse {
            provider_id: self.id().to_string(),
            model: parsed.model.unwrap_or(model),
            content,
            finish_reason: choice
                .finish_reason
                .clone()
                .unwrap_or_else(|| "stop".to_string()),
        })
    }
}

#[derive(Debug, Serialize)]
struct OpenRouterChatRequest {
    model: String,
    messages: Vec<OpenRouterMessage>,
}

#[derive(Debug, Serialize)]
struct OpenRouterMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChatResponse {
    model: Option<String>,
    choices: Vec<OpenRouterChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoice {
    message: OpenRouterResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponseMessage {
    content: Option<String>,
}
