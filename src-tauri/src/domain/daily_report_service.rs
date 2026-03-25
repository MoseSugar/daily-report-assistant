use crate::{
    ai::openai_client::{AiPolishResult, OpenAiCompatibleClient},
    desktop::logging::redact_sensitive_text,
    domain::{
        app_settings::AppSettings,
        daily_report_draft::{DailyReportDraft, GenerationSource},
        error::AppError,
        task_entry::{TaskEntry, TaskStatus},
    },
    storage::task_repository::TaskRepository,
};
use chrono::Local;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct DailyReportService {
    repository: TaskRepository,
}

impl DailyReportService {
    pub fn new(repository: TaskRepository) -> Self {
        Self { repository }
    }

    pub fn get_today_draft(&self) -> Result<DailyReportDraft, AppError> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.get_draft_by_date(&today)
    }

    pub fn get_draft_by_date(&self, date: &str) -> Result<DailyReportDraft, AppError> {
        let entries = self.repository.list_by_date(date)?;
        Ok(Self::build_draft(date.to_string(), entries, None))
    }

    pub fn generate_basic_report(&self, date: &str) -> Result<DailyReportDraft, AppError> {
        let entries = self.repository.list_by_date(date)?;
        let report_text = build_basic_report_text(&entries);
        Ok(Self::build_draft(
            date.to_string(),
            entries,
            Some(report_text),
        ))
    }

    pub fn generate_ai_polished_report(
        &self,
        date: &str,
        settings: &AppSettings,
    ) -> Result<AiPolishResult, AppError> {
        let basic_draft = self.generate_basic_report(date)?;
        let client = OpenAiCompatibleClient::new()?;

        match client.polish_report(settings, basic_draft.clone()) {
            Ok(result) => Ok(result),
            Err(error) => {
                let summary = summarize_ai_failure(&error);
                warn!("ai polish fallback activated: {summary}");
                Ok(AiPolishResult {
                    draft: basic_draft,
                    warning_message: Some(format!(
                        "AI 润色失败，已保留基础日报内容。原因：{summary}"
                    )),
                })
            }
        }
    }

    fn build_draft(
        date: String,
        entries: Vec<TaskEntry>,
        report_text: Option<String>,
    ) -> DailyReportDraft {
        let done_entries = entries
            .iter()
            .filter(|entry| entry.status == TaskStatus::Done)
            .cloned()
            .collect::<Vec<_>>();
        let in_progress_entries = entries
            .iter()
            .filter(|entry| entry.status == TaskStatus::InProgress)
            .cloned()
            .collect::<Vec<_>>();

        DailyReportDraft {
            date,
            entries,
            done_entries,
            in_progress_entries,
            basic_report_text: report_text.unwrap_or_default(),
            polished_report_text: None,
            generation_source: GenerationSource::Basic,
            last_generated_at: Some(chrono::Utc::now()),
        }
    }
}

fn summarize_ai_failure(error: &AppError) -> String {
    let sanitized = redact_sensitive_text(&error.to_string());
    let trimmed = sanitized
        .trim_start_matches("internal error: ")
        .trim_start_matches("validation error: ")
        .trim()
        .to_string();

    if trimmed.chars().count() > 220 {
        format!("{}...", trimmed.chars().take(220).collect::<String>())
    } else {
        trimmed
    }
}

pub fn build_basic_report_text(entries: &[TaskEntry]) -> String {
    let done_entries = entries
        .iter()
        .filter(|entry| entry.status == TaskStatus::Done)
        .collect::<Vec<_>>();
    let in_progress_entries = entries
        .iter()
        .filter(|entry| entry.status == TaskStatus::InProgress)
        .collect::<Vec<_>>();
    let note_entries = entries
        .iter()
        .filter(|entry| !entry.note.trim().is_empty())
        .collect::<Vec<_>>();

    let mut sections = vec!["今日工作日报：".to_string()];

    sections.push(String::new());
    sections.push("已完成：".to_string());
    if done_entries.is_empty() {
        sections.push("1. 今日暂无已完成事项。".to_string());
    } else {
        sections.extend(
            done_entries
                .iter()
                .enumerate()
                .map(|(index, entry)| format!("{}. {}", index + 1, entry.content)),
        );
    }

    sections.push(String::new());
    sections.push("进行中：".to_string());
    if in_progress_entries.is_empty() {
        sections.push("1. 今日暂无进行中事项。".to_string());
    } else {
        sections.extend(
            in_progress_entries
                .iter()
                .enumerate()
                .map(|(index, entry)| format!("{}. {}", index + 1, entry.content)),
        );
    }

    if !note_entries.is_empty() {
        sections.push(String::new());
        sections.push("补充说明：".to_string());
        sections.extend(note_entries.iter().enumerate().map(|(index, entry)| {
            format!("{}. {}：{}", index + 1, entry.content, entry.note.trim())
        }));
    }

    sections.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{build_basic_report_text, DailyReportService};
    use crate::{
        domain::{app_settings::AppSettings, task_entry::TaskEntry, task_service::TaskService},
        storage::{
            database::Database, migrations::run_migrations, task_repository::TaskRepository,
        },
    };

    #[test]
    fn generates_basic_report_text_from_persisted_tasks() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join(format!("dra-draft-{}.sqlite3", uuid::Uuid::new_v4()));
        let database = Database::new(&db_path);
        database.initialize().expect("database should initialize");
        run_migrations(&database).expect("migrations should run");

        let repository = TaskRepository::new(database.clone());
        let task_service = TaskService::new(repository.clone());
        let draft_service = DailyReportService::new(repository);
        let _ = task_service
            .create_task("完成日报列表")
            .expect("task created");

        let draft = draft_service.get_today_draft().expect("draft should build");

        assert_eq!(draft.entries.len(), 1);
        assert_eq!(draft.done_entries.len(), 1);

        std::fs::remove_file(db_path).expect("temp database should be removable");
    }

    #[test]
    fn formats_report_sections() {
        let mut task = TaskEntry::new("完成日报列表").expect("task created");
        task.note = "已完成基本联调".to_string();

        let text = build_basic_report_text(&[task]);

        assert!(text.contains("已完成："));
        assert!(text.contains("补充说明："));
    }

    #[test]
    fn falls_back_to_basic_report_when_ai_is_disabled() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join(format!("dra-ai-fallback-{}.sqlite3", uuid::Uuid::new_v4()));
        let database = Database::new(&db_path);
        database.initialize().expect("database should initialize");
        run_migrations(&database).expect("migrations should run");

        let repository = TaskRepository::new(database.clone());
        let task_service = TaskService::new(repository.clone());
        let draft_service = DailyReportService::new(repository);
        let _ = task_service
            .create_task("准备基础日报")
            .expect("task created");

        let settings = AppSettings::default();
        let result = draft_service
            .generate_ai_polished_report(
                &chrono::Local::now().format("%Y-%m-%d").to_string(),
                &settings,
            )
            .expect("fallback should succeed");

        assert_eq!(
            result.draft.generation_source,
            crate::domain::daily_report_draft::GenerationSource::Basic
        );
        assert!(result.warning_message.is_some());

        std::fs::remove_file(db_path).expect("temp database should be removable");
    }
}
