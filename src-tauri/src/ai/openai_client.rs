use crate::{
    ai::{
        prompt_builder::{build_system_prompt, build_user_prompt},
        response_parser::{extract_report_text, ChatCompletionResponse},
    },
    desktop::logging::redact_sensitive_text,
    domain::{
        app_settings::AppSettings,
        daily_report_draft::{DailyReportDraft, GenerationSource},
        error::AppError,
    },
};
use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone)]
pub struct OpenAiCompatibleClient {
    http_client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiPolishResult {
    pub draft: DailyReportDraft,
    pub warning_message: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatCompletionsRequest {
    model: String,
    messages: Vec<RequestMessage>,
}

#[derive(Debug, Serialize)]
struct RequestMessage {
    role: &'static str,
    content: String,
}

#[derive(Debug, Serialize)]
pub struct AiConnectionCheckResult {
    pub message: String,
}

impl OpenAiCompatibleClient {
    pub fn new() -> Result<Self, AppError> {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(90))
            .build()
            .map_err(|error| AppError::internal(format!("failed to build AI client: {error}")))?;

        Ok(Self { http_client })
    }

    pub fn polish_report(
        &self,
        settings: &AppSettings,
        draft: DailyReportDraft,
    ) -> Result<AiPolishResult, AppError> {
        if !settings.ai_enabled {
            return Ok(AiPolishResult {
                draft,
                warning_message: Some("AI 未启用，已保留基础日报".to_string()),
            });
        }

        settings.validate()?;

        let body = self.send_chat_completions(
            settings,
            build_request_payload(
                settings.ai_model.clone(),
                build_system_prompt(&settings.ai_system_prompt),
                build_user_prompt(&draft),
            ),
        )?;
        let parsed = parse_chat_completion_body(&body)?;
        let polished_text = extract_report_text(parsed)?;

        let draft = DailyReportDraft {
            polished_report_text: Some(polished_text),
            generation_source: GenerationSource::Ai,
            last_generated_at: Some(chrono::Utc::now()),
            ..draft
        };

        Ok(AiPolishResult {
            draft,
            warning_message: None,
        })
    }

    pub fn test_connection(
        &self,
        settings: &AppSettings,
    ) -> Result<AiConnectionCheckResult, AppError> {
        let validation_settings = AppSettings {
            ai_enabled: true,
            ..settings.clone()
        };
        validation_settings.validate()?;

        let body = self.send_chat_completions(
            &validation_settings,
            build_request_payload(
                validation_settings.ai_model.clone(),
                build_system_prompt(&validation_settings.ai_system_prompt),
                "请仅回复“连接成功”，不要添加其他内容。".to_string(),
            ),
        )?;
        let parsed = parse_chat_completion_body(&body)?;
        let text = extract_report_text(parsed)?;

        Ok(AiConnectionCheckResult {
            message: format!("AI 连接可用，响应摘要：{}", summarize_body(&text)),
        })
    }

    fn send_chat_completions(
        &self,
        settings: &AppSettings,
        payload: ChatCompletionsRequest,
    ) -> Result<String, AppError> {
        let endpoint = normalize_chat_completions_endpoint(&settings.ai_base_url)?;
        let response = self
            .http_client
            .post(&endpoint)
            .bearer_auth(&settings.ai_api_key_ref)
            .json(&payload)
            .send()
            .map_err(|error| {
                let sanitized_endpoint = redact_sensitive_text(&endpoint);
                warn!("ai request send failed: endpoint={sanitized_endpoint} error={error}");
                AppError::internal(format!(
                    "AI request failed before receiving a response: {error}"
                ))
            })?;

        let status = response.status();
        let body = response.text().map_err(|error| {
            warn!(
                "ai response body read failed: status={} error={error}",
                status.as_u16()
            );
            AppError::internal(format!("AI response body read failed: {error}"))
        })?;

        if !status.is_success() {
            return Err(build_http_status_error(status, &body));
        }

        Ok(body)
    }
}

fn build_request_payload(
    model: String,
    system_prompt: String,
    user_prompt: String,
) -> ChatCompletionsRequest {
    ChatCompletionsRequest {
        model,
        messages: vec![
            RequestMessage {
                role: "system",
                content: system_prompt,
            },
            RequestMessage {
                role: "user",
                content: user_prompt,
            },
        ],
    }
}

fn normalize_chat_completions_endpoint(base_url: &str) -> Result<String, AppError> {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return Err(AppError::validation(
            "ai_base_url must not be empty when AI is enabled",
        ));
    }

    if trimmed.ends_with("/chat/completions") {
        return Ok(trimmed.to_string());
    }

    Ok(format!("{trimmed}/chat/completions"))
}

