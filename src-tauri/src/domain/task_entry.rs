use crate::domain::error::AppError;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Done,
    InProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEntry {
    pub id: String,
    pub date: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub content: String,
    pub status: TaskStatus,
    pub note: String,
}

impl TaskEntry {
    pub fn empty() -> Self {
        let now = Utc::now();

        Self {
            id: String::new(),
            date: String::new(),
            created_at: now,
            updated_at: now,
            content: String::new(),
            status: TaskStatus::Done,
            note: String::new(),
        }
    }

    pub fn new(content: impl Into<String>) -> Result<Self, AppError> {
        let trimmed = content.into().trim().to_string();

        if trimmed.is_empty() {
            return Err(AppError::validation("task content must not be empty"));
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            date: Local::now().format("%Y-%m-%d").to_string(),
            created_at: now,
            updated_at: now,
            content: trimmed,
            status: TaskStatus::Done,
            note: String::new(),
        })
    }

    pub fn status_as_str(&self) -> &'static str {
        match self.status {
            TaskStatus::Done => "done",
            TaskStatus::InProgress => "in_progress",
        }
    }
}

impl TaskStatus {
    pub fn from_db(value: &str) -> Result<Self, AppError> {
        match value {
            "done" => Ok(Self::Done),
            "in_progress" => Ok(Self::InProgress),
            other => Err(AppError::database(format!(
                "unsupported task status stored in database: {other}"
            ))),
        }
    }
}
