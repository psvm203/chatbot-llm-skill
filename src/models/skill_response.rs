use serde::Serialize;

#[derive(Serialize)]
pub struct SkillResponse {
    pub version: String,
    pub template: Template,
}

#[derive(Serialize)]
pub struct Template {
    pub outputs: Vec<Output>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub simple_text: SimpleText,
}

#[derive(Serialize)]
pub struct SimpleText {
    pub text: String,
}
