use crate::{
    domain::{app_settings::AppSettings, error::AppError},
    storage::database::Database,
};
use chrono::{DateTime, Utc};
use rusqlite::params;

#[derive(Debug, Clone)]
pub struct SettingsRepository {
    database: Database,
}

impl SettingsRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub fn database_path(&self) -> &std::path::Path {
        self.database.path()
    }

    pub fn get(&self) -> Result<AppSettings, AppError> {
        let connection = self.database.open()?;
        let mut statement = connection
            .prepare(
                "SELECT reminder_enabled, off_work_time, remind_before_minutes, global_hotkey,
                        ai_enabled, ai_base_url, ai_model, ai_api_key_ref, ai_system_prompt, report_template, updated_at
                 FROM app_settings
                 WHERE singleton_key = 'default'",
            )
            .map_err(|error| AppError::database(error.to_string()))?;

        match statement.query_row([], |row| {
            let updated_at: String = row.get(10)?;
            let updated_at = DateTime::parse_from_rfc3339(&updated_at)
                .map(|value| value.with_timezone(&Utc))
                .map_err(|error| {
                    rusqlite::Error::FromSqlConversionFailure(
                        10,
                        rusqlite::types::Type::Text,
                        Box::new(error),
                    )
                })?;

            Ok(AppSettings {
                reminder_enabled: row.get::<_, i64>(0)? != 0,
                off_work_time: row.get(1)?,
                remind_before_minutes: row.get(2)?,
                global_hotkey: row.get(3)?,
                ai_enabled: row.get::<_, i64>(4)? != 0,
                ai_base_url: row.get(5)?,
                ai_model: row.get(6)?,
                ai_api_key_ref: row.get(7)?,
                ai_system_prompt: row.get(8)?,
                report_template: row.get(9)?,
                updated_at,
            })
        }) {
            Ok(settings) => Ok(settings),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                let default_settings = AppSettings::default();
                let saved = self.save(&default_settings)?;
                Ok(saved)
            }
            Err(error) => Err(AppError::database(error.to_string())),
        }
    }

    pub fn save(&self, settings: &AppSettings) -> Result<AppSettings, AppError> {
        let connection = self.database.open()?;
        let updated_settings = AppSettings {
            updated_at: Utc::now(),
            ..settings.clone()
        };

        connection
            .execute(
                "INSERT INTO app_settings (
                    singleton_key, reminder_enabled, off_work_time, remind_before_minutes, global_hotkey,
                    ai_enabled, ai_base_url, ai_model, ai_api_key_ref, ai_system_prompt, report_template, updated_at
                ) VALUES ('default', ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                ON CONFLICT(singleton_key) DO UPDATE SET
                    reminder_enabled = excluded.reminder_enabled,
                    off_work_time = excluded.off_work_time,
                    remind_before_minutes = excluded.remind_before_minutes,
                    global_hotkey = excluded.global_hotkey,
                    ai_enabled = excluded.ai_enabled,
                    ai_base_url = excluded.ai_base_url,
                    ai_model = excluded.ai_model,
                    ai_api_key_ref = excluded.ai_api_key_ref,
                    ai_system_prompt = excluded.ai_system_prompt,
                    report_template = excluded.report_template,
                    updated_at = excluded.updated_at",
                params![
                    if updated_settings.reminder_enabled { 1 } else { 0 },
                    updated_settings.off_work_time,
                    updated_settings.remind_before_minutes,
                    updated_settings.global_hotkey,
                    if updated_settings.ai_enabled { 1 } else { 0 },
                    updated_settings.ai_base_url,
                    updated_settings.ai_model,
                    updated_settings.ai_api_key_ref,
                    updated_settings.ai_system_prompt,
                    updated_settings.report_template,
                    updated_settings.updated_at.to_rfc3339(),
                ],
            )
            .map_err(|error| AppError::database(error.to_string()))?;

        Ok(updated_settings)
    }
}
