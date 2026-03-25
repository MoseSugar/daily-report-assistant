use crate::domain::error::AppError;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Database {
    path: PathBuf,
}

impl Database {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn initialize(&self) -> Result<(), AppError> {
        self.open().map(|_| ())
    }

    pub fn open(&self) -> Result<Connection, AppError> {
        Connection::open(&self.path).map_err(|error| AppError::database(error.to_string()))
    }
}
