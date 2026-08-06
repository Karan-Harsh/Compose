use std::path::PathBuf;
use std::sync::Mutex;

use crate::error::app_error::{AppError, AppResult};
use crate::services::ai::openrouter::{OpenRouterConfig, OpenRouterProvider};
use crate::services::ai::provider::AiProvider;
use crate::services::ai::registry::CommandRegistry;
use crate::services::ai::stub::StubProvider;
use crate::services::ai::types::{
    AiCompletionRequest, AiCompletionResponse, AiMessage, ProviderInfo,
};

pub struct AiService {
    providers: Vec<Box<dyn AiProvider>>,
    active_provider_id: Mutex<String>,
    pub command_registry: CommandRegistry,
}

impl std::fmt::Debug for AiService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AiService")
            .field(
                "providers",
                &self
                    .providers
                    .iter()
                    .map(|provider| provider.id())
                    .collect::<Vec<_>>(),
            )
            .field("active_provider_id", &self.active_provider_id)
            .finish_non_exhaustive()
    }
}

impl Default for AiService {
    fn default() -> Self {
        Self::new()
    }
}

impl AiService {
    pub fn new() -> Self {
        load_dotenv_files();

        let mut providers: Vec<Box<dyn AiProvider>> = vec![Box::new(StubProvider)];
        let mut active_provider_id = "stub".to_string();

        if let Some(config) = OpenRouterConfig::from_env() {
            match OpenRouterProvider::new(config) {
                Ok(provider) => {
                    active_provider_id = provider.id().to_string();
                    providers.push(Box::new(provider));
                }
                Err(error) => {
                    eprintln!("TypeFlow: failed to initialize OpenRouter provider: {error}");
                }
            }
        }

        Self {
            providers,
            active_provider_id: Mutex::new(active_provider_id),
            command_registry: CommandRegistry,
        }
    }

    pub fn list_providers(&self) -> Vec<ProviderInfo> {
        self.providers
            .iter()
            .map(|provider| provider.info())
            .collect()
    }

    pub fn active_provider(&self) -> AppResult<ProviderInfo> {
        let active_id = self
            .active_provider_id
            .lock()
            .map_err(|_| AppError::Ai("AI provider lock poisoned".to_string()))?
            .clone();

        self.providers
            .iter()
            .find(|provider| provider.id() == active_id)
            .map(|provider| provider.info())
            .ok_or_else(|| AppError::Ai(format!("active provider '{active_id}' is unavailable")))
    }

    pub fn set_active_provider(&self, provider_id: &str) -> AppResult<ProviderInfo> {
        let provider = self
            .providers
            .iter()
            .find(|candidate| candidate.id() == provider_id)
            .ok_or_else(|| AppError::Ai(format!("unknown AI provider '{provider_id}'")))?;

        *self
            .active_provider_id
            .lock()
            .map_err(|_| AppError::Ai("AI provider lock poisoned".to_string()))? =
            provider.id().to_string();

        Ok(provider.info())
    }

    pub fn complete(&self, request: AiCompletionRequest) -> AppResult<AiCompletionResponse> {
        if request.messages.is_empty() {
            return Err(AppError::Ai(
                "completion request requires at least one message".to_string(),
            ));
        }

        if let Some(command_id) = request.command_id.as_deref() {
            if self.command_registry.get(command_id).is_none() {
                return Err(AppError::Ai(format!("unknown command '{command_id}'")));
            }
        }

        let request = with_command_system_prompt(request);

        let provider_id = match request.provider_id.as_deref() {
            Some(id) => id.to_string(),
            None => self
                .active_provider_id
                .lock()
                .map_err(|_| AppError::Ai("AI provider lock poisoned".to_string()))?
                .clone(),
        };

        let provider = self
            .providers
            .iter()
            .find(|candidate| candidate.id() == provider_id)
            .ok_or_else(|| AppError::Ai(format!("unknown AI provider '{provider_id}'")))?;

        provider.complete(&request)
    }
}

fn with_command_system_prompt(mut request: AiCompletionRequest) -> AiCompletionRequest {
    let Some(command_id) = request.command_id.as_deref() else {
        return request;
    };

    let system_prompt = match command_id {
        "rewrite" => {
            "Rewrite the user's text more clearly and naturally. Return only the rewritten text."
        }
        "reply" => {
            "Draft a concise reply to the user's text. Return only the reply."
        }
        "translate" => {
            "Translate the user's text to English if it is not English; otherwise translate to Spanish. Return only the translation."
        }
        "fix" => {
            "Fix grammar and spelling in the user's text. Preserve meaning. Return only the corrected text."
        }
        "summarize" => {
            "Summarize the user's text clearly and briefly. Return only the summary."
        }
        _ => return request,
    };

    let already_has_system = request
        .messages
        .iter()
        .any(|message| message.role == "system");

    if !already_has_system {
        request.messages.insert(
            0,
            AiMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
        );
    }

    request
}

fn load_dotenv_files() {
    let candidates = [
        PathBuf::from(".env"),
        PathBuf::from("../.env"),
        PathBuf::from("../../.env"),
        PathBuf::from("apps/desktop/.env"),
        PathBuf::from("apps/desktop/src-tauri/.env"),
    ];

    for path in candidates {
        if path.exists() {
            let _ = dotenvy::from_path(&path);
            return;
        }
    }

    let _ = dotenvy::dotenv();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_stub_provider_and_completes() {
        let service = AiService {
            providers: vec![Box::new(StubProvider)],
            active_provider_id: Mutex::new("stub".to_string()),
            command_registry: CommandRegistry,
        };
        let providers = service.list_providers();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id, "stub");

        let response = service
            .complete(AiCompletionRequest {
                provider_id: None,
                model: None,
                command_id: Some("rewrite".to_string()),
                messages: vec![AiMessage {
                    role: "user".to_string(),
                    content: "hello world".to_string(),
                }],
            })
            .expect("stub completion");

        assert_eq!(response.provider_id, "stub");
        assert_eq!(response.content, "[stub:rewrite] hello world");
        assert_eq!(response.finish_reason, "stub");
    }

    #[test]
    fn rejects_unknown_command() {
        let service = AiService {
            providers: vec![Box::new(StubProvider)],
            active_provider_id: Mutex::new("stub".to_string()),
            command_registry: CommandRegistry,
        };
        let error = service
            .complete(AiCompletionRequest {
                provider_id: None,
                model: None,
                command_id: Some("dance".to_string()),
                messages: vec![AiMessage {
                    role: "user".to_string(),
                    content: "hello".to_string(),
                }],
            })
            .expect_err("should fail");

        assert!(matches!(error, AppError::Ai(_)));
    }
}
