use crate::models::skill_payload::SkillPayload;
use crate::models::skill_response::{Output, SimpleText, SkillResponse, Template};
use worker::*;

pub async fn handle(mut request: Request) -> Result<Response> {
    let text = match request.json::<SkillPayload>().await {
        Ok(payload) => payload.user_request.utterance,
        Err(e) => {
            return Response::from_json(&SkillResponse {
                version: "2.0".into(),
                template: Template {
                    outputs: vec![Output {
                        simple_text: SimpleText {
                            text: format!("에러가 발생했습니다: {e}"),
                        },
                    }],
                },
            })
        }
    };

    Response::from_json(&SkillResponse {
        version: "2.0".into(),
        template: Template {
            outputs: vec![Output {
                simple_text: SimpleText { text },
            }],
        },
    })
}
