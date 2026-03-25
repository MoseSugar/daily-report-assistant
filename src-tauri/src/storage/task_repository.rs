use crate::{
    domain::{
        error::AppError,
        task_entry::{TaskEntry, TaskStatus},
    },
    storage::database::Database,
};
use chrono::{DateTime, Utc};
use rusqlite::params;

#[derive(Debug, Clone)]
pub struct TaskRepository {
    database: Database,
}

impl TaskRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub fn database_path(&self) -> &std::path::Path {
        self.database.path()
    }

    pub fn insert(&self, task: &TaskEntry) -> Result<(), AppError> {
        let connection = self.database.open()?;
        connection
            .execute(
                "INSERT INTO task_entries (id, date, created_at, updated_at, content, status, note)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    task.id,
                    task.date,
                    task.created_at.to_rfc3339(),
                    task.updated_at.to_rfc3339(),
                    task.content,
                    task.status_as_str(),
                    task.note
                ],
            )
            .map_err(|error| AppError::database(error.to_string()))?;

        Ok(())
    }

    pub fn list_by_date(&self, date: &str) -> Result<Vec<TaskEntry>, AppError> {
        let connection = self.database.open()?;
        let mut statement = connection
            .prepare(
                "SELECT id, date, created_at, updated_at, content, status, note
                 FROM task_entries
                 WHERE date = ?1
                 ORDER BY created_at ASC",
            )
            .map_err(|error| AppError::database(error.to_string()))?;

        let rows = statement
            .query_map([date], |row| {
                let created_at: String = row.get(2)?;
                let updated_at: String = row.get(3)?;
                let status: String = row.get(5)?;

                let created_at = DateTime::parse_from_rfc3339(&created_at)
                    .map(|value| value.with_timezone(&Utc))
                    .map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?;

                let updated_at = DateTime::parse_from_rfc3339(&updated_at)
                    .map(|value| value.with_timezone(&Utc))
                    .map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            3,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?;

                let status = TaskStatus::from_db(&status).map_err(|error| {
                    rusqlite::Error::FromSqlConversionFailure(
                        5,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            error.to_string(),
                        )),
                    )
                })?;

                Ok(TaskEntry {
                    id: row.get(0)?,
                    date: row.get(1)?,
                    created_at,
                    updated_at,
                    content: row.get(4)?,
                    status,
                    note: row.get(6)?,
                })
            })
            .map_err(|error| AppError::database(error.to_string()))?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row.map_err(|error| AppError::database(error.to_string()))?);
        }

        Ok(tasks)
    }

    pub fn update_task(
        &self,
        task_id: &str,
        status: TaskStatus,
        note: String,
    ) -> Result<TaskEntry, AppError> {
        let connection = self.database.open()?;
        let updated_at = Utc::now().to_rfc3339();
        connection
            .execute(
                "UPDATE task_entries
                 SET status = ?1, note = ?2, updated_at = ?3
                 WHERE id = ?4",
                params![status_str(&status), note, updated_at, task_id],
            )
            .map_err(|error| AppError::database(error.to_string()))?;

        if connection.changes() == 0 {
            return Err(AppError::validation("task not found"));
        }

        let mut statement = connection
            .prepare(
                "SELECT id, date, created_at, updated_at, content, status, note
                 FROM task_entries
                 WHERE id = ?1",
            )
            .map_err(|error| AppError::database(error.to_string()))?;

        let task = statement
            .query_row([task_id], |row| {
                let created_at: String = row.get(2)?;
                let updated_at: String = row.get(3)?;
                let status: String = row.get(5)?;

                let created_at = DateTime::parse_from_rfc3339(&created_at)
                    .map(|value| value.with_timezone(&Utc))
                    .map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            2,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?;

                let updated_at = DateTime::parse_from_rfc3339(&updated_at)
                    .map(|value| value.with_timezone(&Utc))
                    .map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(
                            3,
                            rusqlite::types::Type::Text,
                            Box::new(error),
                        )
                    })?;

                let status = TaskStatus::from_db(&status).map_err(|error| {
                    rusqlite::Error::FromSqlConversionFailure(
                        5,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            error.to_string(),
                        )),
                    )
                })?;

                Ok(TaskEntry {
                    id: row.get(0)?,
                    date: row.get(1)?,
                    created_at,
                    updated_at,
                    content: row.get(4)?,
                    status,
                    note: row.get(6)?,
                })
            })
            .map_err(|error| AppError::database(error.to_string()))?;

        Ok(task)
    }
}

fn status_str(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::Done => "done",
        TaskStatus::InProgress => "in_progress",
    }
}
