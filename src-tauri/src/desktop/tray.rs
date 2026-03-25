use crate::domain::error::AppError;
use tauri::App;

#[cfg(target_os = "windows")]
use crate::desktop::window_manager::{show_capture_window, show_summary_window};
#[cfg(target_os = "windows")]
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

#[cfg(target_os = "windows")]
pub fn setup_tray(app: &App) -> Result<(), AppError> {
    let open_capture =
        MenuItem::with_id(app, "tray_open_capture", "打开快速记录", true, None::<&str>)
            .map_err(|error| AppError::internal(error.to_string()))?;
    let open_summary =
        MenuItem::with_id(app, "tray_open_summary", "打开当日汇总", true, None::<&str>)
            .map_err(|error| AppError::internal(error.to_string()))?;
    let separator = PredefinedMenuItem::separator(app)
        .map_err(|error| AppError::internal(error.to_string()))?;
    let quit = PredefinedMenuItem::quit(app, Some("退出"))
        .map_err(|error| AppError::internal(error.to_string()))?;

    let menu = Menu::with_items(app, &[&open_capture, &open_summary, &separator, &quit])
        .map_err(|error| AppError::internal(error.to_string()))?;
    let icon = app
        .default_window_icon()
        .cloned()
        .ok_or_else(|| AppError::internal("default tray icon is unavailable"))?;

    TrayIconBuilder::with_id("daily-report-assistant-tray")
        .icon(icon)
        .menu(&menu)
        .tooltip("日报助手")
        .show_menu_on_left_click(false)
        .on_menu_event(|app_handle, event| match event.id().as_ref() {
            "tray_open_capture" => {
                let _ = show_capture_window(app_handle);
            }
            "tray_open_summary" => {
                let _ = show_summary_window(app_handle);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let _ = show_capture_window(tray.app_handle());
            }
        })
        .build(app)
        .map_err(|error| AppError::internal(error.to_string()))?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn setup_tray(_app: &App) -> Result<(), AppError> {
    Ok(())
}
