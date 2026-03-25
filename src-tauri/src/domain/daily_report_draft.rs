use crate::domain::task_entry::TaskEntry;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GenerationSource {
    Basic,
    Ai,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyReportDraft {
    pub date: String,
    pub entries: Vec<TaskEntry>,
    pub done_entries: Vec<TaskEntry>,
    pub in_progress_entries: Vec<TaskEntry>,
    pub basic_report_text: String,
    pub polished_report_text: Option<String>,
    pub generation_source: GenerationSource,
    pub last_generated_at: Option<DateTime<Utc>>,
}

impl DailyReportDraft {
    pub fn empty(date: impl Into<String>) -> Self {
        Self {
            date: date.into(),
            entries: Vec::new(),
            done_entries: Vec::new(),
            in_progress_entries: Vec::new(),
            basic_report_text: String::new(),
            polished_report_text: None,
            generation_source: GenerationSource::Basic,
            last_generated_at: None,
        }
    }
}
