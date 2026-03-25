use crate::{
    ai::openai_client::AiPolishResult,
    domain::{
        daily_report_draft::DailyReportDraft,
        daily_report_service::DailyReportService,
        error::AppError,
        task_entry::{TaskEntry, TaskStatus},
        task_service::TaskService,
    },
    storage::{settings_repository::SettingsRepository, task_repository::TaskRepository},
    AppState,
};
use serde::Deserialize;
use tauri::{async_runtime, State};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskPayload {
    pub task_id: String,
    pub status: TaskStatus,
    pub note: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateReportPayload {
    pub date: String,
}

#[tauri::command]
pub async fn summary_generate_ai_polished_report(
    state: State<'_, AppState>,
    payload: GenerateReportPayload,
) -> Result<AiPolishResult, AppError> {
    if payload.date.trim().is_empty() {
        return Err(AppError::validation("date must not be empty"));
    }

    let database = state.database.clone();
    let date = payload.date;

    async_runtime::spawn_blocking(move || {
        let task_repository = TaskRepository::new(database.clone());
        let settings_repository = SettingsRepository::new(database);
        let service = DailyReportService::new(task_repository);
        let settings = settings_repository.get()?;

        service.generate_ai_polished_report(&date, &settings)
    })
    .await
    .map_err(|error| AppError::internal(format!("AI polish task join failed: {error}")))?
}

#[tauri::command]
pub fn summary_get_today_draft(state: State<'_, AppState>) -> Result<DailyReportDraft, AppError> {
    let repository = TaskRepository::new(state.database.clone());
    let service = DailyReportService::new(repository);
    service.get_today_draft()
}

#[tauri::command]
pub fn summary_update_task(
    state: State<'_, AppState>,
    payload: UpdateTaskPayload,
) -> Result<TaskEntry, AppError> {
    if payload.task_id.trim().is_empty() {
        return Err(AppError::validation("task_id must not be empty"));
    }

    let repository = TaskRepository::new(state.database.clone());
    let task_service = TaskService::new(repository);
    task_service.update_task(payload.task_id, payload.status, payload.note)
}

#[tauri::command]
pub fn summary_generate_basic_report(
    state: State<'_, AppState>,
    payload: GenerateReportPayload,
) -> Result<DailyReportDraft, AppError> {
    if payload.date.trim().is_empty() {
        return Err(AppError::validation("date must not be empty"));
    }

    let repository = TaskRepository::new(state.database.clone());
    let service = DailyReportService::new(repository);
    service.generate_basic_report(&payload.date)
}
