use crate::{
    desktop::{
        clipboard::copy_text,
        window_manager::{show_capture_window, show_summary_window},
    },
    domain::error::AppError,
    scheduling::reminder_service::{trigger_summary_reminder, ReminderRuntime},
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

#[derive(Debug, Serialize)]
pub struct PingResponse {
    pub message: &'static str,
}

#[derive(Debug, Deserialize)]
pub struct CopyTextPayload {
    pub text: String,
}

#[tauri::command]
pub fn system_ping() -> Result<PingResponse, AppError> {
    Ok(PingResponse {
        message: "desktop command layer ready",
    })
}

#[tauri::command]
pub fn system_show_capture_window(app_handle: AppHandle) -> Result<PingResponse, AppError> {
    show_capture_window(&app_handle)?;

    Ok(PingResponse {
        message: "capture window requested",
    })
}

#[tauri::command]
pub fn system_show_summary_window(app_handle: AppHandle) -> Result<PingResponse, AppError> {
    show_summary_window(&app_handle)?;

    Ok(PingResponse {
        message: "summary window requested",
    })
}

#[tauri::command]
pub fn system_trigger_summary_reminder(
    app_handle: AppHandle,
    reminder_runtime: State<'_, ReminderRuntime>,
) -> Result<PingResponse, AppError> {
    reminder_runtime.mark_triggered_for_today();
    trigger_summary_reminder(&app_handle)?;

    Ok(PingResponse {
        message: "summary reminder triggered",
    })
}

#[tauri::command]
pub fn system_copy_text(payload: CopyTextPayload) -> Result<PingResponse, AppError> {
    copy_text(&payload.text)?;

    Ok(PingResponse {
        message: "text copied to clipboard",
    })
}