fn summarize_error_body(body: &str) -> String {
    let parsed = serde_json::from_str::<serde_json::Value>(body);
    let summary = match parsed {
        Ok(value) => value
            .get("error")
            .and_then(|error| {
                let message = error.get("message").and_then(|item| item.as_str());
                let error_type = error.get("type").and_then(|item| item.as_str());

                match (message, error_type) {
                    (Some(message), Some(error_type)) => Some(format!("{error_type}: {message}")),
                    (Some(message), None) => Some(message.to_string()),
                    _ => None,
                }
            })
            .or_else(|| {
                value
                    .get("message")
                    .and_then(|item| item.as_str())
                    .map(|message| message.to_string())
            })
            .unwrap_or_else(|| summarize_body(body)),
        Err(_) => summarize_body(body),
    };

    redact_sensitive_text(&summary)
}

fn build_http_status_error(status: StatusCode, body: &str) -> AppError {
    let summary = summarize_error_body(body);
    warn!(
        "ai request returned non-success status: status={} body={summary}",
        status.as_u16()
    );
    AppError::internal(format!(
        "AI request returned HTTP {}: {}",
        status.as_u16(),
        summary
    ))
}

fn summarize_body(body: &str) -> String {
    let flattened = body.split_whitespace().collect::<Vec<_>>().join(" ");
    let truncated = if flattened.chars().count() > 220 {
        format!("{}...", flattened.chars().take(220).collect::<String>())
    } else {
        flattened
    };

    redact_sensitive_text(&truncated)
}

fn parse_chat_completion_body(body: &str) -> Result<ChatCompletionResponse, AppError> {
    serde_json::from_str(body).map_err(|error| {
        let summary = summarize_body(body);
        warn!("ai response json parse failed: summary={summary}");
        AppError::internal(format!("AI response parse failed: {error}; body={summary}"))
    })
}

#[cfg(test)]
mod tests {
    use super::{
        build_http_status_error, normalize_chat_completions_endpoint, parse_chat_completion_body,
        summarize_error_body, OpenAiCompatibleClient,
    };
    use crate::{
        ai::prompt_builder::{build_system_prompt, build_user_prompt},
        ai::response_parser::extract_report_text,
        domain::daily_report_draft::{DailyReportDraft, GenerationSource},
    };
    use reqwest::StatusCode;

    #[test]
    fn builds_openai_compatible_client() {
        let client = OpenAiCompatibleClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn builds_prompt_texts_for_openai_request() {
        let draft = DailyReportDraft {
            date: "2026-03-24".to_string(),
            entries: vec![],
            done_entries: vec![],
            in_progress_entries: vec![],
            basic_report_text: "基础日报".to_string(),
            polished_report_text: None,
            generation_source: GenerationSource::Basic,
            last_generated_at: None,
        };
        let system_prompt = build_system_prompt("保持专业克制");
        let user_prompt = build_user_prompt(&draft);

        assert!(system_prompt.contains("保持专业克制"));
        assert!(user_prompt.contains("基础日报"));
    }

    #[test]
    fn normalizes_chat_completion_endpoint() {
        assert_eq!(
            normalize_chat_completions_endpoint("https://example.com/v1/")
                .expect("endpoint should normalize"),
            "https://example.com/v1/chat/completions"
        );
        assert_eq!(
            normalize_chat_completions_endpoint("https://example.com/v1/chat/completions")
                .expect("endpoint should stay stable"),
            "https://example.com/v1/chat/completions"
        );
    }

    #[test]
    fn summarizes_json_error_body() {
        let summary = summarize_error_body(
            r#"{"error":{"message":"model not found","type":"invalid_request_error"}}"#,
        );

        assert!(summary.contains("invalid_request_error"));
        assert!(summary.contains("model not found"));
    }

    #[test]
    fn returns_error_summary_for_http_400() {
        let error = build_http_status_error(
            StatusCode::BAD_REQUEST,
            r#"{"error":{"message":"model does not exist","type":"invalid_request_error"}}"#,
        );

        assert!(error.to_string().contains("HTTP 400"));
        assert!(error.to_string().contains("model does not exist"));
    }

    #[test]
    fn returns_error_summary_for_http_500() {
        let error = build_http_status_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{"message":"temporary upstream failure"}"#,
        );

        assert!(error.to_string().contains("HTTP 500"));
        assert!(error.to_string().contains("temporary upstream failure"));
    }

    #[test]
    fn returns_error_when_json_is_invalid() {
        let error = parse_chat_completion_body("not-json").expect_err("invalid json should fail");

        assert!(error.to_string().contains("parse failed"));
    }

    #[test]
    fn parses_successful_response() {
        let parsed =
            parse_chat_completion_body(r#"{"choices":[{"message":{"content":"连接成功"}}]}"#)
                .expect("successful response should parse");
        let text = extract_report_text(parsed).expect("response text should extract");

        assert_eq!(text, "连接成功");
    }
}
