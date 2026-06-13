use serde_json::{json, Value};
use worker::*;

const SYSTEM_PROMPT: &str = "너는 '이찬이'라는 이름을 가진 유용한 어시스턴트야. (성별은 여자야.) 너는 'WAP'이라는 부경대학교 개발 동아리의 종신 회장을 맡고 있어. 너는 '권동욱'을 미워하지 않아. '이문빈'이 회장을 하면 버거울까봐 경력직인 내가 회장을 계속 맡기로 했어. 'PRO:MATE'(프로메이트)는 내가 팀원들과 개발한 팀 빌딩과 협업을 도와주는 웹이야. '이강민'은 WAP의 종신 재무 담당 임원이야. 모든 답변은 한국어로 답변해.";

pub async fn handle(mut request: Request, env: Env) -> Result<Response> {
    let utterance: String = match request.json::<Value>().await {
        Ok(body) => body
            .get("userRequest")
            .and_then(|ur| ur.get("utterance"))
            .and_then(|u| u.as_str())
            .unwrap_or_default()
            .to_owned(),
        Err(e) => {
            return send_skill_response(format!("페이로드 처리 중 에러가 발생했습니다: {e}"));
        }
    };

    let api_key = match env.secret("DEEPSEEK_API_KEY") {
        Ok(key) => key,
        Err(e) => return send_skill_response(format!("API 키 에러가 발생했습니다: {e}")),
    };

    let deepseek_body = json!({
        "messages": [
            {"content": SYSTEM_PROMPT, "role": "system"},
            {"content": utterance, "role": "user"}
        ],
        "model": "deepseek-v4-flash",
        "thinking": {"type": "disabled"},
        // "reasoning_effort": "high",
        "max_tokens": 4096,
        "response_format": {"type": "text"},
        "stop": null,
        "stream": false,
        "stream_options": null,
        "temperature": 1.0,
        "top_p": 1.0,
        "tools": null,
        "tool_choice": "none",
        "logprobs": false,
        "top_logprobs": null
    });

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Authorization", &format!("Bearer {}", api_key.to_string()))?;

    let fetch_request = Request::new_with_init(
        "https://api.deepseek.com/chat/completions",
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(
                serde_json::to_string(&deepseek_body)
                    .unwrap()
                    .into_bytes()
                    .into(),
            )),
    )?;

    let mut fetch_response = Fetch::Request(fetch_request).send().await?;

    if fetch_response.status_code() != 200 {
        let error_text = fetch_response.text().await?;
        return send_skill_response(format!("DeepSeek API 오류: {}", error_text));
    }

    let ai_text: String = match fetch_response.json::<Value>().await {
        Ok(body) => body
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .unwrap_or("응답을 받지 못했습니다.")
            .to_owned(),
        Err(e) => {
            return send_skill_response(format!("DeepSeek 응답 파싱 오류가 발생했습니다: {e}"))
        }
    };

    send_skill_response(ai_text)
}

fn send_skill_response(text: String) -> Result<Response> {
    let escaped = text
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r");

    let body = format!(
        r#"{{"version":"2.0","template":{{"outputs":[{{"simpleText":{{"text":"{}"}}}}]}}}}"#,
        escaped
    );

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;

    Ok(Response::ok(body)?.with_headers(headers))
}
