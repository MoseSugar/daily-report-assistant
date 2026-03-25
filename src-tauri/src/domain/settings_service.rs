use crate::{
    domain::{app_settings::AppSettings, error::AppError},
    storage::settings_repository::SettingsRepository,
};

#[derive(Debug, Clone)]
pub struct SettingsService {
    repository: SettingsRepository,
}

impl SettingsService {
    pub fn new(repository: SettingsRepository) -> Self {
        Self { repository }
    }

    pub fn get_settings(&self) -> Result<AppSettings, AppError> {
        self.repository.get()
    }

    pub fn save_settings(&self, settings: AppSettings) -> Result<AppSettings, AppError> {
        settings.validate()?;
        self.repository.save(&settings)
    }
}
