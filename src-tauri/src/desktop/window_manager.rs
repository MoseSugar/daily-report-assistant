use crate::domain::error::AppError;
use tauri::{AppHandle, Emitter, Manager};

fn show_main_window(app_handle: &AppHandle) -> Result<tauri::WebviewWindow, AppError> {
    let window = app_handle
        .get_webview_window("main")
        .ok_or_else(|| AppError::internal("main window not found"))?;

    window
        .show()
        .map_err(|error| AppError::internal(error.to_string()))?;
    window
        .set_focus()
        .map_err(|error| AppError::internal(error.to_string()))?;

    Ok(window)
}

pub fn show_capture_window(app_handle: &AppHandle) -> Result<(), AppError> {
    let _window = show_main_window(app_handle)?;
    app_handle
        .emit("window.capture.hotkey_triggered", "capture")
        .map_err(|error| AppError::internal(error.to_string()))
}

pub fn show_summary_window(app_handle: &AppHandle) -> Result<(), AppError> {
    let _window = show_main_window(app_handle)?;
    app_handle
        .emit("window.summary.open_requested", "summary")
        .map_err(|error| AppError::internal(error.to_string()))
}
