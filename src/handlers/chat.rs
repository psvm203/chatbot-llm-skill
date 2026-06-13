use serde_json::{json, Value};
use worker::*;

pub async fn handle(mut request: Request, env: Env) -> Result<Response> {
    let utterance: String = match request.json::<Value>().await {
        Ok(body) => body
            .get("userRequest")
            .and_then(|ur| ur.get("utterance"))
            .and_then(|u| u.as_str())
            .unwrap_or_default()
            .to_owned(),
        Err(e) => {
            return send_skill_response(format!("페이로드 처리 중 에러가 발생했습니다: {e}"), None);
        }
    };

    let system_prompt = match env.var("PROMPT") {
        Ok(prompt) => prompt.to_string(),
        Err(e) => {
            return send_skill_response(format!("PROMPT 변수 에러가 발생했습니다: {e}"), None)
        }
    };

    let api_key = match env.secret("DEEPSEEK_API_KEY") {
        Ok(key) => key,
        Err(e) => return send_skill_response(format!("API 키 에러가 발생했습니다: {e}"), None),
    };

    let deepseek_body = json!({
        "messages": [
            {"content": system_prompt, "role": "system"},
            {"content": utterance, "role": "user"}
        ],
        "model": "deepseek-v4-flash",
        "thinking": {"type": "disabled"},
        // "reasoning_effort": "high",
        "max_tokens": 4096,
        "response_format": {"type": "json_object"},
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
        return send_skill_response(format!("DeepSeek API 오류: {}", error_text), None);
    }

    let response_text = fetch_response.text().await?;
    let (ai_text, emotion) = match serde_json::from_str::<Value>(&response_text) {
        Ok(body) => {
            let content_str = body
                .get("choices")
                .and_then(|c| c.get(0))
                .and_then(|c| c.get("message"))
                .and_then(|m| m.get("content"))
                .and_then(|c| c.as_str())
                .unwrap_or_default();

            match serde_json::from_str::<Value>(content_str) {
                Ok(parsed) => (
                    parsed
                        .get("text")
                        .and_then(|t| t.as_str())
                        .unwrap_or("응답을 받지 못했습니다.")
                        .to_owned(),
                    Some(
                        parsed
                            .get("emotion")
                            .and_then(|e| e.as_str())
                            .unwrap_or("neutral")
                            .to_owned(),
                    ),
                ),
                Err(_) => (content_str.to_owned(), None),
            }
        }
        Err(e) => {
            return send_skill_response(
                format!("DeepSeek 응답 파싱 오류: {e}\n응답 바디: {response_text}"),
                None,
            )
        }
    };

    send_skill_response(ai_text, emotion)
}

fn send_skill_response(text: String, emotion: Option<String>) -> Result<Response> {
    let mut outputs = vec![json!({"simpleText": {"text": text}})];

    if let Some(ref emotion) = emotion {
        if emotion != "neutral" {
            if let Some(image_url) = get_random_image(emotion) {
                outputs.push(json!({"simpleImage": {"imageUrl": image_url}}));
            }
        }
    }

    let mut body = json!({
        "version": "2.0",
        "template": {
            "outputs": outputs
        }
    });

    if let Some(emotion) = emotion {
        body["data"] = json!({"emotion": emotion});
    }

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;

    Ok(Response::ok(body.to_string())?.with_headers(headers))
}

fn get_random_image(emotion: &str) -> Option<&'static str> {
    let images: &[&str] = match emotion {
        "angry" => &[
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd13307003f604e7b0e6900f9ac53a43965300eb9a",
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd133070039f17e489affba0627eb1eb39695f93dd",
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd133070032df16ed7012359e344d47930e49e9310",
        ],
        "happy" => &[
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd133070039f5287469802eca457586a25a096fd31",
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd1330700315b3f4e3c2033bfd702a321ec6eda72c",
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd1330700326397d82c8691bdabf557d1536959d9c",
        ],
        "sad" => &[
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd133070034022de826f725e10df604bf1b9725cfd",
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd13307003ac8e738cb631e72fdb9a96b36413984e",
            "https://item.kakaocdn.net/do/48ae61750a25688b5b327dfb57b5f4ed66d8fd08427c1f00d04db607cc4cdc8e",
        ],
        "disgusted" => &[
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd133070037f9f127ae3ca5dc7f0f6349aebcdb3c4",
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd1330700366d8fd08427c1f00d04db607cc4cdc8e",
            "https://item.kakaocdn.net/do/c5fa54be4069ebb54786efcd1330700300df9b2a481d32ec42ae698738782a76",
        ],
        "surprised" => &[
            "https://item.kakaocdn.net/do/48ae61750a25688b5b327dfb57b5f4ed15b3f4e3c2033bfd702a321ec6eda72c",
            "https://item.kakaocdn.net/do/48ae61750a25688b5b327dfb57b5f4edfba8e3d30017c7399e19e508ee32200a",
            "https://item.kakaocdn.net/do/48ae61750a25688b5b327dfb57b5f4ed9cbcbe2de7f4969efc79ab353e0c19e8",
        ],
        _ => return None,
    };
    Some(images[worker::Date::now().as_millis() as usize % images.len()])
}
