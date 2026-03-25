use crate::{
    ai::openai_client::{AiConnectionCheckResult, OpenAiCompatibleClient},
    desktop::hotkey_manager::HotkeyRuntime,
    domain::{app_settings::AppSettings, error::AppError, settings_service::SettingsService},
    scheduling::reminder_service::ReminderRuntime,
    storage::settings_repository::SettingsRepository,
    AppState,
};
use tauri::{async_runtime, AppHandle, State};

#[tauri::command]
pub fn settings_get(state: State<'_, AppState>) -> Result<AppSettings, AppError> {
    let repository = SettingsRepository::new(state.database.clone());
    let service = SettingsService::new(repository);
    service.get_settings()
}

#[tauri::command]
pub fn settings_save(
    state: State<'_, AppState>,
    reminder_runtime: State<'_, ReminderRuntime>,
    hotkey_runtime: State<'_, HotkeyRuntime>,
    app_handle: AppHandle,
    settings: AppSettings,
) -> Result<AppSettings, AppError> {
    let repository = SettingsRepository::new(state.database.clone());
    let service = SettingsService::new(repository);
    settings.validate()?;
    hotkey_runtime.register_or_replace(&app_handle, &settings.global_hotkey)?;
    let saved = service.save_settings(settings)?;
    reminder_runtime.reset_for_reschedule();
    Ok(saved)
}

#[tauri::command]
pub async fn settings_test_ai_connection(
    settings: AppSettings,
) -> Result<AiConnectionCheckResult, AppError> {
    async_runtime::spawn_blocking(move || {
        let client = OpenAiCompatibleClient::new()?;
        client.test_connection(&settings)
    })
    .await
    .map_err(|error| AppError::internal(format!("AI connection test join failed: {error}")))?
}
