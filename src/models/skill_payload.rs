use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillPayload {
    intent: Intent,
    pub user_request: UserRequest,
    bot: Bot,
    action: Action,
}

#[derive(Deserialize)]
struct Intent {
    id: String,
    name: String,
}

#[derive(Deserialize)]
pub struct UserRequest {
    timezone: String,
    params: serde_json::Value,
    block: Block,
    pub utterance: String,
    lang: Option<String>,
    user: User,
}

#[derive(Deserialize)]
struct Block {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct User {
    id: String,
    #[serde(rename = "type")]
    user_type: String,
    properties: serde_json::Value,
}

#[derive(Deserialize)]
struct Bot {
    id: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Action {
    name: String,
    client_extra: Option<serde_json::Value>,
    params: serde_json::Value,
    id: String,
    detail_params: serde_json::Value,
}
