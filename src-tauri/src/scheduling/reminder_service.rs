use crate::{
    desktop::window_manager::show_summary_window,
    domain::{app_settings::AppSettings, error::AppError, settings_service::SettingsService},
    storage::{database::Database, settings_repository::SettingsRepository},
};
use chrono::{Local, NaiveTime};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

#[derive(Clone, Default)]
pub struct ReminderRuntime {
    inner: Arc<Mutex<ReminderState>>,
}

#[derive(Default)]
struct ReminderState {
    last_triggered_date: Option<String>,
}

impl ReminderRuntime {
    pub fn should_trigger(&self, settings: &AppSettings) -> bool {
        if !settings.reminder_enabled {
            return false;
        }

        let Ok(off_work_time) = NaiveTime::parse_from_str(&settings.off_work_time, "%H:%M") else {
            return false;
        };

        let today = Local::now().format("%Y-%m-%d").to_string();
        let now = Local::now().naive_local();
        let reminder_time =
            off_work_time - chrono::TimeDelta::minutes(settings.remind_before_minutes);
        let scheduled_at = now.date().and_time(reminder_time);
        let mut state = self.inner.lock().expect("reminder mutex poisoned");

        if state.last_triggered_date.as_deref() == Some(today.as_str()) {
            return false;
        }

        if now >= scheduled_at {
            state.last_triggered_date = Some(today);
            return true;
        }

        false
    }

    pub fn mark_triggered_for_today(&self) {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let mut state = self.inner.lock().expect("reminder mutex poisoned");
        state.last_triggered_date = Some(today);
    }

    pub fn reset_for_reschedule(&self) {
        let mut state = self.inner.lock().expect("reminder mutex poisoned");
        state.last_triggered_date = None;
    }
}

pub fn start_reminder_loop(app_handle: AppHandle, runtime: ReminderRuntime, database: Database) {
    std::thread::spawn(move || loop {
        let repository = SettingsRepository::new(database.clone());
        let service = SettingsService::new(repository);

        if let Ok(settings) = service.get_settings() {
            if runtime.should_trigger(&settings) {
                let _ = trigger_summary_reminder(&app_handle);
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(15));
    });
}

pub fn trigger_summary_reminder(app_handle: &AppHandle) -> Result<(), AppError> {
    show_summary_window(app_handle)?;
    app_handle
        .emit("window.summary.reminder_triggered", "summary")
        .map_err(|error| AppError::internal(error.to_string()))
}
