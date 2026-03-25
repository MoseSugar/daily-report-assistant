use crate::desktop::logging::redact_sensitive_text;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("validation error: {message}")]
    Validation { message: String },
    #[error("database error: {message}")]
    Database { message: String },
    #[error("not implemented: {message}")]
    NotImplemented { message: String },
    #[error("internal error: {message}")]
    Internal { message: String },
}

impl AppError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    pub fn database(message: impl Into<String>) -> Self {
        Self::Database {
            message: message.into(),
        }
    }

    pub fn not_implemented(message: impl Into<String>) -> Self {
        Self::NotImplemented {
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&redact_sensitive_text(&self.to_string()))
    }
}
