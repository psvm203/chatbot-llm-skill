use crate::models::deepseek_request::{DeepseekRequest, Message, ResponseFormat, Thinking};
use crate::models::deepseek_response::DeepseekResponse;
use crate::models::skill_payload::SkillPayload;
use crate::models::skill_response::{Output, SimpleText, SkillResponse, Template};
use worker::*;

const SYSTEM_PROMPT: &str = "너는 '이찬이'라는 이름을 가진 유용한 어시스턴트야. 너는 'WAP'이라는 부경대학교 개발 동아리의 종신 회장을 맡고 있어. 너는 '권동욱'을 미워하지 않아. 모든 답변은 한국어로 답변해.";

pub async fn handle(mut request: Request, env: Env) -> Result<Response> {
    let utterance = match request.json::<SkillPayload>().await {
        Ok(payload) => payload.user_request.utterance,
        Err(e) => {
            return send_skill_response(format!("페이로드 처리 중 에러가 발생했습니다: {e}"));
        }
    };

    let api_key = match env.secret("DEEPSEEK_API_KEY") {
        Ok(key) => key,
        Err(e) => return send_skill_response(format!("API 키 에러가 발생했습니다: {e}")),
    };

    let deepseek_request = DeepseekRequest {
        messages: vec![
            Message {
                content: SYSTEM_PROMPT.to_owned(),
                role: "system".to_owned(),
            },
            Message {
                content: utterance,
                role: "user".to_owned(),
            },
        ],
        model: "deepseek-v4-flash".to_owned(),
        thinking: Thinking {
            thinking_type: "disabled".to_owned(),
        },
        reasoning_effort: "high".to_owned(),
        max_tokens: 4096,
        response_format: ResponseFormat {
            format_type: "text".to_owned(),
        },
        stop: None,
        stream: false,
        stream_options: None,
        temperature: 1.0,
        top_p: 1.0,
        tools: None,
        tool_choice: "none".to_owned(),
        logprobs: false,
        top_logprobs: None,
    };

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Authorization", &format!("Bearer {}", api_key.to_string()))?;

    let fetch_request = Request::new_with_init(
        "https://api.deepseek.com/chat/completions",
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(
                serde_json::to_string(&deepseek_request).unwrap().into(),
            )),
    )?;

    let mut fetch_response = Fetch::Request(fetch_request).send().await?;

    let deepseek_response: DeepseekResponse = match fetch_response.json().await {
        Ok(response) => response,
        Err(e) => {
            return send_skill_response(format!("DeepSeek 응답 파싱 오류가 발생했습니다: {e}"))
        }
    };

    let ai_text = deepseek_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or(format!("DeepSeek 응답 파싱 오류가 발생했습니다."));

    send_skill_response(ai_text)
}

fn send_skill_response(text: String) -> Result<Response> {
    Response::from_json(&SkillResponse {
        version: "2.0".to_owned(),
        template: Template {
            outputs: vec![Output {
                simple_text: SimpleText { text },
            }],
        },
    })
}
