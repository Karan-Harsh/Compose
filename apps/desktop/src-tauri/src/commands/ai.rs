use serde::{Deserialize, Serialize};
use tauri::State;

use crate::services::ai::types::{AiCompletionRequest, AiMessage};
use crate::state::app_state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInfoResponse {
    pub id: String,
    pub kind: String,
    pub display_name: String,
    pub configured: bool,
    pub capabilities: ProviderCapabilitiesResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCapabilitiesResponse {
    pub supports_streaming: bool,
    pub supports_system_prompt: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandDefinitionResponse {
    pub id: String,
    pub description: String,
    pub requires_input: bool,
    pub requires_context: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetActiveAiProviderRequest {
    pub provider_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteAiRequest {
    pub provider_id: Option<String>,
    pub model: Option<String>,
    pub command_id: Option<String>,
    pub messages: Vec<AiMessageRequest>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiMessageRequest {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiCompletionResponsePayload {
    pub provider_id: String,
    pub model: String,
    pub content: String,
    pub finish_reason: String,
}

fn provider_kind_label(kind: &crate::services::ai::types::ProviderKind) -> String {
    match kind {
        crate::services::ai::types::ProviderKind::Stub => "stub".to_string(),
        crate::services::ai::types::ProviderKind::OpenRouter => "openrouter".to_string(),
        crate::services::ai::types::ProviderKind::OpenAi => "openai".to_string(),
        crate::services::ai::types::ProviderKind::Anthropic => "anthropic".to_string(),
        crate::services::ai::types::ProviderKind::Gemini => "gemini".to_string(),
        crate::services::ai::types::ProviderKind::Ollama => "ollama".to_string(),
    }
}

fn to_provider_response(
    info: crate::services::ai::types::ProviderInfo,
) -> ProviderInfoResponse {
    ProviderInfoResponse {
        id: info.id,
        kind: provider_kind_label(&info.kind),
        display_name: info.display_name,
        configured: info.configured,
        capabilities: ProviderCapabilitiesResponse {
            supports_streaming: info.capabilities.supports_streaming,
            supports_system_prompt: info.capabilities.supports_system_prompt,
        },
    }
}

#[tauri::command]
pub fn list_ai_providers(state: State<'_, AppState>) -> Result<Vec<ProviderInfoResponse>, String> {
    Ok(state
        .ai_service
        .list_providers()
        .into_iter()
        .map(to_provider_response)
        .collect())
}

#[tauri::command]
pub fn get_active_ai_provider(
    state: State<'_, AppState>,
) -> Result<ProviderInfoResponse, String> {
    state
        .ai_service
        .active_provider()
        .map(to_provider_response)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_active_ai_provider(
    state: State<'_, AppState>,
    request: SetActiveAiProviderRequest,
) -> Result<ProviderInfoResponse, String> {
    state
        .ai_service
        .set_active_provider(&request.provider_id)
        .map(to_provider_response)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_commands(
    state: State<'_, AppState>,
) -> Result<Vec<CommandDefinitionResponse>, String> {
    Ok(state
        .ai_service
        .command_registry
        .list()
        .into_iter()
        .map(|command| CommandDefinitionResponse {
            id: command.id,
            description: command.description,
            requires_input: command.requires_input,
            requires_context: command.requires_context,
        })
        .collect())
}

#[tauri::command]
pub fn complete_ai(
    state: State<'_, AppState>,
    request: CompleteAiRequest,
) -> Result<AiCompletionResponsePayload, String> {
    let completion = AiCompletionRequest {
        provider_id: request.provider_id,
        model: request.model,
        command_id: request.command_id,
        messages: request
            .messages
            .into_iter()
            .map(|message| AiMessage {
                role: message.role,
                content: message.content,
            })
            .collect(),
    };

    let command_label = completion
        .command_id
        .clone()
        .unwrap_or_else(|| "none".to_string());
    let messages_preview = completion
        .messages
        .iter()
        .map(|message| format!("[{}] {}", message.role, message.content))
        .collect::<Vec<_>>()
        .join("\n");
    eprintln!("TypeFlow: complete_ai command={command_label}\n{messages_preview}");

    match state.ai_service.complete(completion) {
        Ok(response) => {
            eprintln!(
                "TypeFlow: complete_ai ok provider={} model={}\n{}",
                response.provider_id, response.model, response.content
            );
            Ok(AiCompletionResponsePayload {
                provider_id: response.provider_id,
                model: response.model,
                content: response.content,
                finish_reason: response.finish_reason,
            })
        }
        Err(error) => {
            let message = error.to_string();
            eprintln!("TypeFlow: complete_ai failed command={command_label}: {message}");
            Err(message)
        }
    }
}
