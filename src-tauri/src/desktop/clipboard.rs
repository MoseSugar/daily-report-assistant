use crate::domain::error::AppError;

pub fn copy_text(text: &str) -> Result<(), AppError> {
    if text.trim().is_empty() {
        return Err(AppError::validation("copy text must not be empty"));
    }

    let mut clipboard =
        arboard::Clipboard::new().map_err(|error| AppError::internal(error.to_string()))?;

    clipboard
        .set_text(text.to_string())
        .map_err(|error| AppError::internal(format!("clipboard unavailable: {error}")))
}
