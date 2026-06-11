use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeepseekResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage,
    system_fingerprint: String,
}

#[derive(Deserialize)]
pub struct Choice {
    index: u32,
    pub message: Message,
    logprobs: Option<serde_json::Value>,
    finish_reason: String,
}

#[derive(Deserialize)]
pub struct Message {
    role: String,
    pub content: String,
    reasoning_content: String,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
    prompt_tokens_details: PromptTokensDetails,
    completion_tokens_details: CompletionTokensDetails,
    prompt_cache_hit_tokens: u32,
    prompt_cache_miss_tokens: u32,
}

#[derive(Deserialize)]
pub struct PromptTokensDetails {
    cached_tokens: u32,
}

#[derive(Deserialize)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: u32,
}
