use crate::{
    domain::{error::AppError, task_entry::TaskEntry, task_service::TaskService},
    storage::task_repository::TaskRepository,
    AppState,
};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct CreateTaskPayload {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTaskResponse {
    pub task: TaskEntry,
    pub message: String,
}

#[tauri::command]
pub fn capture_create_task(
    state: State<'_, AppState>,
    payload: CreateTaskPayload,
) -> Result<CreateTaskResponse, AppError> {
    let repository = TaskRepository::new(state.database.clone());
    let service = TaskService::new(repository);
    let task = service.create_task(payload.content)?;

    Ok(CreateTaskResponse {
        task,
        message: "保存成功".to_string(),
    })
}

#[tauri::command]
pub fn capture_list_today_tasks(state: State<'_, AppState>) -> Result<Vec<TaskEntry>, AppError> {
    let repository = TaskRepository::new(state.database.clone());
    let service = TaskService::new(repository);
    service.list_today_tasks()
}
