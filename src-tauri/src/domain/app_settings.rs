use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub reminder_enabled: bool,
    pub off_work_time: String,
    pub remind_before_minutes: i64,
    pub global_hotkey: String,
    pub ai_enabled: bool,
    pub ai_base_url: String,
    pub ai_model: String,
    pub ai_api_key_ref: String,
    pub ai_system_prompt: String,
    pub report_template: String,
    pub updated_at: DateTime<Utc>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            reminder_enabled: true,
            off_work_time: "18:00".to_string(),
            remind_before_minutes: 5,
            global_hotkey: "Ctrl+Shift+D".to_string(),
            ai_enabled: false,
            ai_base_url: String::new(),
            ai_model: String::new(),
            ai_api_key_ref: String::new(),
            ai_system_prompt: String::new(),
            report_template: "default".to_string(),
            updated_at: Utc::now(),
        }
    }
}

impl AppSettings {
    pub fn validate(&self) -> Result<(), crate::domain::error::AppError> {
        if self.off_work_time.trim().is_empty() {
            return Err(crate::domain::error::AppError::validation(
                "off_work_time must not be empty",
            ));
        }

        if self.remind_before_minutes < 0 {
            return Err(crate::domain::error::AppError::validation(
                "remind_before_minutes must not be negative",
            ));
        }

        if self.global_hotkey.trim().is_empty() {
            return Err(crate::domain::error::AppError::validation(
                "global_hotkey must not be empty",
            ));
        }

        if self.ai_enabled {
            if self.ai_base_url.trim().is_empty() {
                return Err(crate::domain::error::AppError::validation(
                    "ai_base_url must not be empty when AI is enabled",
                ));
            }
            if self.ai_model.trim().is_empty() {
                return Err(crate::domain::error::AppError::validation(
                    "ai_model must not be empty when AI is enabled",
                ));
            }
            if self.ai_api_key_ref.trim().is_empty() {
                return Err(crate::domain::error::AppError::validation(
                    "ai_api_key_ref must not be empty when AI is enabled",
                ));
            }
        }

        Ok(())
    }
}
