use crate::error::app_error::{AppError, AppResult};
use crate::services::ai::provider::AiProvider;
use crate::services::ai::types::{
    AiCompletionRequest, AiCompletionResponse, ProviderCapabilities, ProviderInfo, ProviderKind,
};

#[derive(Debug, Default)]
pub struct StubProvider;

impl AiProvider for StubProvider {
    fn id(&self) -> &str {
        "stub"
    }

    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            id: self.id().to_string(),
            kind: ProviderKind::Stub,
            display_name: "Stub Provider".to_string(),
            configured: true,
            capabilities: ProviderCapabilities {
                supports_streaming: false,
                supports_system_prompt: true,
            },
        }
    }

    fn complete(&self, request: &AiCompletionRequest) -> AppResult<AiCompletionResponse> {
        let user_content = request
            .messages
            .iter()
            .rev()
            .find(|message| message.role == "user")
            .map(|message| message.content.as_str())
            .filter(|content| !content.trim().is_empty())
            .ok_or_else(|| {
                AppError::Ai("completion request requires a non-empty user message".to_string())
            })?;

        let command_label = request
            .command_id
            .as_deref()
            .unwrap_or("unspecified");

        let model = request
            .model
            .clone()
            .unwrap_or_else(|| "stub-v0".to_string());

        Ok(AiCompletionResponse {
            provider_id: self.id().to_string(),
            model,
            content: format!("[stub:{command_label}] {user_content}"),
            finish_reason: "stub".to_string(),
        })
    }
}
