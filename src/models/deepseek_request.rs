use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DeepseekRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub thinking: Thinking,
    pub reasoning_effort: String,
    pub max_tokens: u32,
    pub response_format: ResponseFormat,
    pub stop: Option<String>,
    pub stream: bool,
    pub stream_options: Option<serde_json::Value>,
    pub temperature: f32,
    pub top_p: f32,
    pub tools: Option<serde_json::Value>,
    pub tool_choice: String,
    pub logprobs: bool,
    pub top_logprobs: Option<u32>,
}

#[derive(Serialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Thinking {
    #[serde(rename = "type")]
    pub thinking_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}
