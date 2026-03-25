use tracing_subscriber::EnvFilter;

pub fn redact_sensitive_text(message: &str) -> String {
    let mut redacted = message.to_string();

    for marker in [
        "Bearer ",
        "bearer ",
        "api_key=",
        "apiKey=",
        "authorization=",
    ] {
        redacted = redact_after_marker(&redacted, marker);
    }

    redact_urls(&redacted)
}

fn redact_after_marker(message: &str, marker: &str) -> String {
    let mut cursor = 0usize;
    let mut output = String::with_capacity(message.len());

    while let Some(relative_index) = message[cursor..].find(marker) {
        let marker_index = cursor + relative_index;
        output.push_str(&message[cursor..marker_index + marker.len()]);

        let value_start = marker_index + marker.len();
        let value_end = message[value_start..]
            .find(|character: char| character.is_whitespace() || matches!(character, '"' | ','))
            .map(|offset| value_start + offset)
            .unwrap_or(message.len());

        output.push_str("[REDACTED]");
        cursor = value_end;
    }

    output.push_str(&message[cursor..]);
    output
}

fn redact_urls(message: &str) -> String {
    let mut cursor = 0usize;
    let mut output = String::with_capacity(message.len());

    while let Some(relative_index) = message[cursor..]
        .find("https://")
        .or_else(|| message[cursor..].find("http://"))
    {
        let url_index = cursor + relative_index;
        output.push_str(&message[cursor..url_index]);

        let url_end = message[url_index..]
            .find(|character: char| {
                character.is_whitespace() || matches!(character, '"' | ',' | ')')
            })
            .map(|offset| url_index + offset)
            .unwrap_or(message.len());

        output.push_str("[REDACTED_URL]");
        cursor = url_end;
    }

    output.push_str(&message[cursor..]);
    output
}

pub fn configure_logging() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::redact_sensitive_text;

    #[test]
    fn redacts_bearer_tokens_and_urls() {
        let raw = "request failed for https://example.com/v1/chat with Authorization: Bearer secret-token";
        let sanitized = redact_sensitive_text(raw);

        assert!(sanitized.contains("[REDACTED_URL]"));
        assert!(sanitized.contains("Bearer [REDACTED]"));
        assert!(!sanitized.contains("secret-token"));
        assert!(!sanitized.contains("example.com/v1/chat"));
    }
}
