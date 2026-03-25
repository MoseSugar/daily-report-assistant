use crate::domain::error::AppError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    #[serde(default)]
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    #[serde(default)]
    pub message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(default)]
    pub content: Option<ContentValue>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ContentValue {
    Text(String),
    Parts(Vec<ContentPart>),
}

#[derive(Debug, Deserialize)]
pub struct ContentPart {
    pub text: Option<String>,
}

pub fn extract_report_text(response: ChatCompletionResponse) -> Result<String, AppError> {
    let Some(choice) = response.choices.into_iter().next() else {
        return Err(AppError::internal("AI response choices are empty"));
    };

    let Some(message) = choice.message else {
        return Err(AppError::internal(
            "AI response first choice message is missing",
        ));
    };

    let Some(content_value) = message.content else {
        return Err(AppError::internal("AI response message content is missing"));
    };

    let content = match content_value {
        ContentValue::Text(text) => text,
        ContentValue::Parts(parts) => parts
            .into_iter()
            .filter_map(|part| part.text)
            .collect::<Vec<_>>()
            .join(""),
    };

    let trimmed = content.trim().to_string();
    if trimmed.is_empty() {
        return Err(AppError::internal("AI response content is empty"));
    }

    Ok(trimmed)
}

#[cfg(test)]
mod tests {
    use super::{
        extract_report_text, ChatCompletionResponse, Choice, ContentPart, ContentValue, Message,
    };

    #[test]
    fn extracts_plain_text_content() {
        let response = ChatCompletionResponse {
            choices: vec![Choice {
                message: Some(Message {
                    content: Some(ContentValue::Text("润色后的日报".to_string())),
                }),
            }],
        };

        let text = extract_report_text(response).expect("text should parse");
        assert_eq!(text, "润色后的日报");
    }

    #[test]
    fn extracts_parts_content() {
        let response = ChatCompletionResponse {
            choices: vec![Choice {
                message: Some(Message {
                    content: Some(ContentValue::Parts(vec![
                        ContentPart {
                            text: Some("第一段".to_string()),
                        },
                        ContentPart {
                            text: Some("第二段".to_string()),
                        },
                    ])),
                }),
            }],
        };

        let text = extract_report_text(response).expect("text should parse");
        assert_eq!(text, "第一段第二段");
    }

    #[test]
    fn rejects_empty_choices() {
        let error = extract_report_text(ChatCompletionResponse { choices: vec![] })
            .expect_err("empty choices should fail");

        assert!(error.to_string().contains("choices are empty"));
    }

    #[test]
    fn rejects_missing_message_content() {
        let error = extract_report_text(ChatCompletionResponse {
            choices: vec![Choice {
                message: Some(Message { content: None }),
            }],
        })
        .expect_err("missing content should fail");

        assert!(error.to_string().contains("content is missing"));
    }
}
