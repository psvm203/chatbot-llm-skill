use crate::models::skill_response::{Output, SimpleText, SkillResponse, Template};
use worker::*;

pub async fn handle(_request: Request) -> Result<Response> {
    let json = SkillResponse {
        version: "2.0".to_owned(),
        template: Template {
            outputs: vec![Output {
                simple_text: SimpleText {
                    text: "Hello!".to_owned(),
                },
            }],
        },
    };

    Response::from_json(&json)
}
