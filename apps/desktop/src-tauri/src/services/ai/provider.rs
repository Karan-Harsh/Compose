use crate::error::app_error::AppResult;
use crate::services::ai::types::{AiCompletionRequest, AiCompletionResponse, ProviderInfo};

pub trait AiProvider: Send + Sync {
    fn id(&self) -> &str;
    fn info(&self) -> ProviderInfo;
    fn complete(&self, request: &AiCompletionRequest) -> AppResult<AiCompletionResponse>;
}
