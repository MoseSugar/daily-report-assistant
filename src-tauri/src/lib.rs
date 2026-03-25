pub mod ai;
pub mod commands;
pub mod desktop;
pub mod domain;
pub mod scheduling;
pub mod storage;

use commands::{capture, settings, summary, system};
use desktop::{hotkey_manager::HotkeyRuntime, logging::configure_logging, tray::setup_tray};
use domain::settings_service::SettingsService;
use scheduling::reminder_service::{start_reminder_loop, ReminderRuntime};
use std::{fs, path::PathBuf};
use storage::{
    database::Database, migrations::run_migrations, settings_repository::SettingsRepository,
};
use tauri::Manager;

pub struct AppState {
    pub database: Database,
}

fn ensure_app_data_dir(app: &tauri::App) -> Result<PathBuf, String> {
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("failed to resolve app data dir: {error}"))?;

    fs::create_dir_all(&app_dir)
        .map_err(|error| format!("failed to create app data dir: {error}"))?;
    Ok(app_dir)
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            configure_logging();

            let app_dir = ensure_app_data_dir(app)?;
            let db_path = app_dir.join("daily-report-assistant.sqlite3");
            let database = Database::new(db_path);
            database
                .initialize()
                .map_err(|error| format!("failed to initialize database: {error}"))?;
            run_migrations(&database)
                .map_err(|error| format!("failed to run migrations: {error}"))?;

            let reminder_runtime = ReminderRuntime::default();
            let hotkey_runtime = HotkeyRuntime::default();
            let settings_repository = SettingsRepository::new(database.clone());
            let settings_service = SettingsService::new(settings_repository);

            if let Ok(settings) = settings_service.get_settings() {
                let _ = hotkey_runtime
                    .register_or_replace(&app.handle().clone(), &settings.global_hotkey);
            }

            start_reminder_loop(
                app.handle().clone(),
                reminder_runtime.clone(),
                database.clone(),
            );
            let _ = setup_tray(app);

            app.manage(AppState { database });
            app.manage(reminder_runtime);
            app.manage(hotkey_runtime);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            capture::capture_create_task,
            capture::capture_list_today_tasks,
            summary::summary_get_today_draft,
            summary::summary_update_task,
            summary::summary_generate_basic_report,
            summary::summary_generate_ai_polished_report,
            settings::settings_get,
            settings::settings_save,
            settings::settings_test_ai_connection,
            system::system_ping,
            system::system_show_capture_window,
            system::system_show_summary_window,
            system::system_trigger_summary_reminder,
            system::system_copy_text,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
